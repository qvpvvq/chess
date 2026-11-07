use std::ops::ControlFlow;

use pgn_reader::{Reader, SanPlus, Visitor};
use shakmaty::{Chess, Position, uci::UciMove};
#[derive(Default)]
pub struct GameMetadata {
    pub white: Option<String>,
    pub black: Option<String>,
    pub result: Option<String>,
    pub event: Option<String>,
}

pub struct MovesStorage {
    meta: GameMetadata,
    moves_uci: Vec<String>,
    pos: Chess,
}

impl Visitor for MovesStorage {
    type Tags = GameMetadata;
    type Movetext = MovesStorage;
    type Output = (GameMetadata, Vec<String>);
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
            moves_uci: Vec::new(),
        })
    }
    // В конце партии возвращает теги и ходы в формате UCI
    fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
        (movetext.meta, movetext.moves_uci)
    }
    // Обработка каждого хода
    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        // Виртуальная доска (атрибут pos) ходит на основе SAN хода
        if let Ok(mv) = san_plus.san.to_move(&movetext.pos) {
            let uci_move = UciMove::from_standard(mv); // Ход перегоняется в UCI
            movetext.moves_uci.push(uci_move.to_string()); // Докидывается к "списку" ходов в структуре
            movetext.pos.play_unchecked(mv); // Обновление доски
        }
        ControlFlow::Continue(())
    }
}
pub fn parse_pgn(pgn_str: &str) -> Option<(GameMetadata, Vec<String>)> {
    let mut reader = Reader::new(pgn_str.as_bytes());
    let mut visitor = MovesStorage {
        meta: GameMetadata::default(),
        pos: Chess::default(),
        moves_uci: Vec::new(),
    };
    reader.read_game(&mut visitor).unwrap_or(None)
}
