use std::io::Result;
use stockfish::Stockfish;

use crate::models::{MoveAnalysis, ParsedMove};
pub struct Analyzer {
    engine: Stockfish,
}
impl Analyzer {
    pub fn new(path_to_engine: &str) -> Result<Self> {
        let mut engine = Stockfish::new(path_to_engine)?;
        engine.set_depth(15);
        engine.set_threads(1)?;
        Ok(Self { engine })
    }
    pub fn analyze_from_uci(
        mut self,
        moves: Vec<ParsedMove>,
    ) -> std::io::Result<Vec<MoveAnalysis>> {
        let mut results: Vec<MoveAnalysis> = Vec::new();
        self.engine.reset_position()?;
        for (i, mv) in moves.into_iter().enumerate() {
            let uci_move = mv.move_uci;
            let engine_output = self.engine.go()?;
            if let Ok(_) = self.engine.play_move(&uci_move) {
                println!("Сходило");
            };
            let best_move = engine_output.best_move().to_string();
            let move_rate = engine_output.eval().value();
            let result = MoveAnalysis {
                ply: i + 1,
                chessmove: ParsedMove::new(uci_move, mv.move_san),
                best_move: best_move,
                eval: move_rate,
            };

            println!(
                " {} ход, {} полуход: {} | Лучший ход: {} | Оценка позиции белых: {}",
                (&result.ply + 2) / 2,
                &result.ply,
                &result.chessmove.move_uci,
                &result.best_move,
                &result.eval
            );
            results.push(result);
        }
        Ok(results)
    }
}
