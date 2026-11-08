import { Injectable, MessageEvent } from '@nestjs/common';
import { filter, map, Subject } from 'rxjs';

@Injectable()
export class ChessEventsService {
  private bus = new Subject<{ gameId: string; event: MessageEvent }>();

  emit(gameId: string, type: 'move' | 'ai_error', data: unknown) {
    this.bus.next({ gameId, event: { type, data } as MessageEvent });
  }

  stream(gameId: string) {
    return this.bus.pipe(
      filter((x) => x.gameId === gameId),
      map((x) => x.event),
    );
  }
}
