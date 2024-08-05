use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

pub fn success<TData: Serialize>(data: TData) -> Json<TData> {
    Json(data)
}

pub fn failed() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
pub fn failed_with_message(message: String) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn failed_with_code(message: String, status_code: StatusCode) -> (StatusCode, String) {
    (status_code, message)
}
