use anyhow::Result;
use sqlx::query;
use crate::services::db;
use crate::models::log_models;

pub async fn insert(log: log_models::Log) -> Result<()> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

    let _ = query("insert into log (price, category, buy_date) values (?, ?, ?)")
        .bind(log.price)
        .bind(log.category)
        .bind(log.buy_date)
        .execute(&*pool)
        .await?;

    Ok(())
}