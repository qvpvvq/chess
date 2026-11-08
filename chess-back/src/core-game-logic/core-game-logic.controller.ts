import { Body, Controller, Get, Param, Post, Sse } from '@nestjs/common';
import { CoreGameLogicService } from './core-game-logic.service';
import { MoveDto } from './dto/move.dto';
import { interval, map, merge } from 'rxjs';
import { ChessEventsService } from './chess-events.service';

@Controller('chess')
export class CoreGameLogicController {
  constructor(
    private readonly service: CoreGameLogicService,
    private events: ChessEventsService,
  ) {}

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

  @Sse('/game:id/events')
  async stream(@Param('id') gameid: string) {
    await this.service.getWholeGame(gameid);
    const ping = interval(25_000).pipe(
      map(() => ({ type: 'ping', data: '' }) as MessageEvent),
    );

    return merge(this.events.stream(gameid), ping);
  }
}
