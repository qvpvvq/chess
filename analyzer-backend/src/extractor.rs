use crate::{errors::AppError, models::ParsedMove};
use pgn_reader::{Reader, SanPlus, Visitor};
use shakmaty::{Chess, Position, uci::UciMove};
use std::ops::ControlFlow;
#[derive(Default, Debug)]
pub struct GameMetadata {
    pub white: Option<String>,
    pub black: Option<String>,
    pub result: Option<String>,
    pub event: Option<String>,
}
pub struct MovesStorage {
    meta: GameMetadata,
    moves: Vec<ParsedMove>,
    pos: Chess,
}

impl Visitor for MovesStorage {
    type Tags = GameMetadata;
    type Movetext = MovesStorage;
    type Output = Result<(GameMetadata, Vec<ParsedMove>), String>;
    // В начале партии теги ставятся дефолтные, то есть None
    fn begin_tags(&mut self) -> std::ops::ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(GameMetadata::default())
    }
    // Парсинг тегов PGN
    fn tag(
        &mut self,
        tags: &mut Self::Tags, // Tags в начале переназначил тип, теперь это типо tags: &mut GameMetadata
        name: &[u8],
        value: pgn_reader::RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        let tag_str = std::str::from_utf8(name).ok(); // name - само типо название тега
        let value_str = value.decode_utf8_lossy();
        match tag_str {
            Some("White") => tags.white = Some(value_str.to_string()),
            Some("Black") => tags.black = Some(value_str.to_string()),
            Some("Result") => tags.result = Some(value_str.to_string()),
            Some("Event") => tags.event = Some(value_str.to_string()),
            _ => {}
        }
        ControlFlow::Continue(())
    }
    // Когда парсер заканчивает читать теги и начинает анализировать ходы, то возвращает теги и ПРИСТУПАЕТ к анализу ходов
    fn begin_movetext(&mut self, tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(MovesStorage {
            meta: tags,
            pos: Chess::default(),
            moves: Vec::new(),
        })
    }
    // В конце партии возвращает теги и ходы в формате UCI
    fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
        if movetext.moves.is_empty() {
            return Err("PGN не содержит ходов".to_string());
        }
        Ok((movetext.meta, movetext.moves))
    }
    // Обработка каждого хода
    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        let mv = match san_plus.san.to_move(&movetext.pos) {
            Ok(mv) => mv,
            Err(e) => {
                return ControlFlow::Break(Err(format!(
                    "Некорректный ход {}: {}",
                    san_plus.san, e
                )));
            }
        };
        let uci_move = UciMove::from_standard(mv);
        movetext.moves.push(ParsedMove::new(
            uci_move.to_string(),
            san_plus.san.to_string(),
        ));
        movetext.pos.play_unchecked(mv);
        ControlFlow::Continue(())
    }
}
pub fn parse_pgn(pgn_str: &str) -> Result<(GameMetadata, Vec<ParsedMove>), AppError> {
    let mut reader = Reader::new(pgn_str.as_bytes());
    let mut visitor = MovesStorage {
        meta: GameMetadata::default(),
        pos: Chess::default(),
        moves: Vec::new(),
    };
    reader
        .read_game(&mut visitor)
        .map_err(|e| AppError::ParserError(format!("Ошибка парсинга PGN: {e}")))?
        .ok_or_else(|| AppError::ParserError("PGN не содержит игры".to_string()))?
        .map_err(|e| AppError::ParserError(format!("Некорректная партия: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pgn() {
        let pgn = r#"
        [Event "Live Chess"]
        [Site "Chess.com"]
        [Date "2026.11.19"]
        [Round "?"]
        [White "aza2sosal"]
        [Black "AriiiiAdnan"]
        [Result "1-0"]
        [TimeControl "600"]
        [WhiteElo "388"]
        [BlackElo "393"]
        [Termination "aza2sosal победил - соперник покинул партию"]
        [Link "https://www.chess.com/game/187479872116"]

        1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Bxc6 bxc6 5. Nxe5 d6 6. Nxc6 Qd7 7. Nd4 Qe7 8.
        d3 Bh3 9. Qh5 Bxg2 10. Nf5 Qe5 11. Qg5 Bxh1 12. Bf4 Qxb2 13. Nd2 Qxa1+ 14. Ke2
        h6 15. Qh5 g6 16. Qh3 gxf5 17. exf5 Nf6 18. Qf1 Qxa2 19. Qxh1 Bg7 20. Qxa8+ Ke7
        21. Qc6 Ng4 22. f3 Nxh2 23. Bxh2 h5 24. Qxc7+ Kf6 25. Qxd6+ Kg5 26. Bf4+ Kh4 27.
        Qe7+ Kh3 28. f6 Bxf6 29. Qxf6 Rg8 30. Nc4 Qxc2+ 31. Ke3 Re8+ 32. Ne5 Qc5+ 33.
        Ke4 a5 34. Qf5+ Kg2 35. Qxf7 Rd8 36. Qxh5 a4 37. d4 Qc2+ 38. Ke3 a3 39. Qf7 a2
        40. Qa7 Qc1+ 41. Ke4 a1=Q 42. Qg7+ Kf2 43. Nd3+ Ke2 44. Nxc1+ Qxc1 45. Qg2+ Kd1
        46. Bxc1 Kxc1 47. Qg5+ Kc2 48. Qxd8 Kc3 49. Qc7+ Kd2 50. f4 Ke2 51. f5 1-0
        "#;
        let result = parse_pgn(pgn);
        assert!(result.is_ok());
    }
    #[test]
    fn test_empty_pgn() {
        let pgn = "";
        let result = parse_pgn(pgn);
        assert!(result.is_err())
    }
    #[test]
    fn test_illegal_pgn() {
        let pgn = r#"
        [BlackElo "393"]
        [Termination "aza2sosal победил - соперник покинул партию"]
        1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Bxc6 bxc6 5. Nxe5 d6 6. Nxc6 Qd7 7. Nd4 Qe7 8.
        d3 Bh3 9. Qh5 Bxg2 10. Nf5 Qe5 11. Qg5 Bxh1 12. Bf4 Qxb2 13. Nd2 Qxa1+ 14. Ke2
        h6 15. Qh5 g6 16. Qh3 gxf5 17. exf5 Nf6 18. Qf1 Qxa2 19. Qxh1 Bg7 20. Qxa8+ Ke7
        21. Qc6 Ng4 22. f3 Nxh2 23. Bxh2 h5 24. Qxc7+ Kf6 25. Qxd6+ Kg5 26. Bf4+ Kh4 27.
        Qe7+ Kh3 33.
        Ke4 a5 34. Qf5+ Kg2 35. Qxf7 Rd8 36. Qxh5 a4 37. d4 Qc2+ 38. Ke3 a3 39. Qf7 a2
        40. Qa7 Qc1+ 41. Ke4 a1=Q 42. Qg7+ Kf2 43. Nd3+ Ke2 44. Nxc1+ Qxc1 45. Qg2+ Kd1
        46. Bxc1 Kxc1 47. Qg5+ Kc2 48. Qxd8 Kc3 49. Qc7+ Kd2 50. f4 Ke2 51. f5 1-0
        "#; // ВЫРЕЗАНО ВСЕ С 28 ДО 33 ХОДА
        let result = parse_pgn(pgn);
        assert!(result.is_err())
    }
    #[test]
    fn test_no_moves_pgn() {
        let pgn = r#"
    [Event "Live Chess"]
    [Site "Chess.com"]
    [Date "2026.11.19"]
    [Round "?"]
    [White "aza2sosal"]
    [Black "AriiiiAdnan"]
    [Result "1-0"]
    [TimeControl "600"]
    [WhiteElo "388"]
    [BlackElo "393"]
    [Termination "aza2sosal победил - соперник покинул партию"]
    [Link "https://www.chess.com/game/187479872116"]
    "#;
        let result = parse_pgn(pgn);
        assert!(result.is_err())
    }
}
