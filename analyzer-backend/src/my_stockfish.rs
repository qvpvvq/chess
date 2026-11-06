use crate::AppState;
use axum::extract::State;
// TEST FN
pub async fn show_position(State(state): State<AppState>) -> String {
    let mut stockfish = state.lock().unwrap();

    stockfish.reset_position().unwrap();
    let board = stockfish.get_board_display().unwrap();
    let fen_pos = stockfish.get_fen().unwrap();

    let board = format!("{}\n{}", board, fen_pos);
    board
}
pub async fn get_best_move(State(state): State<AppState>) -> String {
    let mut stockfish = state.lock().unwrap();
    let best_move = stockfish.go().unwrap().best_move().to_owned();
    best_move
}
