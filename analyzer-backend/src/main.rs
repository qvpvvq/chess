mod core;
mod errors;
mod service;

use axum::{Router, routing::post};
use core::Analyzer;
use service::http::{handle_analyze_full_game, handle_bot_move};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::service::nats::run_nats;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    dotenvy::dotenv().ok();
    let engine_base_name = env::var("STOCKFISH_NAME").expect("Ошибка связанная с env");
    let engine_filename = format!("{}{}", engine_base_name, env::consts::EXE_SUFFIX);
    let engine_path = Arc::new(format!("./bin/{}", engine_filename));
    let analyzer = Analyzer::new(&engine_path).expect("Не удалось запустить Stockfish");
    let state: Arc<Mutex<Analyzer>> = Arc::new(Mutex::new(analyzer));
    run_nats(Arc::clone(&state)).await?;
    let app = Router::new()
        .route("/analyze_game", post(handle_analyze_full_game))
        .route("/fen", post(handle_bot_move))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
