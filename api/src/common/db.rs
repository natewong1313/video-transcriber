use std::error::Error;

use sqlx::{
    Sqlite, SqlitePool,
    migrate::{MigrateDatabase, Migrator},
};

pub async fn connect(db_url: &str) -> Result<SqlitePool, Box<dyn Error>> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("created database"),
            Err(err) => panic!("error creating database: {}", err),
        }
    }
    let pool = SqlitePool::connect(db_url).await?;

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
