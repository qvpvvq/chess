import { NatsJetstreamService } from '@app/nats-jetstream';
import { Injectable, Logger } from '@nestjs/common';
import { PrismaService } from 'src/prisma/prisma.service';
import { Interval } from '@nestjs/schedule';
import {
  chessPublishSubjects,
  chessPublishSubjectsBuilder,
} from 'interfaces/chess.streams';

interface Claimed {
  id: string;
  type: string;
  payload: unknown;
  attempts: number;
}

function subjectBuilder(type: string, payload: any): string | undefined {
  switch (type) {
    case chessPublishSubjects.request:
      return chessPublishSubjectsBuilder(payload.id).request;
    default:
      return undefined;
  }
}

@Injectable()
export class OutboxPublisher {
  private readonly logger = new Logger(OutboxPublisher.name);
  private running = false;

  constructor(
    private prisma: PrismaService,
    private JSService: NatsJetstreamService,
  ) {}

  @Interval(1_000)
  async tick() {
    if (this.running) return;
    this.running = true;

    try {
      for (let i = 0; i < 20; i++) {
        const [ev] = await this.claim();
        if (!ev) break;
        try {
          const subject = subjectBuilder(ev.type, ev.payload);
          if (!subject) throw new Error(`unknown outbox type: ${ev.type}`);

          await this.JSService.publishMessage(subject, ev.payload, ev.id);
          this.logger.debug(`succesfully published`);
          await this.prisma.outbox.update({
            where: { id: ev.id },
            data: { sentAt: new Date(), lastError: null },
          });
        } catch (err) {
          await this.scheduleRetry(ev, err.message);
          this.logger.warn(
            `outbox publish failed id=${ev.id} type=${ev.type}: ${(err as Error).message}`,
          );
        }
      }
    } finally {
      this.running = false;
    }
  }

  private async claim(): Promise<Claimed[]> {
    const candidates = await this.prisma.outbox.findMany({
      where: { sentAt: null, nextAttemptAt: { lte: new Date() } },
      orderBy: { createdAt: 'asc' },
      take: 1,
    });

    const claimed: Claimed[] = [];

    for (const event of candidates) {
      const res = await this.prisma.outbox.updateMany({
        where: {
          id: event.id,
          sentAt: null,
          nextAttemptAt: event.nextAttemptAt,
        },
        data: {
          nextAttemptAt: new Date(Date.now() + 30_000),
          attempts: { increment: 1 },
        },
      });

      if (res.count === 1) {
        claimed.push({
          id: event.id,
          type: event.type,
          payload: event.payload,
          attempts: event.attempts + 1,
        });
      }
    }

    return claimed;
  }

  private async scheduleRetry(ev: Claimed, error: string) {
    const delaySec = Math.min(2 ** ev.attempts, 60);
    const message = error.slice(0, 500);

    await this.prisma.outbox.update({
      where: { id: ev.id },
      data: {
        lastError: message,
        nextAttemptAt: new Date(Date.now() + delaySec * 1000),
      },
    });

    this.logger.warn(
      `outbox publish failed id=${ev.id} attempt=${ev.attempts}`,
    );
  }
}
