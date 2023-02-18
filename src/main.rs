#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use models::preflight_models;
use rocket::http::Method;
use rocket::response::status;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod services;

#[get("/")]
pub fn root() -> status::Accepted<String> {
    status::Accepted(Some("Welcome to paylogger!".to_string()))
}

#[options("/<_p..>")]
pub fn preflight(_p: std::path::PathBuf) -> preflight_models::PfResponse {
    preflight_models::PfResponse
}

#[launch]
fn rocket() -> _ {
    //let cors = CorsOptions::default()
    //    .allowed_origins(AllowedOrigins::all())
    //    .allowed_methods(
    //        vec![Method::Get, Method::Post, Method::Patch]
    //            .into_iter()
    //            .map(From::from)
    //            .collect(),
    //    )
    //    .allow_credentials(true);

    rocket::build()
        .mount("/", routes![root])
        .mount("/", routes![preflight])
        .mount("/log", routes![controllers::log_controllers::index])
        .mount("/log", routes![controllers::log_controllers::post_new_log])
        .mount("/log", routes![controllers::log_controllers::per_category])
        .mount("/log", routes![controllers::log_controllers::per_day])
        .mount("/log", routes![controllers::log_controllers::total])
        .mount(
            "/category",
            routes![controllers::category_controllers::get_category_id],
        )
        .mount(
            "/category",
            routes![controllers::category_controllers::post_new_category],
        )
        .mount("/auth", routes![controllers::auth_controllers::login])
        .mount(
            "/auth",
            routes![controllers::auth_controllers::login_callback],
        )
}
