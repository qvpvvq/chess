use crate::core::bot::bot_move;
use crate::core::extractor::parse_pgn;
use crate::core::models::{AppState, MoveAnalysis};
use crate::errors::AppError;
use crate::service::types::http::{HttpBotRequest, HttpBotResponse};
use axum::Json;
use axum::extract::State;

pub async fn handle_analyze_full_game(
    State(state): State<AppState>,
    body: String,
) -> Result<Json<Vec<MoveAnalysis>>, AppError> {
    println!("Ok: Запрос дошел");
    let (metadata, moves) = parse_pgn(&body)?;
    println!("Ok: Анализатор занят для анализа партии");
    let mut analyzer = state.lock().await;
    println!("{:#?} {:?}", metadata, moves);
    let analyzed_moves = analyzer.analyze_from_uci(moves)?;
    Ok(Json(analyzed_moves))
}

pub async fn handle_bot_move(
    State(state): State<AppState>,
    Json(request): Json<HttpBotRequest>,
) -> Result<Json<(HttpBotResponse)>, AppError> {
    let (bot_move, eval) = bot_move(state, request.fen).await?;
    Ok(Json((HttpBotResponse::new(bot_move, eval))))
}
