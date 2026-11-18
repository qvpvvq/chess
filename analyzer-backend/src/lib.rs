// use axum::{Router, routing::get};
pub mod analyzer;
mod bot_play;
mod errors;
mod extractor;
mod models;
use extractor::GameMetadata;

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
    println!("Ok: Запрос дошел");
    let mut analyzer = state.lock().await;
    println!("Ok: Анализатор занят для анализа партии");
    let (metadata, moves) = handle_parse_pgn(&body)?;
    println!("{:#?} {:?}", metadata, moves);
    let analyzed_moves = analyzer.analyze_from_uci(moves)?;
    Ok(Json(analyzed_moves))
}
fn handle_parse_pgn(pgn: &str) -> Result<(GameMetadata, Vec<ParsedMove>), AppError> {
    extractor::parse_pgn(pgn)
}

fn handle_bot_move() {
    todo!()
}
