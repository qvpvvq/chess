use crate::core::Analyzer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Move = String;
pub type AppState = Arc<Mutex<Analyzer>>;
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
/// Оценка шахматной позиции, возвращаемая движком.
///
/// Возможные варианты [`Eval::Cp`], [`Eval::Mate`]
pub enum Eval {
    /// Форсированный мат через указанное количество ходов
    ///
    /// Положительное число - мат черным
    /// Отрицательное число - мат белым.
    Mate(i32),
    /// Оценка в сантипешках (например, `150` означает перевес белых в 1.5 пешки).
    Cp(i32),
}
