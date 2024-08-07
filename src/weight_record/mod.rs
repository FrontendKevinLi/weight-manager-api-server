mod controller;
mod service;
pub use controller::generate_router;
use serde::{Deserialize, Serialize};
pub use service::insert_weight_record;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct WeightRecord {
    id: i64,
    weight: rust_decimal::Decimal,
    date: time::Date,
    create_time: time::OffsetDateTime,
    update_time: time::OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CreateWeightRecord {
    weight: rust_decimal::Decimal,
    date: time::Date,
}
