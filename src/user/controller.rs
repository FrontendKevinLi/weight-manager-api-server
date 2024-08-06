use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Json;
use axum::Router;

use crate::response;
use crate::AppState;

use super::service;
use super::User;

pub fn generate_router() -> Router<AppState> {
    Router::new().route("/", get(get_users))
}

#[axum_macros::debug_handler]
async fn get_users(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    match service::fetch_users(&app_state.pool).await {
        Ok(users) => Ok(response::success(users)),
        Err(_) => Err(response::failed()),
    }
}
