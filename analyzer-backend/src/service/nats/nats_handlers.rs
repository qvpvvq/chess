use crate::core::extractor::parse_fen;
use crate::service::events::{ChessBotRequestEvent, ChessBotResponseEvent};
use crate::service::handlers::AppState;

pub async fn nats_handle_bot_move(
    state: AppState,
    request: ChessBotRequestEvent,
) -> ChessBotResponseEvent {
    let fen = match parse_fen(&request.fen) {
        Ok(fen) => fen,
        Err(e) => {
            return ChessBotResponseEvent::Error {
                game_id: request.game_id,
                version: request.version,
                error: e.to_string(),
            };
        }
    };
    let mut analyzer = state.lock().await;
    let response = match analyzer.get_move_from_fen(fen) {
        Ok((eng_move, eval)) => ChessBotResponseEvent::Ok {
            game_id: request.game_id,
            version: request.version,
            uci: eng_move,
            eval,
        },
        Err(error) => ChessBotResponseEvent::Error {
            game_id: request.game_id,
            version: request.version,
            error,
        },
    };
    response
}
