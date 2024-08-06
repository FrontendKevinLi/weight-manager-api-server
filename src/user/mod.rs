mod controller;
mod service;

pub use controller::generate_router;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Serialize, FromRow)]
pub struct User {
    id: i64,
    username: String,
    create_time: OffsetDateTime,
    update_time: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}
