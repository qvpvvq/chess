import asyncio
import json
import logging
import os
import random
import re
import signal

from dotenv import load_dotenv

load_dotenv()  # до чтения os.environ

import chess
import nats
from nats.errors import TimeoutError as NatsTimeout
from openai import AsyncOpenAI

NATS_URL = os.getenv("NATS_URL", "nats://localhost:4222")
STREAM = "CHESS"
DURABLE = "ai_worker"                       # должен совпадать с консьюмером в Nest
REQUEST_FILTER = "chess.ai.request.>"
MODEL = os.environ["LLM_MODEL"]
MAX_ATTEMPTS = 3
FALLBACK_RANDOM = os.getenv("FALLBACK_RANDOM") == "1"  # отладка: случайный ход вместо ошибки

log = logging.getLogger("ai-worker")

llm = AsyncOpenAI(
    api_key=os.environ["LLM_API_KEY"],      # нет ключа: падаем сразу при старте
    base_url=os.getenv("LLM_BASE_URL") or None,
)

SYSTEM = (
    "Ты шахматный игрок. Тебе дают позицию и список легальных ходов в формате UCI. "
    "Выбери один ход из списка и ответь ТОЛЬКО им, без пояснений. Пример: e7e5"
)
UCI_RE = re.compile(r"\b([a-h][1-8][a-h][1-8][qrbn]?)\b")


# ---------- шахматная часть ----------

def build_board(fen: str, history: list[str]) -> tuple[chess.Board, list[str]]:
    """Позиция из FEN и история в SAN (для LLM читается лучше, чем UCI)."""
    board = chess.Board(fen)
    replay, san = chess.Board(), []
    try:
        for uci in history:
            move = chess.Move.from_uci(uci)
            san.append(replay.san(move))
            replay.push(move)
        if replay.board_fen() != board.board_fen():
            san = []                         # история не сходится с позицией: не показываем
    except ValueError:
        san = []
    return board, san


def build_prompt(board: chess.Board, san: list[str], legal: list[str], feedback: str = "") -> str:
    side = "белые" if board.turn == chess.WHITE else "чёрные"
    parts = [
        f"Ты играешь за {side}.",
        f"Позиция (FEN): {board.fen()}",
        f"Партия до этого момента: {' '.join(san)}" if san else "",
        f"Легальные ходы: {' '.join(legal)}",
        feedback,
        "Твой ход (UCI):",
    ]
    return "\n".join(p for p in parts if p)


async def ask_llm(user_prompt: str) -> str:
    resp = await llm.chat.completions.create(
        model=MODEL,
        messages=[
            {"role": "system", "content": SYSTEM},
            {"role": "user", "content": user_prompt},
        ],
    )
    return resp.choices[0].message.content or ""


async def choose_move(board: chess.Board, san: list[str]) -> str:
    legal = [m.uci() for m in board.legal_moves]
    if not legal:
        raise ValueError("нет легальных ходов (партия окончена)")
    if len(legal) == 1:
        return legal[0]                      # выбора нет, LLM не нужна

    feedback = ""
    for attempt in range(1, MAX_ATTEMPTS + 1):
        text = await ask_llm(build_prompt(board, san, legal, feedback))
        for candidate in UCI_RE.findall(text):
            if candidate in legal:
                return candidate
        feedback = (
            f"Ответ «{text.strip()[:30]}» не из списка легальных ходов. "
            "Выбери строго из списка."
        )
        log.warning("попытка %d: нелегальный ответ LLM: %r", attempt, text[:50])

    if FALLBACK_RANDOM:
        return random.choice(legal)
    raise ValueError("LLM не вернула легальный ход")


# ---------- NATS ----------

async def publish_response(js, game_id: str, version: int, **body):
    payload = {"gameId": game_id, "version": version, **body}
    await js.publish(
        f"chess.ai.response.{game_id}",
        json.dumps(payload).encode(),
        headers={"Nats-Msg-Id": f"{game_id}:{version}"},  # дедупликация ответа
    )


async def heartbeat(msg):
    """Пока LLM думает, продлеваем ack_wait, чтобы сообщение не ушло второму воркеру."""
    while True:
        await asyncio.sleep(10)
        await msg.in_progress()


async def handle(js, msg):
    # 1. разбор: мусор повторять бессмысленно
    try:
        req = json.loads(msg.data)
        game_id, version = req["gameId"], int(req["version"])
        fen, history = req["fen"], req.get("history", [])
        color = req["color"]
        board, san = build_board(fen, history)
        # просят ходить за сторону, которая по FEN сейчас не ходит: данные рассинхронизированы
        if (board.turn == chess.WHITE) != (color == "white"):
            raise ValueError("очередь в FEN не совпадает с color")
    except (ValueError, KeyError, TypeError) as e:
        log.error("некорректный запрос, отбрасываю: %s", e)
        return await msg.term()

    # 2. расчёт хода
    hb = asyncio.create_task(heartbeat(msg))
    try:
        result = {"uci": await choose_move(board, san)}
    except ValueError as e:
        # LLM не справилась: сообщаем Nest, а не молчим
        log.error("игра %s: %s", game_id, e)
        result = {"error": str(e)}
    except Exception:
        # сеть, лимиты API, 5xx: повторим позже
        log.exception("сбой LLM, повтор через 5 с")
        return await msg.nak(delay=5)
    finally:
        hb.cancel()

    # 3. публикуем ответ и только потом ack
    try:
        await publish_response(js, game_id, version, **result)
    except Exception:
        log.exception("не удалось опубликовать ответ, повтор")
        return await msg.nak(delay=5)

    await msg.ack()
    log.info("игра %s v%d: %s", game_id, version, result)


async def main():
    logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
    nc = await nats.connect(NATS_URL)
    js = nc.jetstream()

    # стрим и консьюмер создаёт Nest (NatsBootstrapService), воркер только подключается
    sub = await js.pull_subscribe(REQUEST_FILTER, durable=DURABLE, stream=STREAM)

    stop = asyncio.Event()
    loop = asyncio.get_running_loop()
    for sig in (signal.SIGINT, signal.SIGTERM):
        try:
            loop.add_signal_handler(sig, stop.set)
        except NotImplementedError:          # Windows
            pass

    log.info("воркер запущен (модель %s), жду запросы", MODEL)
    while not stop.is_set():
        try:
            msgs = await sub.fetch(1, timeout=5)
        except NatsTimeout:
            continue
        for msg in msgs:
            await handle(js, msg)

    await nc.drain()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass