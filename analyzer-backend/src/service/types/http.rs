use crate::core::models::{Eval, Move};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpBotRequest {
    pub fen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct HttpBotResponse {
    uci_move: Move,
    eval: Eval,
}
impl HttpBotResponse {
    pub fn new(uci_move: Move, eval: Eval) -> Self {
        Self { uci_move, eval }
    }
}
