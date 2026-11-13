import { Module } from '@nestjs/common';
import { CommentResponseConsumerService } from './comment-response-consumer.service';
import { NatsJetstreamModule } from '@app/nats-jetstream';
import { PrismaModule } from 'src/prisma/prisma.module';
import { CoreGameLogicModule } from 'src/core-game-logic/core-game-logic.module';

@Module({
  imports: [NatsJetstreamModule, PrismaModule, CoreGameLogicModule],
  providers: [CommentResponseConsumerService],
})
export class CommentResponseConsumerModule {}
