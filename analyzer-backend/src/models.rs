use serde::Serialize;

#[derive(Serialize)]
pub struct MoveAnalysis {
    pub ply: usize,            // 1, 2, 3...
    pub chessmove: ParsedMove, // Ход, который был (пока можешь писать UCI)
    pub best_move: String,     // Лучший ход от Стокфиша
    pub eval: i32,             // Оценка (например, 35)
}
#[derive(Serialize, Debug)]
pub struct ParsedMove {
    pub move_uci: String,
    pub move_san: String,
}
impl ParsedMove {
    pub fn new(move_uci: String, move_san: String) -> Self {
        Self { move_uci, move_san }
    }
}
