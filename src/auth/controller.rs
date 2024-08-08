use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;
use axum::routing::{get, post};
use axum::{Json, Router};

use super::{AuthBody, AuthError, AuthPayload, Claims, KEYS};

pub fn generate_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/protected", get(protected_handler))
}

#[axum::debug_handler]
pub async fn login_handler(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
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
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

#[axum::debug_handler]
pub async fn protected_handler(claims: Claims) -> Result<Json<String>, AuthError> {
    Ok(Json(format!("Hello, {}!", claims.email)))
}
