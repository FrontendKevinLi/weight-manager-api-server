use axum::{
    async_trait, extract::FromRequestParts, http::StatusCode, response::IntoResponse,
    RequestPartsExt,
};
use axum_extra::headers::authorization::Bearer;
use axum_extra::{headers::Authorization, TypedHeader};
use jsonwebtoken::{DecodingKey, EncodingKey, TokenData};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

mod controller;
mod service;
pub use controller::generate_router;

#[derive(Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_owned(),
        }
    }
}

#[derive(Deserialize)]
pub struct AuthPayload {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    email: String,
    exp: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let token = header.0.token();

        let token_data: TokenData<Claims> =
            jsonwebtoken::decode(token, &KEYS.decoding, &jsonwebtoken::Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        let exp = i64::try_from(token_data.claims.exp).map_err(|_| AuthError::TokenExpiration)?;
        if exp < chrono::Utc::now().timestamp() {
            return Err(AuthError::TokenExpiration);
        }

        Ok(token_data.claims)
    }
}

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET should be present!");
    Keys::new(secret.as_bytes())
});

pub enum AuthError {
    InvalidToken,
    MissingCredentials,
    TokenCreation,
    IncorrectCredentials,
    TokenExpiration,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            Self::IncorrectCredentials => (StatusCode::BAD_REQUEST, "Incorrect credentials"),
            Self::TokenExpiration => (StatusCode::UNAUTHORIZED, "Credential expired"),
        };

        (status_code).into_response()
    }
}
