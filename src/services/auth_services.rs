use anyhow::Result;
use dotenv::dotenv;
use reqwest;
use serde_json::json;
use std::collections::HashMap;
use std::env;

pub async fn get_auth(code: &str) -> Result<String> {
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
        .text()
        .await?;

    Ok(res)
}
