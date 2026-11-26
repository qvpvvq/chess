import asyncio
import json
import logging
import os
import signal

from dotenv import load_dotenv

load_dotenv()  # до чтения os.environ

import nats
from nats.errors import TimeoutError as NatsTimeout
from openai import AsyncOpenAI

NATS_URL = os.getenv("NATS_URL", "nats://localhost:4222")
STREAM = "CHESS_COMMENTS"
DURABLE = "comment_request"                    # должен совпадать с консьюмером в Nest
REQUEST_FILTER = "chess.comment.request.>"
MODEL = os.environ["LLM_MODEL"]
MAX_TOKENS = int(os.getenv("LLM_MAX_TOKENS", "512"))
REASONING_EFFORT = os.getenv("LLM_REASONING_EFFORT")
BATCH = 4                                     # ход игрока и ход ИИ считаем параллельно
MAX_PERSONA_LEN = 400
MAX_MEMORY = 8
MAX_REPLY_LEN = 500

log = logging.getLogger("comment-worker")

llm = AsyncOpenAI(
    api_key=os.environ["LLM_API_KEY"],        # нет ключа: падаем сразу при старте
    base_url=os.getenv("LLM_BASE_URL") or None,
    timeout=30,                               # без таймаута зависший запрос блокирует воркер
    max_retries=1,
)


# ---------- промпты ----------

def judge(color: str, before: int | None, after: int | None) -> str | None:
    """Качество хода считаем кодом: LLM в шахматах ошибается и хвалит зевки."""
    if before is None or after is None:
        return None
    loss = (before - after) * (1 if color == "white" else -1)  # сколько потерял ходивший
    if loss <= 10:
        return "точный ход"
    if loss <= 50:
        return "хороший ход"
    if loss <= 100:
        return "неточность"
    if loss <= 250:
        return "ошибка"
    return "грубая ошибка"


def build_system(persona: str) -> str:
    return (
        f"Ты шахматный соперник и комментатор. Твой образ: {persona}\n"
        "Всегда оставайся в образе. Отвечай на русском, 1-2 короткими предложениями, "
        "без кавычек и без пояснений вне роли. "
        "Опирайся только на факты из запроса, не придумывай ходы и оценки позиции."
    )


def build_prompt(r: dict) -> str:
    memory = "\n".join(f"- {t}" for t in r["previous"])
    parts = [f"Ранее ты говорил:\n{memory}" if memory else ""]

    if r["type"] == "INTRO":
        parts.append(
            "Партия только началась. Поприветствуй игрока и брось ему вызов в своём образе."
        )
    else:
        by_ai = r["author"] == "AI"
        verdict = None if by_ai else judge(r["color"], r["evalBeforeCp"], r["evalAfterCp"])
        parts += [
            f"Ход сделал: {'ты сам' if by_ai else 'игрок'}, ход {r['san']}.",
            f"Последние ходы: {' '.join(r['historySan'])}" if r["historySan"] else "",
            f"Оценка движка: {verdict}." if verdict else "",
            "Говори от первого лица («я пошёл...»)." if by_ai else "Прокомментируй ход игрока.",
            "Не повторяй свои прошлые фразы.",
        ]
    return "\n".join(p for p in parts if p)


async def ask_llm(system: str, user: str) -> str:
    extra = {"reasoning_effort": REASONING_EFFORT} if REASONING_EFFORT else None
    resp = await llm.chat.completions.create(
        model=MODEL,
        messages=[
            {"role": "system", "content": system},
            {"role": "user", "content": user},
        ],
        max_completion_tokens=MAX_TOKENS,
        extra_body=extra,
    )
    choice = resp.choices[0]
    text = (choice.message.content or "").strip().strip('"«»').strip()
    if not text:
        # у моделей с рассуждениями бывает пустой ответ, если токены ушли на раздумья
        log.warning("пустой ответ LLM, finish_reason=%s", choice.finish_reason)
    return text[:MAX_REPLY_LEN]


# ---------- разбор запроса ----------

def parse_request(raw: bytes) -> dict:
    """Из NATS приходит что угодно: проверяем форму вручную. ValueError = мусор, повтор не поможет."""
    try:
        r = json.loads(raw)
    except json.JSONDecodeError as e:
        raise ValueError(f"не JSON: {e}")
    if not isinstance(r, dict):
        raise ValueError("ожидался объект")

    game_id = r.get("gameId")
    type = r.get("type")
    persona = r.get("persona")
    if not isinstance(game_id, str) or not game_id:
        raise ValueError("нет gameId")
    if type not in ("INTRO", "MOVE"):
        raise ValueError(f"неизвестный type: {type!r}")
    if not isinstance(persona, str) or not persona.strip():
        raise ValueError("нет persona")

    previous = r.get("previousComments") or []
    if not isinstance(previous, list):
        raise ValueError("previousComments должен быть массивом")

    out = {
        "gameId": game_id,
        "type": type,
        "persona": persona.strip()[:MAX_PERSONA_LEN],
        "previous": [str(t)[:300] for t in previous[-MAX_MEMORY:]],
        "moveId": None,                      # <- новое
        "ply": None, "author": None, "color": None, "san": None,
        "historySan": [], "evalBeforeCp": None, "evalAfterCp": None,
    }

    if type == "MOVE":
        move_id = r.get("moveId")
        if not isinstance(move_id, str) or not move_id:
            raise ValueError("нет moveId")
        # Prisma-enum приходит как WHITE/BLACK, внутри работаем со строчными
        color = str(r.get("color", "")).lower()
        if color not in ("white", "black"):
            raise ValueError("color должен быть white или black")
        if r.get("author") not in ("USER", "AI"):
            raise ValueError("author должен быть USER или AI")
        if not isinstance(r.get("san"), str) or not r["san"]:
            raise ValueError("нет san")
        if not isinstance(r.get("ply"), int):
            raise ValueError("нет ply")
        out.update(
            moveId=move_id,
            ply=r["ply"], author=r["author"], color=color, san=r["san"],
            historySan=[str(m) for m in (r.get("historySan") or [])][-10:],
            evalBeforeCp=r.get("evalBeforeCp"), evalAfterCp=r.get("evalAfterCp"),
        )
    return out


# ---------- NATS ----------

async def publish_comment(js, req: dict, text: str):
    payload = {
        "gameId": req["gameId"],
        "type": req["type"],
        "moveId": req["moveId"],             # None для INTRO
        "ply": req["ply"],
        "text": text,
    }
    # id дедупликации: для хода берём moveId, он уникален и не зависит от ply
    msg_id = (
        f"intro:{req['gameId']}" if req["type"] == "INTRO"
        else f"move:{req['moveId']}"
    )
    await js.publish(
        f"chess.comment.response.{req['gameId']}",
        json.dumps(payload, ensure_ascii=False).encode(),
        headers={"Nats-Msg-Id": msg_id},
    )

async def handle(js, msg):
    # 1. разбор: мусор повторять бессмысленно
    try:
        req = parse_request(msg.data)
    except ValueError as e:
        log.error("некорректный запрос, отбрасываю: %s", e)
        return await msg.term()

    # 2. генерация. Комментарий необязателен: при сбое повторяем один раз и сдаёмся,
    # партия от этого не страдает (лимит доставок задаёт консьюмер в Nest)
    try:
        text = await ask_llm(build_system(req["persona"]), build_prompt(req))
    except Exception:
        log.exception("сбой LLM (игра %s, %s), повтор через 3 с", req["gameId"], req["type"])
        return await msg.nak(delay=3)

    if not text:
        return await msg.nak(delay=3)

    # 3. публикуем и только потом ack
    try:
        await publish_comment(js, req, text)
    except Exception:
        log.exception("не удалось опубликовать комментарий, повтор")
        return await msg.nak(delay=3)

    await msg.ack()
    log.info("игра %s %s ply=%s: %s", req["gameId"], req["type"], req["ply"], text)


async def main():
    logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
    nc = await nats.connect(NATS_URL)
    js = nc.jetstream()

    # стрим и консьюмер создаёт Nest (NatsBootstrapService), воркер только подключается
    sub = await js.pull_subscribe_bind(durable=DURABLE, stream=STREAM)

    stop = asyncio.Event()
    loop = asyncio.get_running_loop()
    for sig in (signal.SIGINT, signal.SIGTERM):
        try:
            loop.add_signal_handler(sig, stop.set)
        except NotImplementedError:           # Windows
            pass

    log.info("комментатор запущен (модель %s), жду запросы", MODEL)
    while not stop.is_set():
        try:
            msgs = await sub.fetch(BATCH, timeout=5)
        except NatsTimeout:
            continue
        # сообщения обрабатываются параллельно: два комментария за один ход не ждут друг друга
        await asyncio.gather(*(handle(js, m) for m in msgs))

    await nc.drain()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass