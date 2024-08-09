use std::str::FromStr;
use std::time::Duration;

use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::HeaderMap;
use axum::http::Request;
use axum::response::Response;
use axum::Router;
use dotenv::dotenv;
use sqlx::mysql::MySql;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::pool::Pool;
use sqlx::ConnectOptions;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::debug_span;
use tracing::info_span;
use tracing::Span;

mod auth;
mod response;
mod user;
mod user_weight_record;
mod weight_record;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: Pool<MySql>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("Database url should be present!");

    let connection_options = MySqlConnectOptions::from_str(&database_url)
        .unwrap_or_else(|err| panic!("Failed to connect to database: {}", err))
        .log_statements(log::LevelFilter::Debug)
        .log_slow_statements(log::LevelFilter::Debug, Duration::from_secs(1));

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .unwrap_or_else(|err| panic!("Failed to connect to database: {}", err));

    let app_state = AppState { pool };

    let app = Router::new()
        .nest("/auth", auth::generate_router())
        .nest("/users", user::generate_router())
        .nest("/weight-records", weight_record::generate_router())
        .nest(
            "/user-weight-records",
            user_weight_record::generate_router(),
        )
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
