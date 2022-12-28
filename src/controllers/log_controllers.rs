use rocket;
use std::collections::HashMap;
use crate::models::log_models::Log;
use crate::services::log_services;
use rocket_contrib::json::Json;
use rocket::response::status::{self, BadRequest};
use anyhow::Result;
use futures::executor::block_on;

#[get("/")]
pub fn index() -> Result<status::Accepted<Json<Vec<Log>>>, BadRequest<String>> {
    match block_on(log_services::get()) {
        Ok(logs) => Ok(status::Accepted(Some(Json(logs)))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[get("/this_month")]
pub fn this_month() -> Result<status::Accepted<String>, BadRequest<String>> {
    match block_on(log_services::get_total_this_month()) {
        Ok(total) => Ok(status::Accepted(Some(total.to_string()))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[get("/this_month/per_day")]
pub fn this_month_per_day() -> Result<status::Accepted<Json<HashMap<String, i64>>>, BadRequest<String>> {
    match block_on(log_services::get_price_per_date_this_month()) {
        Ok(map) => Ok(status::Accepted(Some(Json(map)))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[get("/this_month/per_category")]
pub fn this_month_per_category() -> Result<status::Accepted<Json<HashMap<String, i64>>>, BadRequest<String>> {
    match block_on(log_services::get_price_per_category_this_month()) {
        Ok(map) => Ok(status::Accepted(Some(Json(map)))),
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



