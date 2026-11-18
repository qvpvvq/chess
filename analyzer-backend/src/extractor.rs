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
