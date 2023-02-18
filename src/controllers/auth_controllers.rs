use rocket::response::Redirect;
use rocket_contrib::json::Json;
use rocket::response::status::{self, BadRequest};
use futures::executor::block_on;
use dotenv::dotenv;
use std::env;

#[get("/login")]
pub fn login() -> Redirect {
    dotenv().ok();
    Redirect::to(env::var("DISCORD_AUTH_URL").unwrap())
}

#[get("/login/callback?<code>")]
pub fn login_callback(code: String) -> String {
    code
}
