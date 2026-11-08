import { NatsJetstreamService } from '@app/nats-jetstream';
import { Injectable, OnModuleInit } from '@nestjs/common';
import { chessStream } from 'interfaces/chess.streams';

const streamSchemas = [
  // Схема потока шахматных ходов
  {
    name: chessStream.name,
    subjects: chessStream.subjects,
    consumers: [
      {
        name: chessStream.consumers.aiWorker.durable,
        filter: chessStream.consumers.aiWorker.filter,
      },
      {
        name: chessStream.consumers.aiResponses.durable,
        filter: chessStream.consumers.aiResponses.filter,
      },
    ],
  },
];

@Injectable()
export class NatsBootstrapService implements OnModuleInit {
  constructor(private JSService: NatsJetstreamService) {}

  async onModuleInit() {
    for (const stream of streamSchemas) {
      await this.JSService.ensureMessagesStream(stream.name, stream.subjects);
      for (const consumer of stream.consumers) {
        await this.JSService.ensureMessagesConsumer(
          stream.name,
          consumer.name,
          consumer.filter,
        );
      }
    }
  }
}
