use crate::core::models::Eval;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChessBotRequestEvent {
    #[serde(rename = "gameId")]
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
#[serde(untagged)]
pub enum ChessBotResponseEvent {
    #[serde(rename_all = "camelCase")]
    Ok {
        game_id: String,
        version: u64,
        uci: String,
        eval: Eval,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        game_id: String,
        version: u64,
        error: String,
    },
}
