import { Module } from '@nestjs/common';
import { CoreGameLogicController } from './core-game-logic.controller';
import { CoreGameLogicService } from './core-game-logic.service';
import { PrismaModule } from 'src/prisma/prisma.module';

@Module({
  imports: [PrismaModule],
  controllers: [CoreGameLogicController],
  providers: [CoreGameLogicService],
})
export class CoreGameLogicModule {}
