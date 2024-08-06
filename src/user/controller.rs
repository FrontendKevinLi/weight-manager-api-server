use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use axum::Json;
use axum::Router;

use crate::response;
use crate::AppState;

use super::service;
use super::CreateUser;
use super::User;

pub fn generate_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user_by_id))
        .route("/:id", put(update_user))
}

#[axum::debug_handler]
async fn get_users(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    match service::fetch_users(&app_state.pool).await {
        Ok(users) => Ok(response::success(users)),
        Err(_) => Err(response::failed()),
    }
}

#[axum::debug_handler]
async fn get_user_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Option<User>>, (StatusCode, String)> {
    match service::fetch_users_by_id(&app_state.pool, id).await {
        Ok(user) => Ok(response::success(Option::from(user))),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Ok(response::success(None)),
            err => Err(response::failed_with_message(err.to_string())),
        },
    }
}

#[axum::debug_handler]
async fn create_user(
    State(app_state): State<AppState>,
    Json(user): Json<CreateUser>,
) -> Result<Json<u64>, StatusCode> {
    match service::insert_user(&app_state.pool, user).await {
        Ok(id) => Ok(response::success(id)),
        Err(_) => Err(response::failed()),
    }
}

#[axum::debug_handler]
async fn update_user(
    State(app_state): State<AppState>,
    Path(id): Path<u64>,
    Json(user): Json<CreateUser>,
) -> Result<Json<u64>, StatusCode> {
    match service::update_user(&app_state.pool, user, id).await {
        Ok(id) => Ok(response::success(id)),
        Err(_) => Err(response::failed()),
    }
}
