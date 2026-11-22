mod events;

use crate::core::Analyzer;
use crate::core::bot::bot_move;
use crate::errors::AppError;
use crate::service::events::ChessBotRequestEvent;
use crate::service::handlers::AppState;
use async_nats::ConnectOptions;
use async_nats::jetstream;
use async_nats::jetstream::consumer::PullConsumer;
use futures_util::StreamExt;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

// pub use handlers;
async fn run_nats(state: Arc<Mutex<Analyzer>>) -> Result<(), async_nats::Error> {
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
    while let Some(item) = messages.next().await {
        let msg = match item {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Ошибка получения из JetStream: {e}");
                continue;
            }
        };
        let output: Outcome =
            match process_message(msg.subject.as_str(), &msg.payload, state.clone()).await {
                Err(e) => {
                    eprintln!("Ошибка чтения сообщения Jetstream {e}");
                    if let Err(e) = msg.ack().await {
                        eprintln!("Не удалось подтвердить входящее сообщение: {e}");
                    }
                    continue;
                }
                Ok(output) => output,
            };
        match output {
            Outcome::Reply { subject, payload } => {
                let ack_future = match jetstream.publish(subject, payload.into()).await {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Ошибка публикации сообщения {e}");
                        continue;
                    }
                };
                if let Err(e) = ack_future.await {
                    eprintln!("Сервер не подтвердил запись ответа: {e}");
                    continue;
                }
            }
            Outcome::Skip => {
                if let Err(e) = msg.ack().await {
                    eprintln!("Не удалось подтвердить входящее сообщение: {e}");
                }
            }
        }
    }
    Ok(())
}
enum Outcome {
    Reply { subject: String, payload: Vec<u8> },
    Skip,
}
async fn process_message(subject: &str, msg: &[u8], state: AppState) -> Result<Outcome, AppError> {
    let response: Outcome = match subject {
        subj if subj.ends_with(".analysis") => {
            todo!();
        }
        subj if subj.ends_with(".bot") => {
            let request = serde_json::from_slice::<ChessBotRequestEvent>(msg)
                .map_err(|e| AppError::InvalidNatsRequest(format!("Невалидный запрос {e}")))?;
            let result = bot_move(Arc::clone(&state), request).await;
            let subject = "chess.engine.response.bot".to_string();
            let response_json = serde_json::to_vec(&result)
                .map_err(|e| AppError::InvalidNatsRequest(format!("Невалидный запрос {e}")))?;
            Outcome::Reply {
                payload: response_json,
                subject,
            }
        }
        _other => Outcome::Skip,
    };
    Ok(response)
}
