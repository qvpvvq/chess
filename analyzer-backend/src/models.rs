use serde::Serialize;

#[derive(Serialize)]
pub struct MoveAnalysis {
    pub ply: usize,
    pub chessmove: ParsedMove,
    pub best_move: String,
    pub eval: i32,
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
