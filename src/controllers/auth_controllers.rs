use crate::services::auth_services::get_auth;
use dotenv::dotenv;
use futures::executor::block_on;
use rocket::response::status::{self, BadRequest};
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use std::env;

#[get("/login")]
pub fn login() -> Redirect {
    dotenv().ok();
    Redirect::to(env::var("DISCORD_AUTH_URL").unwrap())
}

#[get("/login/callback?<code>")]
pub async fn login_callback(code: String) -> String {
    match get_auth(&code).await {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}
