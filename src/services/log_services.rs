use std::collections::HashMap;
use anyhow::{Result, anyhow};
use sqlx::{query, query_as};
use crate::services::db;
use crate::models::log_models::Log;
use chrono::*;


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

pub async fn get_total_this_month() -> Result<i64> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

     let each_amount = query!("SELECT price FROM log WHERE DATE_FORMAT(buy_date, '%Y%m') = DATE_FORMAT(NOW(), '%Y%m') ")
    .fetch_all(&*pool)
    .await?;

    let total_amount = each_amount
        .iter()
        .map(|record| record.price.unwrap() as i64)
        .sum::<i64>();
    
    Ok(total_amount)
}

pub async fn get_price_per_date_this_month() -> Result<HashMap<String, i64>> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

     let each_amount = query!("
        SELECT 
            buy_date, price
        FROM 
            log 
        WHERE
            DATE_FORMAT(buy_date, '%Y%m') = DATE_FORMAT(NOW(), '%Y%m')
        ORDER BY
            buy_date ASC;
     ")
    .fetch_all(&*pool)
    .await?;

    let mut price_per_day = HashMap::<String, i64>::new();
    for record in each_amount {
        price_per_day
            .entry(record.buy_date.unwrap().to_string())
            .and_modify(|price| *price += record.price.unwrap() as i64)
            .or_insert(record.price.unwrap() as i64);
    }

    Ok(price_per_day)
}

pub async fn get_price_per_day(
    year: Option<usize>,
    month: Option<usize>,
    day: Option<usize>
) -> Result<HashMap<String, i64>> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

     let each_amount = query!("
        SELECT 
            buy_date, price
        FROM 
            log 
        WHERE
            DATE_FORMAT(buy_date, ?) = ?
        ORDER BY
            buy_date ASC;
     ",
     get_date_format(year, month, day)?,
     get_date_string(year, month, day)?)
    .fetch_all(&*pool)
    .await?;

    let mut price_per_day = HashMap::<String, i64>::new();
    for record in each_amount {
        price_per_day
            .entry(record.buy_date.unwrap().to_string())
            .and_modify(|price| *price += record.price.unwrap() as i64)
            .or_insert(record.price.unwrap() as i64);
    }

    Ok(price_per_day)
}

pub async fn get_price_per_category_this_month() -> Result<HashMap<String, i64>> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

     let each_amount = query!("
        SELECT 
            *
        FROM 
            log 
        INNER JOIN
            category 
        ON 
            log.category = category.id
        WHERE
            DATE_FORMAT(buy_date, '%Y%m') = DATE_FORMAT(NOW(), '%Y%m');
     ")
    .fetch_all(&*pool)
    .await?;

    let mut price_per_category = HashMap::<String, i64>::new();
    for record in each_amount {
        price_per_category
            .entry(record.name.unwrap().to_string())
            .and_modify(|price| *price += record.price.unwrap() as i64)
            .or_insert(record.price.unwrap() as i64);
    }

    Ok(price_per_category)
}

pub async fn get_price_per_category_month(month: String) -> Result<HashMap<String, i64>> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();
    let today = Local::now().naive_local();

    let each_amount = query!("
        SELECT 
            *
        FROM 
            log 
        INNER JOIN
            category 
        ON 
            log.category = category.id
        WHERE
            DATE_FORMAT(buy_date, '%Y%m') = ?;
     ", format!("{}{}", today.year(), month))
    .fetch_all(&*pool)
    .await?;
    println!("{}{}", today.year(), month);

    let mut price_per_category = HashMap::<String, i64>::new();
    for record in each_amount {
        price_per_category
            .entry(record.name.unwrap().to_string())
            .and_modify(|price| *price += record.price.unwrap() as i64)
            .or_insert(record.price.unwrap() as i64);
    }

    Ok(price_per_category)
}

pub async fn get_price_per_category(
    year: Option<usize>,
    month: Option<usize>,
    day: Option<usize>
) -> Result<HashMap<String, i64>> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

    let each_amount = query!
    (
        "
        SELECT 
            *
        FROM 
            log 
        INNER JOIN
            category 
        ON 
            log.category = category.id
        WHERE
            DATE_FORMAT(buy_date, ?) = ?;
         ", 
         get_date_format(year, month, day)?,
         get_date_string(year, month, day)?,
    )
    .fetch_all(&*pool)
    .await?;

    let mut price_per_category = HashMap::<String, i64>::new();
    for record in each_amount {
        price_per_category
            .entry(record.name.unwrap().to_string())
            .and_modify(|price| *price += record.price.unwrap() as i64)
            .or_insert(record.price.unwrap() as i64);
    }

    Ok(price_per_category)
}

fn get_date_format(
    year: Option<usize>,
    month: Option<usize>,
    day: Option<usize>
) -> Result<String> {
    match (year, month, day) {
        (Some(_), Some(_), Some(_)) => Ok("%Y%m%d".to_string()),
        (Some(_), Some(_), None   ) => Ok("%Y%m".to_string()),
        (Some(_), None   , None    )=> Ok("%Y".to_string()),
        _ => Err(anyhow!("Query string of date is invalid!"))
    }
}

fn get_date_string(
    year: Option<usize>,
    month: Option<usize>,
    day: Option<usize>
) -> Result<String> {
    match (year, month, day) {
        (Some(year), Some(month), Some(day)) => Ok(format!("{}{:>02}{:>02}", year, month, day)),
        (Some(year), Some(month), None     ) => Ok(format!("{}{:>02}", year, month)),
        (Some(year), None       , None     ) => Ok(format!("{}", year)),
        _ => Err(anyhow!("Query string of date is invalid!"))
    }
}
