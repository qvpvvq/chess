import { Module } from '@nestjs/common';
import { PrismaModule } from './prisma/prisma.module';
import { ConfigModule } from '@nestjs/config';
import { CoreGameLogicModule } from './core-game-logic/core-game-logic.module';
import { NatsBootstrapModule } from './nats-bootstrap/nats-bootstrap.module';
import { AiResponseConsumerModule } from './ai-response-consumer/ai-response-consumer.module';
import { OutboxModule } from './outbox/outbox.module';
import { ScheduleModule } from '@nestjs/schedule';
@Module({
  imports: [
    PrismaModule,
    ConfigModule.forRoot({
      envFilePath: '.env',
      isGlobal: true,
    }),
    CoreGameLogicModule,
    NatsBootstrapModule,
    AiResponseConsumerModule,
    OutboxModule,
    ScheduleModule.forRoot(),
  ],
  controllers: [],
  providers: [],
})
export class AppModule {}
