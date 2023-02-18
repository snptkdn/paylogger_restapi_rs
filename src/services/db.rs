use anyhow::Result;
use dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Result<Db> {
        dotenv::dotenv().ok();
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(&env::var("DATABASE_URL")?)
            .await?;

        Ok(Db(Arc::new(pool)))
    }
}
