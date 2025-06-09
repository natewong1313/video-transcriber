use anyhow::Result;
use dotenv::dotenv;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let db_conn = PgPool::connect(&db_url).await?;

    let message = r#"{"url": "https://videos3.nate-wong.com/1/sample.mp3", "id": 1}"#;
    sqlx::query("SELECT pg_notify('transcriber_tasks', $1)")
        .bind(message)
        .execute(&db_conn)
        .await?;
    Ok(())
}
