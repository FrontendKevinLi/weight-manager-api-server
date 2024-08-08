mod controller;
mod service;

pub use controller::generate_router;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
}
