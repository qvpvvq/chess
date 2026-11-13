use crate::Analyzer;
use crate::models::{ChessRequestEvent, ChessResponseEvent};
pub fn normalize_data(request: ChessRequestEvent, analyzer: &mut Analyzer) -> ChessResponseEvent {
    match analyzer.get_move_from_fen(request.fen) {
        Ok((eng_move, eval)) => ChessResponseEvent::Ok {
            game_id: request.game_id,
            version: request.version,
            uci: eng_move,
            eval_cp: eval,
        },
        Err(error) => ChessResponseEvent::Error {
            game_id: request.game_id,
            version: request.version,
            error,
        },
    }
}
