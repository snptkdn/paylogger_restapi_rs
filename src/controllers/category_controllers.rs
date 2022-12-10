use rocket;
use crate::models::category_models::Category;
use crate::services::category_services;
use rocket_contrib::json::Json;
use rocket::response::status::{self, BadRequest};
use anyhow::Result;
use futures::executor::block_on;

#[get("/")]
pub fn index() -> Result<status::Accepted<Json<Vec<Category>>>, BadRequest<String>> {
    match block_on(category_services::get()) {
        Ok(logs) => Ok(status::Accepted(Some(Json(logs)))),
        Err(e) => Err(status::BadRequest(Some(e.to_string())))
    }
}

#[post("/", data = "<category>")]
pub fn post_new_category(category: Json<Category>) -> Result<status::Accepted<String>, BadRequest<String>> { 
    let category = Category {
        id: category.id,
        name: category.name.clone(),
    };

    match block_on(category_services::insert(category)) {
        Ok(()) => Ok(status::Accepted(Some("insert success.".to_string()))),
        Err(e) => Err(status::BadRequest(Some(format!("insert failed. {}", e))))
    }
    
}



