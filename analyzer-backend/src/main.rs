use axum::{Router, routing::get};
mod analyzer;
mod extractor;

fn main() {
    let pgn = r#"
  [Event "Play vs Bot"]
  [Site "Chess.com"]
  [Date "2026.11.04"]
  [Round "?"]
  [White "Luna-BOT"]
  [Black "aza2sosal"]
  [Result "1-0"]
  [TimeControl "?"]
  [WhiteElo "1000"]
  [BlackElo "1600"]
  [Termination "Luna поставил мат"]
  [ECO "D02"]
  [EndDate "2026.11.04"]
  [Link "https://www.chess.com/game/computer/2524198596?move=0"]

  1. Nf3 d5 2. d4 Nc6 3. Bf4 Be6 4. Qd2 Nf6 5. Ng5 h6 6. Nxe6 fxe6 7. c4 dxc4 8.
  Be5 Ng4 9. Nc3 Ncxe5 10. Qd1 Nc6 11. e3 g6 12. Qxg4 g5 13. Qh5+ Kd7 14. Bxc4 Na5
  15. Bd3 Bg7 16. Bf1 Rf8 17. Qd1 Ke8 18. Qh5+ Rf7 19. Be2 Nc6 20. h3 Nb4 21. O-O
  Nd5 22. Rab1 Nxc3 23. bxc3 Kf8 24. Qg4 Qd7 25. Bd3 Qa4 26. Rxb7 e5 27. Rxc7 exd4
  28. cxd4 Qa5 29. Qc8+ Rxc8 30. Rxc8+ Qd8 31. Rxd8# 1-0"#;

    if let Some((meta, moves)) = crate::extractor::parse_pgn(pgn) {
        println!("Белые: {:?}", meta.white.unwrap());
        println!("Черные: {:?}", meta.black.unwrap());
        println!("Ходы в UCI: {:?}", moves);
    } else {
        println!("Ошибка парсинга");
    }
}
