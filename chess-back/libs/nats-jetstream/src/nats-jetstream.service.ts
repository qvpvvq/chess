import {
  Injectable,
  Logger,
  OnModuleDestroy,
  OnModuleInit,
} from '@nestjs/common';
import { NatsConnection, connect, nanos } from '@nats-io/transport-node';
import {
  AckPolicy,
  DeliverPolicy,
  DiscardPolicy,
  jetstream,
  JetStreamClient,
  jetstreamManager,
  JetStreamManager,
  ReplayPolicy,
  RetentionPolicy,
  StorageType,
} from '@nats-io/jetstream';

@Injectable()
export class NatsJetstreamService implements OnModuleInit, OnModuleDestroy {
  private logger = new Logger(NatsJetstreamService.name);

  private natsConnection: NatsConnection;
  JSClient: JetStreamClient;
  JSManager: JetStreamManager;

  async onModuleInit() {
    try {
      this.natsConnection = await connect({
        servers: process.env.NATS_URL,
      });
      this.JSClient = jetstream(this.natsConnection);
      this.JSManager = await jetstreamManager(this.natsConnection);
      this.logger.debug('connected to nats jetstream');
    } catch (error) {
      this.logger.error('connection failed');
      throw error;
    }
  }

  async onModuleDestroy() {
    await this.natsConnection.drain();
  }

  async ensureMessagesStream(streamName: string, subjects: string[]) {
    const params = {
      name: streamName,
      subjects,
      retention: RetentionPolicy.Workqueue,
      discard: DiscardPolicy.Old,
      max_bytes: 1024 * 1024 * 1024,
      storage: StorageType.File,
      duplicate_window: nanos(2 * 60 * 1000),
      max_msgs_per_subject: -1,
    };

    let exists = true;

    try {
      await this.JSManager.streams.info(streamName);
    } catch {
      exists = false;
    }

    try {
      if (exists) {
        await this.JSManager.streams.update(streamName, params);
      } else {
        await this.JSManager.streams.add(params);
      }
    } catch (err) {
      this.logger.error(
        `Nats stream ${streamName} failed to create/update - ${err}`,
      );
    }
  }

  async ensureMessagesConsumer(
    streamName: string,
    consumerName: string,
    subjectToConsumeFrom: string,
  ) {
    const params = {
      durable_name: consumerName,
      filter_subject: subjectToConsumeFrom,
      ack_policy: AckPolicy.Explicit,
      deliver_policy: DeliverPolicy.All,
      replay_policy: ReplayPolicy.Instant,
      max_deliver: 5,
      backoff: [nanos(10_000), nanos(20_000), nanos(30_000)],
      max_ack_pending: 90,
    };

    let exists = true;

    try {
      await this.JSManager.consumers.info(streamName, consumerName);
    } catch (err) {
      exists = false;
    }

    try {
      if (!exists) {
        await this.JSManager.consumers.add(streamName, params);
      }
    } catch (err) {
      this.logger.error(`Nats JetStreamError - ${err}`);
    }
  }

  async publishMessage(subject: string, data: unknown, msgId?: string) {
    const pubAck = await this.JSClient.publish(subject, JSON.stringify(data), {
      msgID: msgId,
      timeout: 5_000,
    });

    if (pubAck.duplicate) {
      this.logger.debug(`message on ${subject} is a dublicate`);
    }

    return pubAck;
  }
}
