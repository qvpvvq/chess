use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
#[derive(Debug)]
pub enum AppError {
    BadPgn(String),      // Ошибка парсинга (вина клиента)
    EngineError(String), // Ошибка Стокфиша (вина сервера)
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match self {
            AppError::BadPgn(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::EngineError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({"error": error_msg, "status": status.as_u16()}));
        (status, body).into_response()
    }
}
