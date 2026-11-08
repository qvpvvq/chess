import { Module } from '@nestjs/common';
import { OutboxService } from './outbox.service';
import { PrismaModule } from 'src/prisma/prisma.module';
import { NatsJetstreamModule } from '@app/nats-jetstream';

@Module({
  imports: [PrismaModule, NatsJetstreamModule],
  providers: [OutboxService],
})
export class OutboxModule {}
