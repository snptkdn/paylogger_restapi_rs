use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Option<i64>,
    pub name: String,
}
