use axum::extract::{Path, State};
use axum::{http::StatusCode, routing::get, Json, Router};

use crate::{response, AppJson, AppState};

use super::service;
use super::UserWeightRecord;

pub fn generate_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_user_weight_records))
        .route("/:id", get(get_user_weight_records_by_id))
}

async fn get_user_weight_records(
    State(app_state): State<AppState>,
) -> Result<AppJson<Vec<UserWeightRecord>>, StatusCode> {
    match service::fetch_all(&app_state.pool).await {
        Ok(records) => Ok(AppJson(records)),
        Err(_) => Err(response::failed()),
    }
}

async fn get_user_weight_records_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<AppJson<Vec<UserWeightRecord>>, StatusCode> {
    match service::fetch_by_id(&app_state.pool, id).await {
        Ok(records) => Ok(response::success(records)),
        Err(_) => Err(response::failed()),
    }
}
