use analys_backend::analyzer::Analyzer;
use analys_backend::handle_analyze_full_game;
use axum::{Router, routing::post};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let engine_base_name = env::var("STOCKFISH_NAME").expect("Ошибка связанная с env");
    let engine_filename = format!("{}{}", engine_base_name, env::consts::EXE_SUFFIX);
    let engine_path = Arc::new(format!("./bin/{}", engine_filename));
    let analyzer = Analyzer::new(&engine_path).expect("Не удалось запустить Stockfish");
    let state = Arc::new(Mutex::new(analyzer));
    let app = Router::new()
        .route("/", post(handle_analyze_full_game))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
