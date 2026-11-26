import { NatsJetstreamService } from '@app/nats-jetstream';
import { Injectable, OnModuleInit } from '@nestjs/common';
import { chessStream, commentStream } from 'interfaces/chess.streams';

const streamSchemas = [
  // Схема потока шахматных ходов (игрок походил - движок ответил)
  {
    name: chessStream.name,
    subjects: chessStream.subjects,
    consumers: [
      {
        name: chessStream.consumers.engineWorker.durable,
        filter: chessStream.consumers.engineWorker.filter, // консьюмит Rust
      },
      {
        name: chessStream.consumers.engineResponses.durable, // консьюмит Nest
        filter: chessStream.consumers.engineResponses.filter,
      },
    ],
  },
  {
    name: commentStream.name,
    subjects: commentStream.subjects,
    consumers: [
      {
        name: commentStream.consumers.commentRequest.durable,
        filter: commentStream.consumers.commentRequest.filter, // консьюмит python
      },
      {
        name: commentStream.consumers.commentResponse.durable,
        filter: commentStream.consumers.commentResponse.filter, // консьюмит nest
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
