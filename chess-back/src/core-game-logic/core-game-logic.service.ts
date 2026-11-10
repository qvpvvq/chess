import {
  BadRequestException,
  ConflictException,
  Injectable,
  Logger,
  NotFoundException,
} from '@nestjs/common';
import { PrismaService } from 'src/prisma/prisma.service';

import { Chess, Move } from 'chess.js';
import { randomUUID } from 'crypto';
import { MoveDto } from './dto/move.dto';
import { Game, GameStatus, moveRole } from 'generated/prisma';
import { Figure, NAMES, ROOK_CASTLE, toColor } from 'interfaces/chess.types';
import { OutboxService } from 'src/outbox/outbox.service';
import { chessPublishSubjects } from 'interfaces/chess.streams';

function computeMove(gameData: Game, dto: MoveDto) {
  const chess = new Chess(gameData.fen);

  // делаем ход - валидируем
  let move: Move;
  try {
    move = chess.move({
      from: dto.from,
      to: dto.to,
      promotion: dto.promotion || 'q',
    }); // проверяем валидность хода
  } catch {
    throw new BadRequestException('Нелегальный ход');
  }

  const figures = (gameData.figures as unknown as Figure[]).map((f) => ({
    ...f, // берем массив фигур из сессии - типизируем (тк в prisma сохраняется просто JSON)
  }));

  // поедание фигуры
  let captured: Figure | null = null;
  if (move.isCapture()) {
    // при взятии на проходе съеденная пешка стоит не на клетке "to"
    const square = move.isEnPassant() ? dto.to[0] + dto.from[1] : dto.to;
    captured = figures.find((f) => f.position === square)!;
    captured.position = null;
  }

  // рокировка
  if (move.isKingsideCastle() || move.isQueensideCastle()) {
    const side = move.isKingsideCastle() ? 'king' : 'queen';
    const [rookFrom, rookTo] = ROOK_CASTLE[move.color][side];
    figures.find((f) => f.position === rookFrom)!.position = rookTo;
  }

  // перемещение фигуры
  const movingFigure = figures.find((f) => f.position === dto.from)!;
  movingFigure.position = dto.to;
  if (move.isPromotion()) {
    movingFigure.name = NAMES[move.promotion!];
  }

  const status = chess.isCheckmate()
    ? GameStatus.checkmate
    : chess.isStalemate()
      ? GameStatus.stalemate
      : chess.isDraw()
        ? GameStatus.draw
        : GameStatus.active;

  return { figures, status, captured, chess, move };
}

@Injectable()
export class CoreGameLogicService {
  private logger = new Logger(CoreGameLogicService.name);

  constructor(
    private readonly prisma: PrismaService,
    private outbox: OutboxService,
  ) {}

  async startGame() {
    const chess = new Chess();

    const figures = chess
      .board()
      .flat()
      .filter((cell) => cell !== null)
      .map((cell) => ({
        id: randomUUID(),
        name: NAMES[cell.type],
        color: toColor(cell.color),
        position: cell.square,
      }));

    return this.prisma.game.create({
      data: { fen: chess.fen(), figures: figures },
    });
  }

  async makeAMove(gameId: string, dto: MoveDto) {
    const gameData = await this.prisma.game.findUnique({
      where: { id: gameId },
    });
    if (!gameData) throw new NotFoundException('партия не найдена');
    if (gameData.status !== GameStatus.active)
      throw new BadRequestException('Партия завершена');

    const { figures, chess, status, captured, move } = computeMove(
      gameData,
      dto,
    );

    const gameUpdated = await this.prisma.$transaction(async (trans) => {
      const updatedGame = await trans.game.update({
        where: { id: gameId, version: gameData.version },
        data: {
          fen: chess.fen(),
          figures: figures,
          turn: toColor(chess.turn()),
          status,
          winner: status === GameStatus.checkmate ? gameData.turn : null,
          version: { increment: 1 },
        },
        include: { moves: true },
      });

      if (!updatedGame)
        throw new ConflictException('Версия партии изменилась, повторите');

      await trans.move.create({
        data: {
          gameId,
          ply: gameData.version + 1,
          color: gameData.turn,
          san: move.san,
          uci: move.from + move.to + (move.promotion ?? ''),
          fenAfter: chess.fen(),
        },
      });

      const movesUciHistory = updatedGame.moves.map((m) => m.uci);

      const event = {
        gameId: updatedGame.id,

        version: updatedGame.version,
        fen: updatedGame.fen,
        color: updatedGame.turn,
        history: movesUciHistory,
      };

      await this.outbox.enqueue(trans, chessPublishSubjects.request, event);
      return updatedGame;
    });

    return { san: move.san, captured, game: gameUpdated };
  }

  async getWholeGame(gameId: string) {
    const game = await this.prisma.game.findUnique({
      where: { id: gameId },
      include: { moves: { orderBy: { ply: 'asc' } } },
    });

    if (!game) throw new NotFoundException('Игра не найдена');

    return game;
  }

  async applyAiMove(
    gameId: string,
    dto: MoveDto,
    author: moveRole,
    expectedVersion?: number,
  ) {
    const gameData = await this.prisma.game.findUnique({
      where: { id: gameId },
    });

    if (!gameData) throw new NotFoundException('Партия не найдена');

    const aiTurn = gameData.aiColor === gameData.turn;
    if (author === moveRole.AI && !aiTurn)
      throw new ConflictException('Сейчас не очередь ИИ');
    if (author === moveRole.USER && aiTurn)
      throw new BadRequestException('Сейчас ходит ИИ');

    if (!!expectedVersion && gameData.version !== expectedVersion) {
      throw new ConflictException('Позиция партии изменилась');
    }

    const { figures, chess, status, captured, move } = computeMove(
      gameData,
      dto,
    );

    const updated = await this.prisma.$transaction(async (tx) => {
      const { count } = await tx.game.updateMany({
        where: { id: gameId, version: gameData.version },
        data: {
          fen: chess.fen(),
          figures: figures as any,
          turn: toColor(chess.turn()),
          status,
          winner: status === GameStatus.checkmate ? gameData.turn : null,
          version: { increment: 1 },
        },
      });
      if (count === 0) throw new ConflictException('Версия партии изменилась');

      await tx.move.create({
        data: {
          gameId,
          ply: gameData.version + 1,
          color: gameData.turn,
          role: author,
          san: move.san,
          uci: move.from + move.to + (move.promotion ?? ''),
          fenAfter: chess.fen(),
        },
      });
      return tx.game.findUniqueOrThrow({ where: { id: gameId } });
    });

    return { san: move.san, captured, game: updated };
  }
}
