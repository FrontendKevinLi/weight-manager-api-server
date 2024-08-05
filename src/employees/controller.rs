use super::service;
use super::{Employee, InsertEmployee};
use crate::{AppState, StandardResponse};
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, put};
use axum::routing::post;
use axum::Json;
use axum::Router;
use axum_macros;

pub fn employees_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_employees))
        .route("/", post(insert_employee))
        .route("/:user_id", get(get_employee_by_id))
        .route("/:user_id", put(put_employee_by_id))
}

#[axum_macros::debug_handler]
async fn get_employees(
    State(app_state): State<AppState>,
) -> Result<axum::Json<StandardResponse<Vec<Employee>>>, StatusCode> {
    match service::fetch_employees(&app_state.pool).await {
        Ok(value) => Ok(StandardResponse::success(value)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum_macros::debug_handler]
async fn get_employee_by_id(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<axum::Json<StandardResponse<Option<Employee>>>, StatusCode> {
    match service::fetch_employee_by_id(&app_state.pool, user_id).await {
        Ok(value) => Ok(StandardResponse::success(Option::from(value))),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Ok(StandardResponse::success(Option::None)),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

#[axum_macros::debug_handler]
async fn insert_employee(
    State(app_state): State<AppState>,
    Json(employee): Json<InsertEmployee>,
) -> Result<axum::Json<StandardResponse<u64>>, StatusCode> {
    match service::insert_employee(&app_state.pool, employee).await {
        Ok(value) => Ok(StandardResponse::success(value)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[axum_macros::debug_handler]
async fn put_employee_by_id(
    Path(user_id): Path<i64>,
    State(app_state): State<AppState>,
    Json(employee): Json<InsertEmployee>
) -> Result<Json<StandardResponse<u64>>, StatusCode> {
    match service::put_employee(&app_state.pool, employee, user_id).await {
        Ok(value) => Ok(StandardResponse::success(value)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
