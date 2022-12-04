use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Log {
   pub price: i64,
   pub category: i64,
   pub buy_date: String
}