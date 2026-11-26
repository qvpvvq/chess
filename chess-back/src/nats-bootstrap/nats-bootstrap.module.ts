import { Module } from '@nestjs/common';
import { NatsBootstrapService } from './nats-bootstrap.service';
import { NatsJetstreamModule } from '@app/nats-jetstream';

@Module({
  imports: [NatsJetstreamModule],
  providers: [NatsBootstrapService],
})
export class NatsBootstrapModule {}
