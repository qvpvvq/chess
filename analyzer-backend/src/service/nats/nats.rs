use crate::core::models::AppState;
use crate::errors::AppError;
use crate::service::nats::events::NatsBotRequestEvent;
use crate::service::nats::nats_handlers::nats_bot_move;
use async_nats::ConnectOptions;
use async_nats::jetstream;
use async_nats::jetstream::AckKind;
use async_nats::jetstream::consumer::PullConsumer;
use bytes::Bytes;
use futures_util::StreamExt;
use std::env;
use std::sync::Arc;

pub async fn run_nats(state: AppState) -> Result<(), async_nats::Error> {
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
        eprintln!(
            "Получено сообщение: subject={}, payload={:?}",
            msg.subject,
            String::from_utf8_lossy(&msg.payload)
        );
        let output: Outcome =
            match process_message(msg.subject.as_str(), &msg.payload, Arc::clone(&state)).await {
                Err(e) => {
                    eprintln!("Ошибка сериализации входящего сообщения {e}");
                    ack(&msg, AckKind::Term).await;
                    continue;
                }
                Ok(output) => output,
            };
        match output {
            Outcome::Reply { subject, payload } => {
                let ack_future = match jetstream.publish(subject, payload).await {
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
                ack(&msg, AckKind::Ack).await;
                eprintln!("Сообщение подтверждено")
            }
            Outcome::Terminate => ack(&msg, AckKind::Term).await,
        }
    }
    Ok(())
}
enum Outcome {
    Reply { subject: String, payload: Bytes },
    Terminate,
}

async fn process_message(subject: &str, msg: &[u8], state: AppState) -> Result<Outcome, AppError> {
    let response: Outcome = match subject {
        subj if subj.ends_with(".analysis") => {
            eprintln!("analysis пока не реализован, subject={subj}");
            Outcome::Terminate
        }
        subj if subj.ends_with(".bot") => {
            let request = serde_json::from_slice::<NatsBotRequestEvent>(msg)
                .map_err(|e| AppError::InvalidNatsRequest(format!("Невалидный запрос {e}")))?;
            let result: Bytes = nats_bot_move(Arc::clone(&state), request).await?;
            let subject_response = "chess.engine.response.bot".to_string();

            Outcome::Reply {
                payload: result,
                subject: subject_response,
            }
        }
        _other => {
            eprintln!("Неизвестный subject, terminated: {subject}");
            Outcome::Terminate
        }
    };
    Ok(response)
}
async fn ack(msg: &jetstream::Message, kind: AckKind) {
    if let Err(e) = msg.ack_with(kind).await {
        eprintln!("Не удалось подтвердить сообщение ({kind:?}): {e}");
    }
}
