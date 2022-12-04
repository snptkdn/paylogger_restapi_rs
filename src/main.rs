#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod models;
mod services;
mod controllers;

fn main() {
    rocket::ignite()
        .mount("/", routes![controllers::log_controllers::index])  // ここにルーティングをセットする
        .mount("/log", routes![controllers::log_controllers::post_new_log])  // ここにルーティングをセットする
        .launch();
}
