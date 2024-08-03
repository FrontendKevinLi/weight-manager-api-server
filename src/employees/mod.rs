pub mod controller;
mod service;
use time::Date;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Employee {
    pub employee_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub hire_date: Option<Date>,
    pub job_title: Option<String>,
    pub salary: Option<rust_decimal::Decimal>,
}
