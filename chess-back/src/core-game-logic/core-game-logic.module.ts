import { Module } from '@nestjs/common';
import { CoreGameLogicController } from './core-game-logic.controller';
import { CoreGameLogicService } from './core-game-logic.service';
import { PrismaModule } from 'src/prisma/prisma.module';
import { OutboxModule } from 'src/outbox/outbox.module';
import { ChessEventsService } from './chess-events.service';

@Module({
  imports: [PrismaModule, OutboxModule],
  controllers: [CoreGameLogicController],
  providers: [CoreGameLogicService, ChessEventsService],
  exports: [CoreGameLogicService, ChessEventsService],
})
export class CoreGameLogicModule {}
