use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;
#[derive(Debug)]
pub enum AppError {
    BadPgn(String),      // Невалидный PGN (Вина клиента)
    BadFen(String),      // Невалидный FEN (вина клиента)
    EngineError(String), // Ошибка Стокфиша (вина сервера)
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match self {
            AppError::BadPgn(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::BadFen(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::EngineError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({"error": error_msg, "status": status.as_u16()}));
        (status, body).into_response()
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadFen(msg) | AppError::BadPgn(msg) | AppError::EngineError(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}
