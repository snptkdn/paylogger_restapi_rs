#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket::response::status;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method;

#[macro_use]
extern crate rocket;

mod models;
mod services;
mod controllers;

#[get("/")]
pub fn root() -> status::Accepted<String> {
    status::Accepted(Some("Welcome to paylogger!".to_string()))
}

fn main() {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::ignite()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![root])
        .mount("/log", routes![controllers::log_controllers::index])
        .mount("/log", routes![controllers::log_controllers::post_new_log])
        .mount("/log", routes![controllers::log_controllers::per_category])
        .mount("/log", routes![controllers::log_controllers::per_day])
        .mount("/log", routes![controllers::log_controllers::total])
        .mount("/category", routes![controllers::category_controllers::get_category_id]) 
        .mount("/category", routes![controllers::category_controllers::post_new_category])
        .launch();
}
