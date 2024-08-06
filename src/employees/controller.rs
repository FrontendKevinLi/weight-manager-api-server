use super::service;
use super::{Employee, InsertEmployee};
use crate::response;
use crate::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::routing::{get, put};
use axum::Json;
use axum::Router;

pub fn employees_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_employees))
        .route("/", post(insert_employee))
        .route("/:user_id", get(get_employee_by_id))
        .route("/:user_id", put(put_employee_by_id))
}

#[axum::debug_handler]
async fn get_employees(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Employee>>, StatusCode> {
    match service::fetch_employees(&app_state.pool).await {
        Ok(value) => Ok(Json(value)),
        Err(_) => Err(response::failed()),
    }
}

#[axum::debug_handler]
async fn get_employee_by_id(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Option<Employee>>, (StatusCode, String)> {
    match service::fetch_employee_by_id(&app_state.pool, user_id).await {
        Ok(value) => Ok(response::success(Option::from(value))),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Ok(response::success(None)),
            err => Err(response::failed_with_message(err.to_string())),
        },
    }
}

#[axum::debug_handler]
async fn insert_employee(
    State(app_state): State<AppState>,
    Json(employee): Json<InsertEmployee>,
) -> Result<Json<u64>, StatusCode> {
    match service::insert_employee(&app_state.pool, employee).await {
        Ok(value) => Ok(response::success(value)),
        Err(_) => Err(response::failed()),
    }
}

#[axum::debug_handler]
async fn put_employee_by_id(
    Path(user_id): Path<i64>,
    State(app_state): State<AppState>,
    Json(employee): Json<InsertEmployee>,
) -> Result<Json<u64>, StatusCode> {
    match service::put_employee(&app_state.pool, employee, user_id).await {
        Ok(value) => Ok(response::success(value)),
        Err(_) => Err(response::failed()),
    }
}
