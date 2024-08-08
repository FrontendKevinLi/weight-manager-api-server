use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;
use axum::routing::post;
use axum::{http::StatusCode, Json, Router};

use super::{AuthBody, AuthPayload, Claims, KEYS};

pub fn generate_router() -> Router<AppState> {
    Router::new().route("/login", post(login_handler))
}

#[axum::debug_handler]
pub async fn login_handler(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, StatusCode> {
    if payload.email.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // TODO: add email and password database validation

    // TODO: change to internal server error
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time must be present!");

    let claims = Claims {
        email: payload.email,
        exp: now.as_millis(),
    };

    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthBody::new(token)))
}
