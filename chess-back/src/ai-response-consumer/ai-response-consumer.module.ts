import { Module } from '@nestjs/common';
import { AiResponseConsumerService } from './ai-response-consumer.service';
import { NatsJetstreamModule } from '@app/nats-jetstream';
import { CoreGameLogicModule } from 'src/core-game-logic/core-game-logic.module';

@Module({
  imports: [NatsJetstreamModule, CoreGameLogicModule],
  providers: [AiResponseConsumerService],
})
export class AiResponseConsumerModule {}
