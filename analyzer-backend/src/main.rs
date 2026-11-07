// use axum::{Router, routing::get};
mod analyzer;
mod extractor;
use analyzer::Analyzer;
fn main() {
    let pgn = r#"
        [Event "Live Chess"]
        [Site "Chess.com"]
        [Date "2026.11.07"]
        [Round "-"]
        [White "chuxan228"]
        [Black "nomercurial"]
        [Result "0-1"]
        [CurrentPosition "rn6/pp4p1/2p3k1/8/4K3/8/PP4Pr/5R2 w - - 2 30"]
        [Timezone "UTC"]
        [ECO "B02"]
        [ECOUrl "https://www.chess.com/openings/Alekhines-Defense-Scandinavian-Variation"]
        [UTCDate "2026.11.07"]
        [UTCTime "18:58:10"]
        [WhiteElo "309"]
        [BlackElo "328"]
        [TimeControl "300"]
        [Termination "nomercurial won on time"]
        [StartTime "18:58:10"]
        [EndDate "2026.11.07"]
        [EndTime "19:07:14"]
        [Link "https://www.chess.com/ru/analysis/game/live/186737219660/review?move=53&tab=review&classification=greatfind&autorun=true&utm_campaign=game_review_share"]
        [WhiteUrl "https://www.chess.com/bundles/web/images/noavatar_l.84a92436.gif"]
        [WhiteCountry "116"]
        [WhiteTitle ""]
        [BlackUrl "https://www.chess.com/bundles/web/images/noavatar_l.84a92436.gif"]
        [BlackCountry "116"]
        [BlackTitle ""]

        1. e4 d5 2. Nc3 Nf6 3. d3 Bg4 4. f3 dxe4 5. dxe4 Bh5 6. Nd5 Bxf3 7. Nxf3 Nxe4 8.
        Bd3 e6 9. Bxe4 exd5 10. Bxd5 Bb4+ 11. c3 c6 12. Bg5 Qa5 13. cxb4 Qxb4+ 14. Qd2
        Qc5 15. Be4 O-O 16. Rc1 Qb5 17. Ne5 Re8 18. Rf1 Qxe5 19. Rxf7 Qxe4+ 20. Be3 Kxf7
        21. Kd1 Qa4+ 22. Qc2 Qg4+ 23. Qe2 Rxe3 24. Qxg4 Rd3+ 25. Ke2 Rd5 26. Ke3 h5 27.
        Qxh5+ Rxh5 $1 28. Ke4 Rxh2 29. Rf1+ Kg6 0-1
  "#;

    let uci = sraka(pgn).unwrap();
    let analyzer = Analyzer::new("./bin/stockfish-linux-x86-64-universal").unwrap();
    analyzer.analyze_from_uci(uci).unwrap();
}
fn sraka(pgn: &str) -> Result<Vec<String>, String> {
    if let Some((meta, moves)) = crate::extractor::parse_pgn(pgn) {
        println!("Белые: {:?}", meta.white.unwrap());
        println!("Черные: {:?}", meta.black.unwrap());
        println!("Ходы в UCI: {:?}", moves);
        Ok(moves)
    } else {
        println!("Ошибка парсинга");
        return Err("Кривой pgn вы отправили к сожалению!".to_string());
    }
}
