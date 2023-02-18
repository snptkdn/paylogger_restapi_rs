use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Log {
    pub price: i64,
    pub category: i64,
    pub buy_date: String,
}
