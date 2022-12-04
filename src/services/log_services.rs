use anyhow::Result;
use sqlx::query;
use crate::services::db;
use crate::models::log_models;

struct LogServices {
}

impl LogServices {
    async fn insert(log: log_models::Log) -> Result<()> {
        let db = db::Db::new().await;
        let pool = db.0.clone();

        let _ = query("insert into log (price, category, date) values ($1, $2, $3)")
            .bind(log.price)
            .bind(log.category)
            .bind(log.buy_date.format("%Y/%m/%d").to_string())
            .execute(&*pool)
            .await?;

        Ok(())
    }
}