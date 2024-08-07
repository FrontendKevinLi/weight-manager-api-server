use serde::Serialize;

mod controller;
mod service;
pub use controller::generate_router;
pub use service::insert_user_weight_record;

#[derive(Serialize)]
pub struct UserWeightRecord {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub weight: rust_decimal::Decimal,
    pub date: time::Date,
}
