import { NatsJetstreamService } from '@app/nats-jetstream';
import { ConsumerMessages, JsMsg } from '@nats-io/jetstream';
import {
  BadRequestException,
  Injectable,
  Logger,
  OnApplicationBootstrap,
  OnModuleDestroy,
} from '@nestjs/common';
import { moveRole } from 'generated/prisma';
import { ChessResponseEvent, chessStream } from 'interfaces/chess.streams';
import { ChessEventsService } from 'src/core-game-logic/chess-events.service';
import { CoreGameLogicService } from 'src/core-game-logic/core-game-logic.service';
import { MoveDto } from 'src/core-game-logic/dto/move.dto';

const UCI = /^([a-h][1-8])([a-h][1-8])([qrbn])?$/;

function UciToMove(uci: string): MoveDto {
  const parsedUciMove = UCI.exec(uci);

  if (!parsedUciMove)
    throw new BadRequestException(`Некоректная строка uci: ${uci}`);

  return {
    from: parsedUciMove[1],
    to: parsedUciMove[2],
    promotion: parsedUciMove[3] as MoveDto['promotion'],
  };
}

@Injectable()
export class AiResponseConsumerService
  implements OnApplicationBootstrap, OnModuleDestroy
{
  private logger = new Logger(AiResponseConsumerService.name);
  private messages: ConsumerMessages;

  constructor(
    private JSService: NatsJetstreamService,
    private gameService: CoreGameLogicService,
    private chessEvents: ChessEventsService,
  ) {}

  async onApplicationBootstrap() {
    const consumer = await this.JSService.JSClient.consumers.get(
      chessStream.name,
      chessStream.consumers.engineResponses.durable,
    );

    this.messages = await consumer.consume({ max_messages: 20 });

    void this.processMessages();
  }

  onModuleDestroy() {
    this.messages.stop();
  }

  async processMessages() {
    for await (const msg of this.messages) {
      await this.handleMessage(msg);
    }
  }

  async handleMessage(msg: JsMsg) {
    let event: ChessResponseEvent;
    try {
      event = msg.json<ChessResponseEvent>();
    } catch {
      return msg.term();
    }

    if ('error' in event) {
      this.logger.error(`ИИ не смог сходить в ${event.gameId}: ${event.error}`);
      this.chessEvents.emit(event.gameId, 'ai_error', {
        version: event.version,
      });
      return msg.ack();
    }

    try {
      const dto = UciToMove(event.uci);
      const result = await this.gameService.applyAiMove(
        event.gameId,
        dto,
        moveRole.AI,
        event.version,
        event.evalCp,
      );

      this.chessEvents.emit(event.gameId, 'move', {
        version: result.game.version,
      });
      return msg.ack();
    } catch (error) {
      this.logger.error(`failed to compile message - ${error}`);
      return msg.nak();
    }
  }
}
