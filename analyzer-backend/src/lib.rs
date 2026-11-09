// use axum::{Router, routing::get};
mod analyzer;
mod extractor;
mod models;
use crate::models::{MoveAnalysis, ParsedMove};
use analyzer::Analyzer;
use axum::Json;
use axum::http::StatusCode;
fn analyze_by_pgn(body: &String) -> Result<Vec<MoveAnalysis>, String> {
    let moves = sraka(&body).map_err(|e| e.to_string())?;
    let mut analyzer =
        Analyzer::new("./bin/stockfish-windows-x86-64-universal").map_err(|e| e.to_string())?;
    let analyzed_moves = analyzer.analyze_from_uci(moves).unwrap();
    Ok(analyzed_moves)
}
fn sraka(pgn: &str) -> Result<Vec<ParsedMove>, String> {
    if let Some((_, moves)) = crate::extractor::parse_pgn(pgn) {
        Ok(moves)
    } else {
        println!("Ошибка парсинга");
        return Err("Кривой pgn вы отправили к сожалению!".to_string());
    }
}
pub async fn handle_analys_full(
    body: String,
) -> Result<Json<Vec<MoveAnalysis>>, (StatusCode, String)> {
    match analyze_by_pgn(&body) {
        Ok(analysis) => Ok(Json(analysis)), // <--- Возвращаем JSON со структурой
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}
