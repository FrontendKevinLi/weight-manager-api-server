use super::{service, WeightRecord};
use crate::{response, AppState};
use axum::extract::State;
use axum::routing::get;
use axum::{http::StatusCode, Json, Router};

pub fn generate_router() -> Router<AppState> {
    Router::new().route("/", get(get_weight_records))
    // .route("/", post(create_weight_records))
}

#[axum::debug_handler]
async fn get_weight_records(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<WeightRecord>>, StatusCode> {
    match service::fetch_weight_records(&app_state.pool).await {
        Ok(weight_records) => Ok(response::success(weight_records)),
        Err(_) => Err(response::failed()),
    }
}

// #[axum::debug_handler]
// async fn create_weight_records(
//     State(app_state): State<AppState>,
//     Json(weight_record): Json<CreateWeightRecord>,
// ) -> Result<Json<u64>, StatusCode> {
//     match service::insert_weight_record(&app_state.pool, weight_record).await {
//         Ok(id) => Ok(response::success(id)),
//         Err(_) => Err(response::failed()),
//     }
// }
