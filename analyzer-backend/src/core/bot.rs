use crate::core::extractor::parse_fen;
use crate::core::models::{AppState, Eval, Move};
use crate::errors::AppError;

// pub async fn bot_move(state: AppState, request: ChessBotRequestEvent) -> ChessBotResponseEvent {
//     let fen = match parse_fen(&request.fen) {
//         Ok(fen) => fen,
//         Err(e) => {
//             return ChessBotResponseEvent::Error {
//                 game_id: request.game_id,
//                 version: request.version,
//                 error: e.to_string(),
//             };
//         }
//     };
//     let mut analyzer = state.lock().await;
//     let response = match analyzer.get_move_from_fen(fen) {
//         Ok((eng_move, eval)) => ChessBotResponseEvent::Ok {
//             game_id: request.game_id,
//             version: request.version,
//             uci: eng_move,
//             eval,
//         },
//         Err(error) => ChessBotResponseEvent::Error {
//             game_id: request.game_id,
//             version: request.version,
//             error: error.to_string(),
//         },
//     };
//     response
// }
/// Вычисляет лучший ход для бота и возвращает оценку позиции на основе строки FEN.
///
/// # Аргументы
///
/// * `state` - Общее состояние приложения (`Arc<Mutex<Analyzer>>`).
/// * `fen` - Позиция на доске в нотации Форсайта - Эдвардса.
///
/// # Возвращаемое значение
///
/// Кортеж `(Move, Eval)`, где `Move` - это псевдоним типа `String` для хода,
/// а [`Eval`] - оценка позиции.
///
/// # Ошибки
///
/// Функция вернёт `AppError` в следующих случаях:
/// * Передана невалидная строка `fen`.
/// * Для текущей позиции нет доступных ходов.
/// * Произошла внутренняя ошибка шахматного движка или библиотеки-обёртки.
pub async fn bot_move(state: AppState, fen: String) -> Result<(Move, Eval), AppError> {
    let fen = parse_fen(&fen)?;
    let mut analyzer = state.lock().await;
    Ok(analyzer.get_move_from_fen(fen)?)
}
