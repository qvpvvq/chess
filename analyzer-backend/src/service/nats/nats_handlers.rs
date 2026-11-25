use crate::{
    core::{bot::bot_move, models::AppState},
    errors::AppError,
    service::nats::events::{NatsBotRequestEvent, NatsBotResponseEvent},
};
use bytes::Bytes;
pub async fn nats_bot_move(
    state: AppState,
    request: NatsBotRequestEvent,
) -> Result<Bytes, AppError> {
    let bot_result = match bot_move(state, request.fen).await {
        Ok((chessmove, eval)) => NatsBotResponseEvent::Ok {
            game_id: request.game_id,
            version: request.version,
            uci: chessmove,
            eval,
        },
        Err(e) => NatsBotResponseEvent::Error {
            game_id: request.game_id,
            version: request.version,
            error: e.to_string(),
        },
    };
    let payload = Bytes::from(serde_json::to_vec(&bot_result)?);
    Ok(payload)
}
