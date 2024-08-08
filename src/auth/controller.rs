use crate::AppState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use super::{service, AuthBody, AuthError, AuthPayload, Claims, KEYS};

pub fn generate_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/protected", get(protected_handler))
}

#[axum::debug_handler]
pub async fn login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    service::verify_user(&app_state.pool, &payload)
        .await
        .map_err(|_| AuthError::IncorrectCredentials)?;

    let claims = Claims {
        email: payload.email,
        exp: jsonwebtoken::get_current_timestamp() + 3600 * 1000,
    };

    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

#[axum::debug_handler]
pub async fn protected_handler(claims: Claims) -> Result<Json<String>, AuthError> {
    Ok(Json(format!("Hello, {}!", claims.email)))
}
