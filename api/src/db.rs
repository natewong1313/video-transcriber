use std::error::Error;

use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

const DB_URL: &str = "sqlite://test.db";

pub async fn connect_db() -> Result<SqlitePool, Box<dyn Error>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("created database"),
            Err(err) => panic!("error creating database: {}", err),
        }
    }
    let pool = SqlitePool::connect(DB_URL).await?;
    Ok(pool)
}
