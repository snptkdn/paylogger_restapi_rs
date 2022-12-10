use rocket;
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



