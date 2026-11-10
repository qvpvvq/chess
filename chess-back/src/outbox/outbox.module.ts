import { Module } from '@nestjs/common';
import { OutboxService } from './outbox.service';
import { PrismaModule } from 'src/prisma/prisma.module';
import { NatsJetstreamModule } from '@app/nats-jetstream';
import { OutboxPublisher } from './outbox.processor';

@Module({
  imports: [PrismaModule, NatsJetstreamModule],
  providers: [OutboxService, OutboxPublisher],
  exports: [OutboxService, OutboxPublisher],
})
export class OutboxModule {}
