// use axum::{Router, routing::get};
mod analyzer;
mod errors;
mod extractor;
mod models;
use crate::errors::AppError;
use crate::models::{MoveAnalysis, ParsedMove};
use analyzer::Analyzer;
use axum::Json;
pub async fn handle_analyze_full_game(body: String) -> Result<Json<Vec<MoveAnalysis>>, AppError> {
    let moves = handle_parse_pgn(&body)?;
    let mut analyzer = Analyzer::new("./bin/stockfish-linux-x86-64-universal").map_err(|_| {
        AppError::EngineError("Не удалось найти исполняемый файл движка".to_string())
    })?;
    let analyzed_moves = analyzer.analyze_from_uci(moves)?;
    Ok(Json(analyzed_moves))
}
fn handle_parse_pgn(pgn: &str) -> Result<Vec<ParsedMove>, AppError> {
    if let Some((_, moves)) = crate::extractor::parse_pgn(pgn) {
        Ok(moves)
    } else {
        println!("Ошибка парсинга");
        return Err(AppError::BadPgn("Некоректный PGN".to_string()));
    }
}
