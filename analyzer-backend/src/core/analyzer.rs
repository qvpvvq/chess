use stockfish::{EvalType, Stockfish};

use crate::{
    core::models::{Eval, Move, MoveAnalysis, ParsedMove},
    errors::AppError,
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
            let engine_output = self.engine.go().map_err(|e| {
                AppError::EngineError(format!("Движку не удалось проанализировать ход: {e}"))
            })?;
            self.engine
                .play_move(&uci_move)
                .map_err(|e| AppError::EngineError(format!("Ошибка движка: {e}")))?;

            println!("Сходило");

            let best_move = engine_output.best_move().to_string();
            let move_rate = engine_output.eval().value();
            let eval = match engine_output.eval().eval_type() {
                EvalType::Centipawn => Eval::Cp(move_rate),
                EvalType::Mate => Eval::Mate(move_rate),
            };

            let result = MoveAnalysis {
                ply: i + 1,
                chessmove: ParsedMove::new(uci_move, mv.move_san),
                best_move,
                eval,
            };

            println!(
                " {} ход, {} полуход: {} | Лучший ход: {} | Оценка позиции белых: {:?}",
                result.ply.div_ceil(2),
                result.ply,
                result.chessmove.move_uci,
                result.best_move,
                result.eval
            );
            results.push(result);
        }
        Ok(results)
    }

    pub fn get_move_from_fen(&mut self, fen: String) -> Result<(Move, Eval), AppError> {
        self.engine
            .set_fen_position(&fen)
            .map_err(|e| AppError::EngineError(e.to_string()))?;
        let engine_move = self
            .engine
            .go()
            .map_err(|e| AppError::EngineError(e.to_string()))?;
        let best_move = engine_move.best_move().to_owned();

        let eval_value = engine_move.eval().value();
        let eval: Eval = match &engine_move.eval().eval_type() {
            EvalType::Centipawn => Eval::Cp(eval_value),
            EvalType::Mate => Eval::Mate(eval_value),
        };
        Ok((best_move, eval))
    }
}
