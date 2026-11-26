import { Global, Module } from '@nestjs/common';
import { NatsJetstreamService } from './nats-jetstream.service';

@Global()
@Module({
  providers: [NatsJetstreamService],
  exports: [NatsJetstreamService],
})
export class NatsJetstreamModule {}
