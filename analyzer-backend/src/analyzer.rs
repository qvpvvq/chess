use std::io::Result;
use stockfish::Stockfish;
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
    pub fn analyze_from_uci(mut self, moves: Vec<String>) -> std::io::Result<()> {
        self.engine.reset_position()?;
        for (i, mv) in moves.iter().enumerate() {
            let is_white_turn = i % 2 == 0;
            let engine_output = self.engine.go()?;
            let best_move = engine_output.best_move();
            let move_rate = engine_output.eval().value();

            println!(
                " {} ход, {} полуход: {} | Лучший ход: {} | Оценка позиции белых: {}",
                (i + 2) / 2,
                i + 1,
                mv,
                best_move,
                move_rate
            );
            self.engine.play_move(mv)?;
        }
        Ok(())
    }
}
