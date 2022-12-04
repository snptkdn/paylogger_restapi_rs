use rocket;
use crate::models::log_models::Log;
use crate::services::log_services;
use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket::response::status::{self, BadRequest};
use rocket::http::Status;
use anyhow::Result;
use futures::executor::block_on;


#[get("/")]
pub fn index() -> &'static str {
    "Welcome to PAYLOGGER"
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
        //Ok() => Ok(status::Accepted(Some("insert success.").to_string())),
        Err(e) => Err(status::BadRequest(Some(format!("insert failed. {}", e))))
    }
    
}



