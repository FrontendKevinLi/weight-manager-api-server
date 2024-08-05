use axum::Router;
use dotenv::dotenv;
use sqlx::mysql::MySql;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::pool::Pool;
use tokio::net::TcpListener;

pub mod response;
use crate::response::StandardResponse;
pub mod employees;
use crate::employees::controller::employees_router;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<MySql>,
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
        .nest("/employees", employees_router())
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
