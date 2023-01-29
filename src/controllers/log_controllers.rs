use rocket;
use std::collections::HashMap;
use crate::models::log_models::Log;
use crate::services::log_services;
use crate::services::validation_services::is_valid_date;
use rocket_contrib::json::Json;
use rocket::response::status::{self, BadRequest};
use futures::executor::block_on;

#[get("/")]
pub fn index() -> Result<status::Accepted<Json<Vec<Log>>>, BadRequest<String>> {
    match block_on(log_services::get()) {
        Ok(logs) => Ok(status::Accepted(Some(Json(logs)))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[get("/per_day?<year>&<month>&<day>")]
pub fn per_day(
    year: Option<usize>,
    month: Option<usize>,
    day: Option<usize>
) -> Result<status::Accepted<Json<HashMap<String, i64>>>, BadRequest<String>> {
    if !is_valid_date(year, month, day) {
        return Err(status::BadRequest(Some("invalid date!".to_string())));
    };

    match block_on(log_services::get_price_per_day(year, month, day)) {
        Ok(each_category) => Ok(status::Accepted(Some(Json(each_category)))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[get("/per_category?<year>&<month>&<day>")]
pub fn per_category(
    year: Option<usize>,
    month: Option<usize>,
    day: Option<usize>
) -> Result<status::Accepted<Json<HashMap<String, i64>>>, BadRequest<String>> {
    if !is_valid_date(year, month, day) {
        return Err(status::BadRequest(Some("invalid date!".to_string())));
    };

    match block_on(log_services::get_price_per_category(year, month, day)) {
        Ok(each_category) => Ok(status::Accepted(Some(Json(each_category)))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[get("/total?<year>&<month>&<day>")]
pub fn total(
    year: Option<usize>,
    month: Option<usize>,
    day: Option<usize>
) -> Result<status::Accepted<String>, BadRequest<String>> {
    if !is_valid_date(year, month, day) {
        return Err(status::BadRequest(Some("invalid date!".to_string())));
    };

    match block_on(log_services::get_total(year, month, day)) {
        Ok(total_amount) => Ok(status::Accepted(Some(total_amount.to_string()))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[post("/", data = "<log>")]
pub fn post_new_log(log: Json<Log>) -> Result<status::Accepted<String>, BadRequest<String>> { 
    let log = Log {
        price: log.price,
        category: log.category,
        buy_date: log.buy_date.to_string(),
    };

    match block_on(log_services::insert(log)) {
        Ok(()) => Ok(status::Accepted(Some("insert success.".to_string()))),
        Err(e) => Err(status::BadRequest(Some(format!("insert failed. {}", e))))
    }
    
}



