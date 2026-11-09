// use axum::{Router, routing::get};
mod analyzer;
mod extractor;
mod models;
use analyzer::Analyzer;

use crate::models::ParsedMove;
fn main() {
    let pgn = r#"
      [Event "Live Chess"]
      [Site "Chess.com"]
      [Date "2026.11.08"]
      [Round "-"]
      [White "nomercurial"]
      [Black "Khalil_Ghimaji"]
      [Result "1-0"]
      [CurrentPosition "rn3rk1/p1p2ppQ/bp1bpq1n/6N1/3P4/P7/1PP2PPP/R1B1KB1R b KQ - 0 12"]
      [Timezone "UTC"]
      [ECO "D00"]
      [ECOUrl "https://www.chess.com/openings/Queens-Pawn-Opening-Chigorin-Variation-2...e6"]
      [UTCDate "2026.11.08"]
      [UTCTime "00:08:30"]
      [WhiteElo "409"]
      [BlackElo "387"]
      [TimeControl "600"]
      [Termination "nomercurial won by checkmate"]
      [StartTime "00:08:30"]
      [EndDate "2026.11.08"]
      [EndTime "00:14:23"]
      [Link "https://www.chess.com/game/live/186751366954"]

      1. d4 {[%clk 0:09:57.1]} 1... e6 {[%clk 0:09:49.9]} 2. Nc3 {[%clk 0:09:54]} 2... d5 {[%clk 0:09:47.3]} 3. a3 {[%clk 0:09:51.9]} 3... Qf6 {[%clk 0:09:26.3]} 4. Nf3 {[%clk 0:09:46.5]} 4... Nh6 {[%clk 0:09:19.9]} 5. Ne5 {[%clk 0:09:45.6]} 5... Bd6 {[%clk 0:08:46.1]} 6. Nf3 {[%clk 0:09:20.3]} 6... O-O {[%clk 0:08:26.6]} 7. e4 {[%clk 0:09:13]} 7... dxe4 {[%clk 0:07:43]} 8. Nxe4 {[%clk 0:09:10.4]} 8... Qf5 {[%clk 0:07:07.6]} 9. Qd3 {[%clk 0:08:54.6]} 9... b6 {[%clk 0:06:49.8]} 10. Nfg5 {[%clk 0:08:46.9]} 10... Ba6 {[%clk 0:06:00.1]} 11. Nf6+ {[%clk 0:08:31]} 11... Qxf6 {[%clk 0:05:46.7]} 12. Qxh7# {[%clk 0:08:24.5]} 1-0

  "#;

    let uci = sraka(pgn).unwrap();
    let analyzer = Analyzer::new("./bin/stockfish-windows-x86-64-universal").unwrap();
    analyzer.analyze_from_uci(uci).unwrap();
}
fn sraka(pgn: &str) -> Result<Vec<ParsedMove>, String> {
    if let Some((meta, moves)) = crate::extractor::parse_pgn(pgn) {
        println!("Белые: {:?}", meta.white.unwrap());
        println!("Черные: {:?}", meta.black.unwrap());
        println!(
            "Ходы в UCI: {:?}",
            moves.iter().map(|m| &m.move_uci).collect::<Vec<_>>()
        );
        Ok(moves)
    } else {
        println!("Ошибка парсинга");
        return Err("Кривой pgn вы отправили к сожалению!".to_string());
    }
}
