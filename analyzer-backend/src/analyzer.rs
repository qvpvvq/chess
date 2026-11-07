use pgn_reader::Reader;
use std::io::Result;
use stockfish::Stockfish;
struct Analyzer {
    engine: Stockfish,
}
impl Analyzer {
    pub fn new(path_to_engine: &str) -> Result<Self> {
        let mut engine = Stockfish::new(path_to_engine)?;
        engine.set_depth(15);
        engine.set_threads(1)?;
        Ok(Self { engine })
    }
}
