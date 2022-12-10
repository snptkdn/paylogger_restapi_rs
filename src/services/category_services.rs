use anyhow::Result;
use sqlx::{query, query_as};
use crate::services::db;
use crate::models::category_models::Category;

pub async fn insert(category: Category) -> Result<()> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

    let _ = query("insert into category (name) values (?)")
        .bind(category.name)
        .execute(&*pool)
        .await?;

    Ok(())
}

pub async fn get() -> Result<Vec<Category>> {
    let db = db::Db::new().await?;
    let pool = db.0.clone();

    let categories = 
        query_as::<_, Category>("select * from category")
        .fetch_all(&*pool)
        .await?;
    
    Ok(categories)
}