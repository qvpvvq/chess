use stockfish::Stockfish;

use crate::{
    errors::AppError,
    models::{MoveAnalysis, ParsedMove},
};
pub struct Analyzer {
    engine: Stockfish,
}
impl Analyzer {
    pub fn new(path_to_engine: &str) -> Result<Self, AppError> {
        let mut engine = Stockfish::new(path_to_engine).map_err(|_| {
            AppError::EngineError("Не удалось создать экземпляр движка".to_string())
        })?;
        engine.set_depth(15);
        engine.set_threads(1).map_err(|_| {
            AppError::EngineError("Не удалось создать экземпляр движка".to_string())
        })?;
        Ok(Self { engine })
    }
    pub fn analyze_from_uci(
        &mut self,
        moves: Vec<ParsedMove>,
    ) -> Result<Vec<MoveAnalysis>, AppError> {
        let mut results: Vec<MoveAnalysis> = Vec::new();
        self.engine
            .reset_position()
            .map_err(|_| AppError::EngineError("Не удалось сбросить позицию".to_string()))?;
        for (i, mv) in moves.into_iter().enumerate() {
            let uci_move = mv.move_uci;
            let engine_output = self.engine.go().map_err(|_| {
                AppError::EngineError("Движок не может сейчас проанализировать партию".to_string())
            })?;
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
                (result.ply + 1) / 2,
                result.ply,
                result.chessmove.move_uci,
                result.best_move,
                result.eval
            );
            results.push(result);
        }
        Ok(results)
    }
    pub fn get_move_from_fen(&mut self, fen: String) -> Result<(String, i32), String> {
        self.engine
            .set_fen_position(&fen)
            .map_err(|e| e.to_string())?;
        let engine_move = self.engine.go().map_err(|e| e.to_string())?;
        let (best_move, eval) = (
            engine_move.best_move().to_owned(),
            engine_move.eval().value(),
        );
        Ok((best_move, eval))
    }
}
