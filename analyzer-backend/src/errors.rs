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
    InvalidNatsRequest(String),
    SerializationError(String), // битый JSON, неподдерживаемая тема: повтор не поможет
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match self {
            Self::BadPgn(msg) | Self::BadFen(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::EngineError(msg) | Self::SerializationError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            Self::InvalidNatsRequest(_) => {
                tracing::error!("InvalidNatsRequest дошёл до HTTP-слоя (баг)");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal error".to_string(),
                )
            }
        };
        let body = Json(json!({"error": error_msg, "status": status.as_u16()}));
        (status, body).into_response()
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadFen(msg)
            | AppError::BadPgn(msg)
            | AppError::EngineError(msg)
            | AppError::InvalidNatsRequest(msg)
            | AppError::SerializationError(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}
