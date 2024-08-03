use axum::{extract::State, routing::get, Router};
use axum::http::StatusCode;
use axum_macros;
use dotenv::dotenv;
use serde;
use serde::Serialize;
use sqlx::mysql::MySql;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::pool::{Pool};
use tokio::net::TcpListener;

pub mod employees;
use crate::employees::controller::employees_router;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<MySql>,
}

#[derive(Serialize)]
pub struct StandardResponse<TData> {
    pub message: String,
    pub data: TData,
}

impl<TData> StandardResponse<TData> {
    fn success(data: TData) -> axum::Json<StandardResponse<TData>> {
        axum::Json(StandardResponse {
            message: String::from("success"),
            data,
        })
    }

    fn failed(err: sqlx::Error, data: TData) -> axum::Json<StandardResponse<TData>> {
        axum::Json(StandardResponse {
            message: err.to_string(),
            data,
        })
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database url should be present!");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|err| panic!("Failed to connect to database: {}", err));

    let app_state = AppState { pool };

    let app = Router::new()
        .route("/", get(default_controller))
        .nest("/employees", employees_router())
        .route("/error", get(error_controller))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[axum_macros::debug_handler]
async fn default_controller(State(app_state): State<AppState>) -> axum::Json<StandardResponse<i64>> {
    match default_service(&app_state.pool).await {
        Ok(value) => StandardResponse::success(value),
        Err(err) => StandardResponse::failed(err, 0),
    }
}

async fn default_service(pool: &Pool<MySql>) -> Result<i64, sqlx::Error> {
    match sqlx::query_as::<MySql, (i64,)>("SELECT ?")
        .bind(150_i64)
        .fetch_one(pool)
        .await
    {
        Ok(row) => Ok(row.0),
        Err(err) => Err(err),
    }
}

async fn error_controller() -> Result<String, (StatusCode, String)> {
    Err(internal_error(&String::from("Test Error")))
}

fn internal_error(err: &str) -> (StatusCode, String)
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
