use crate::models::ChessRequestEvent;
use crate::{handle_analyze_full_game, handle_bot_move};
use async_nats::ConnectOptions;
use async_nats::jetstream;
use async_nats::jetstream::consumer::PullConsumer;
use futures_util::StreamExt;
use std::env;
async fn run_nats() -> Result<(), async_nats::Error> {
    let options = ConnectOptions::new()
        .retry_on_initial_connect()
        .max_reconnects(None);
    let client = options
        .connect(env::var("NATS_URL").expect("Incorrect NATS_URL (.env)"))
        .await?;
    let jetstream = jetstream::new(client);
    let consumer: PullConsumer = jetstream
        .get_consumer_from_stream("engine_worker", "CHESS")
        .await?;
    let mut messages = consumer.messages().await?;
    while let Some(msg) = messages.next().await {
        let msg = msg?;
        let subject = msg.subject.as_str();
        let result = match subject {
            s if s.ends_with(".analysis") => handle_analyze_full_game(&msg.payload).await,
            s if s.ends_with(".move") => handle_bot_move(&msg.payload).await,
            other => {
                eprintln!("Неизвестная тема: {other}, пропускаю");
                msg.ack().await?;
                continue;
            }
        };
    }

    Ok(())
}
