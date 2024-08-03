use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use axum_macros;
use crate::{AppState, StandardResponse};
use super::Employee;
use super::service::fetch_employees;
use super::service::fetch_employee_by_id;

pub fn employees_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_employees))
		.route("/:user_id", get(get_employee_by_id))
}

#[axum_macros::debug_handler]
async fn get_employees(State(app_state): State<AppState>) -> Result<axum::Json<StandardResponse<Vec<Employee>>>, StatusCode> {
    match fetch_employees(&app_state.pool).await {
        Ok(value) => Ok(StandardResponse::success(value)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_employee_by_id(State(app_state): State<AppState>, Path(user_id): Path<i64>) -> Result<axum::Json<StandardResponse<Option<Employee>>>, StatusCode> {
	match fetch_employee_by_id(&app_state.pool, user_id).await {
		Ok(value) => Ok(StandardResponse::success(Option::from(value))),
		Err(err) => {
			match err {
				sqlx::Error::RowNotFound => Ok(StandardResponse::success(Option::None)),
				_ => Err(StatusCode::INTERNAL_SERVER_ERROR)
			}

			// match err.into_database_error() {
			// 	Some(err) => 
			// 		Err((StatusCode::INTERNAL_SERVER_ERROR, err.message().to_string()))
			// 	,
			// 	None => Err((StatusCode::INTERNAL_SERVER_ERROR, StatusCode::INTERNAL_SERVER_ERROR.to_string()))
			// }
			// Err((StatusCode::INTERNAL_SERVER_ERROR, db_err.message().to_string()))
			// Err((StatusCode::INTERNAL_SERVER_ERROR, "Test".to_string()))
		}
	}
}
