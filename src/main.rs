#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod models;
mod services;
mod controllers;

fn main() {
    rocket::ignite()
        .mount("/", routes![controllers::log_controllers::index])
        .mount("/log", routes![controllers::log_controllers::post_new_log])
        .mount("/log", routes![controllers::log_controllers::this_month])
        .mount("/log", routes![controllers::log_controllers::this_month_per_day])
        .mount("/log", routes![controllers::log_controllers::this_month_per_category])
        .mount("/log", routes![controllers::log_controllers::month_per_category])
        .mount("/log", routes![controllers::log_controllers::per_category])
        .mount("/log", routes![controllers::log_controllers::per_day])
        .mount("/log", routes![controllers::log_controllers::total])
        .mount("/category", routes![controllers::category_controllers::get_category_id]) 
        .mount("/category", routes![controllers::category_controllers::post_new_category])
        .launch();
}
