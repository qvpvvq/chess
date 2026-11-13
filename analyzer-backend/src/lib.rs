// use axum::{Router, routing::get};
pub mod analyzer;
mod bot_play;
mod errors;
mod extractor;
mod models;
use crate::errors::AppError;
use crate::models::{MoveAnalysis, ParsedMove};
use analyzer::Analyzer;
use axum::Json;
use axum::extract::State;
pub use std::sync::Arc;
use tokio::sync::Mutex;
pub type AppState = Arc<Mutex<Analyzer>>;
pub async fn handle_analyze_full_game(
    State(state): State<AppState>,
    body: String,
) -> Result<Json<Vec<MoveAnalysis>>, AppError> {
    println!("Запрос дошел");
    let mut analyzer = state.lock().await;
    println!("Анализатор создан");
    let moves = handle_parse_pgn(&body)?;
    println!("{:?}", moves);
    let analyzed_moves = analyzer.analyze_from_uci(moves)?;
    Ok(Json(analyzed_moves))
}
fn handle_parse_pgn(pgn: &str) -> Result<Vec<ParsedMove>, AppError> {
    if let Some((_, moves)) = crate::extractor::parse_pgn(pgn) {
        if moves.is_empty() {
            return Err(AppError::BadPgn(
                "Партия не содержит валидных ходов".to_string(),
            ));
        }
        Ok(moves)
    } else {
        Err(AppError::BadPgn("Не удалось распарсить PGN".to_string()))
    }
}
