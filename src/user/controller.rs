use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use axum::Json;
use axum::Router;

use crate::response;
use crate::user_weight_record::UserWeightRecord;
use crate::weight_record::CreateWeightRecord;
use crate::AppState;

use super::service;
use super::CreateUser;
use super::DateRange;
use super::User;

pub fn generate_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user_by_id))
        .route("/:id", put(update_user))
        .route("/:id/weight-record", get(get_weight_record_by_user_id))
        .route("/:id/weight-record", post(create_weight_record_by_user_id))
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
) -> Result<Json<u64>, (StatusCode, String)> {
    let is_user_exist = service::is_user_exist(&app_state.pool, &user.email)
        .await
        .map_err(|_| response::failed_with_message("Server Error".to_string()))?;

    if is_user_exist {
        return Err(response::failed_with_message(
            "The user with this email already exists!".to_string(),
        ));
    }

    match service::insert_user(&app_state.pool, &app_state.argon2_context, user).await {
        Ok(id) => Ok(response::success(id)),
        Err(err) => Err(response::failed_with_message(format!(
            "Server Error: {}",
            err.to_string()
        ))),
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

async fn get_weight_record_by_user_id(
    State(app_state): State<AppState>,
    Path(user_id): Path<u64>,
    date_range: Option<Query<DateRange>>,
) -> Result<Json<Vec<UserWeightRecord>>, StatusCode> {
    let Query(date_range) = date_range.unwrap_or_default();
    dbg!(&date_range);

    match service::fetch_weight_record_by_user_id(&app_state.pool, user_id, date_range).await {
        Ok(records) => Ok(response::success(records)),
        Err(_) => Err(response::failed()),
    }
}

async fn create_weight_record_by_user_id(
    State(app_state): State<AppState>,
    Path(user_id): Path<u64>,
    Json(weight_record): Json<CreateWeightRecord>,
) -> Result<Json<u64>, StatusCode> {
    match service::create_weight_record_by_user_id(&app_state.pool, user_id, weight_record).await {
        Ok(id) => Ok(response::success(id)),
        Err(_) => Err(response::failed()),
    }
}
