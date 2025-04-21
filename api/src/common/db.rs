use std::error::Error;

use sqlx::{
    Sqlite, SqlitePool,
    migrate::{MigrateDatabase, Migrator},
};

const DB_URL: &str = "sqlite://test.db";

pub async fn connect() -> Result<SqlitePool, Box<dyn Error>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("created database"),
            Err(err) => panic!("error creating database: {}", err),
        }
    }
    let pool = SqlitePool::connect(DB_URL).await?;

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations_dir = std::path::Path::new(&crate_dir).join("./migrations");
    match Migrator::new(migrations_dir)
        .await
        .unwrap()
        .run(&pool)
        .await
    {
        Ok(_) => println!("performed migrations"),
        Err(err) => panic!("error performing migrations: {}", err),
    }

    Ok(pool)
}
