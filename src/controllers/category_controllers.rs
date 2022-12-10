use rocket;
use crate::models::category_models::Category;
use crate::services::category_services;
use rocket_contrib::json::Json;
use rocket::response::status::{self, BadRequest};
use anyhow::Result;
use futures::executor::block_on;

#[get("/?<name>")]
pub fn get_category_id(name: Option<String>) -> Result<status::Accepted<Json<Vec<Category>>>, BadRequest<String>> {
    if let Some(name) = name {
        match block_on(category_services::get_cateogry_by_id(name)) {
            Ok(category) => Ok(status::Accepted(Some(Json(vec!(category))))),
            Err(e) => Err(status::BadRequest(Some(e.to_string())))
        }
    } else {
        match block_on(category_services::get()) {
            Ok(categories) => Ok(status::Accepted(Some(Json(categories)))),
            Err(e) => Err(status::BadRequest(Some(e.to_string())))
        }
    }
    
}

#[post("/", data = "<category>")]
pub fn post_new_category(category: Json<Category>) -> Result<status::Accepted<String>, BadRequest<String>> { 
    let category = Category {
        id: None,
        name: category.name.clone(),
    };

    match block_on(category_services::insert(category)) {
        Ok(()) => Ok(status::Accepted(Some("insert success.".to_string()))),
        Err(e) => Err(status::BadRequest(Some(format!("insert failed. {}", e))))
    }
    
}



