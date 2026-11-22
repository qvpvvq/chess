use crate::core::analyzer::Analyzer;
use crate::core::extractor::GameMetadata;
use crate::core::extractor::{parse_fen, parse_pgn};
use crate::core::models::{MoveAnalysis, ParsedMove};
use crate::errors::AppError;
use crate::service::events::{ChessBotResponseEvent, ChessBotRequestEvent};
use axum::Json;
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppState = Arc<Mutex<Analyzer>>;

pub async fn handle_analyze_full_game(
    State(state): State<AppState>,
    body: String,
) -> Result<Json<Vec<MoveAnalysis>>, AppError> {
    println!("Ok: Запрос дошел");
    let mut analyzer = state.lock().await;
    println!("Ok: Анализатор занят для анализа партии");
    let (metadata, moves) = handle_parse_pgn(&body)?;
    println!("{:#?} {:?}", metadata, moves);
    let analyzed_moves = analyzer.analyze_from_uci(moves)?;
    Ok(Json(analyzed_moves))
}
fn handle_parse_pgn(pgn: &str) -> Result<(GameMetadata, Vec<ParsedMove>), AppError> {
    parse_pgn(pgn)
}

pub async fn handle_bot_move(
    State(state): State<AppState>,
    Json(request): Json<ChessBotRequestEvent>,
) -> Json<ChessBotResponseEvent> {
    let fen = match parse_fen(&request.fen) {
        Ok(fen) => fen,
        Err(e) => {
            return Json(ChessBotResponseEvent::Error {
                game_id: request.game_id,
                version: request.version,
                error: e.to_string(),
            });
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
    Json(response)
}
