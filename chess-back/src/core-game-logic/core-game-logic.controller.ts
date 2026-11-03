import { Body, Controller, Get, Param, Post } from '@nestjs/common';
import { CoreGameLogicService } from './core-game-logic.service';
import { MoveDto } from './dto/move.dto';

@Controller('chess')
export class CoreGameLogicController {
  constructor(private readonly service: CoreGameLogicService) {}

  @Post('start')
  startGame() {
    return this.service.startGame();
  }

  @Post(':id/move')
  makeAMove(@Param('id') id: string, @Body() dto: MoveDto) {
    return this.service.makeAMove(id, dto);
  }

  @Get('/game/:id')
  getGameInfo(@Param('id') id: string) {
    return this.service.getWholeGame(id);
  }
}
