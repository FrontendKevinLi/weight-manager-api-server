mod controller;
mod service;

use chrono::prelude::*;
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

#[derive(Deserialize, Debug)]
pub struct DateRange {
    start_date: String,
    end_date: String,
}

impl Default for DateRange {
    fn default() -> Self {
        let utc_now = Utc::now();
        let utc_now_days = get_days_from_month(utc_now.year(), utc_now.month()).unwrap_or(28);

        Self {
            start_date: utc_now.with_day(1).unwrap().format("%Y-%m-%d").to_string(),
            end_date: utc_now
                .with_day(utc_now_days)
                .expect("Default with 28 days must be valid")
                .format("%Y-%m-%d")
                .to_string(),
        }
    }
}

pub fn get_days_from_month(year: i32, month: u32) -> Result<u32, std::num::TryFromIntError> {
    let days = NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .expect("Day one must exist")
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).expect("Day one must exist"))
    .num_days();

    u32::try_from(days)
}
