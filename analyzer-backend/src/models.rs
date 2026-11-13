use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChessRequestEvent {
    pub game_id: String,
    pub version: u64,
    pub fen: String, // снапшот всей доски
    pub color: Color,
    pub history: Vec<String>, // история ходов в UCI формате
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChessResponseEvent {
    Ok {
        game_id: String,
        version: u64,
        uci: String,
        eval_cp: i32,
    },
    Error {
        game_id: String,
        version: u64,
        error: String,
    },
}
