use anyhow::Result;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
struct OAuthResponse {
    access_token: String,
}

#[derive(Deserialize)]
pub struct MeInfo {
    pub user: DiscordUser,
}

#[derive(Deserialize)]
struct DiscordUser {
    pub user_name: String,
    pub id: String,
}

pub async fn get_access_token(code: &str) -> Result<String> {
    dotenv().ok();
    let mut params = HashMap::new();
    params.insert("client_id", env::var("DISCORD_CLIENT_ID")?);
    params.insert("client_secret", env::var("DISCORD_CLIENT_SECRET")?);
    params.insert("grant_type", "authorization_code".to_string());
    params.insert("code", code.to_string());
    params.insert("redirect_uri", env::var("DISCORD_REDIRECT_URL")?);

    let client = reqwest::Client::new();
    let res = client
        .post(env::var("DISCORD_TOKEN_URL")?)
        .form(&params)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?
        .json::<OAuthResponse>()
        .await?;

    Ok(res.access_token)
}

pub async fn get_me_info(token: &str) -> Result<MeInfo> {
    dotenv().ok();

    let client = reqwest::Client::new();
    let res = client
        .get(env::var("DISCORD_ME_URL")?)
        .bearer_auth(token)
        .send()
        .await?
        .json::<MeInfo>()
        .await?;

    Ok(res)
}
