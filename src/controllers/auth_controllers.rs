use crate::services::auth_services::{get_access_token, get_me_info};
use dotenv::dotenv;
use futures::executor::block_on;
use rocket::response::status::{Accepted, BadRequest};
use rocket::response::Redirect;
use rocket_contrib::json::Json;
use std::env;

#[get("/login")]
pub fn login() -> Redirect {
    dotenv().ok();
    Redirect::to(env::var("DISCORD_AUTH_URL").unwrap())
}

#[get("/login/callback?<code>")]
pub async fn login_callback(code: String) -> Result<Accepted<String>, BadRequest<String>> {
    let token = get_access_token(&code).await.expect("Error");
    let me_info = get_me_info(&token).await.expect("Error");
    Ok(Accepted(Some(me_info.user.id)))
}
