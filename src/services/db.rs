use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Pool, MySql};
use std::env;
use dotenv;
use std::sync::Arc;
use anyhow::Result;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Result<Db> {
        dotenv::dotenv().ok();
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(
                &env::var("DATABASE_URL")?
            )
            .await?;

        Ok(Db(Arc::new(pool)))
    }
}