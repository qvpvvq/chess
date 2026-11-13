import { NatsJetstreamService } from '@app/nats-jetstream';
import { ConsumerMessages, JsMsg } from '@nats-io/jetstream';
import {
  Injectable,
  Logger,
  OnApplicationBootstrap,
  OnModuleDestroy,
} from '@nestjs/common';
import { CommentResponseEvent, commentStream } from 'interfaces/chess.streams';
import { ChessEventsService } from 'src/core-game-logic/chess-events.service';
import { PrismaService } from 'src/prisma/prisma.service';

@Injectable()
export class CommentResponseConsumerService
  implements OnApplicationBootstrap, OnModuleDestroy
{
  private logger = new Logger(CommentResponseConsumerService.name);
  private messages: ConsumerMessages;

  constructor(
    private JSService: NatsJetstreamService,
    private prisma: PrismaService,
    private chessEvents: ChessEventsService,
  ) {}

  async onApplicationBootstrap() {
    const consumer = await this.JSService.JSClient.consumers.get(
      commentStream.name,
      commentStream.consumers.commentResponse.durable,
    );

    this.messages = await consumer.consume({ max_messages: 20 });
    void this.processMessages();
  }

  onModuleDestroy() {
    this.messages.stop();
  }

  async processMessages() {
    for await (const m of this.messages) {
      await this.handleMessage(m);
    }
  }

  async handleMessage(msg: JsMsg) {
    let event: CommentResponseEvent;
    try {
      event = msg.json<CommentResponseEvent>();
    } catch (error) {
      this.logger.error(`cannot parse message - ${error}`);
      return msg.term();
    }

    try {
      switch (event.type) {
        case 'INTRO':
          await this.prisma.game.update({
            where: { id: event.gameId },
            data: {
              comments: [event.text],
            },
          });

          this.logger.debug(`saved intro dialogue`);

          this.chessEvents.emit(event.gameId, 'comment', {
            gameId: event.gameId,
          });

          return msg.ack();
        case 'MOVE':
          if (event.moveId !== undefined) {
            await this.prisma.move.update({
              where: {
                id: event.moveId,
                gameId: event.gameId,
              },
              data: {
                comment: event.text,
              },
            });

            this.logger.debug(`saved move comment`);

            this.chessEvents.emit(event.gameId, 'comment', {
              gameId: event.gameId,
            });
            return msg.ack();
          } else {
            this.logger.debug('move id не был передан');
            return msg.term();
          }
        default:
          this.logger.debug('wrong event.type');
          return msg.term();
      }
    } catch (error) {
      this.logger.error(`This instance failed to process msg - ${error}`);
      return msg.nak();
    }
  }
}
