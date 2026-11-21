use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MoveAnalysis {
    pub ply: usize,
    pub chessmove: ParsedMove,
    pub best_move: String,
    pub eval: Eval,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum Eval {
    Mate(i32),
    Cp(i32),
}
