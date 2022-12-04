use anyhow::Result;
use sqlx::{query, query_as};
use crate::services::db;
use crate::models::log_models::Log;

pub async fn insert(log: Log) -> Result<()> {
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

pub async fn get() -> Result<Vec<Log>> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

    //let logs = query_as::<_, Log>(query)
    let records = query!("select * from log")
        .fetch_all(&*pool)
        .await?;
    
    let mut logs = Vec::new();
    for record in records {
        let log = Log {
            price: record.price.unwrap_or(0) as i64,
            category: record.category.unwrap_or(0) as i64,
            buy_date: record.buy_date.unwrap().to_string(),
        };

        logs.push(log);
    }

    Ok(logs)
}