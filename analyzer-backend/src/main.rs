use axum::{Router, routing::get};
mod my_stockfish;
use std::sync::{Arc, Mutex};
use stockfish::Stockfish;
type AppState = Arc<Mutex<Stockfish>>;
#[tokio::main]
async fn main() {
    let stockfissh = Stockfish::new("./bin/stockfish-windows-x86-64-universal.exe").unwrap();
    let state: AppState = Arc::new(Mutex::new(stockfissh));
    let app = Router::new()
        .route("/showpos", get(my_stockfish::show_position))
        .route("/bestmove", get(my_stockfish::get_best_move))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:666").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
