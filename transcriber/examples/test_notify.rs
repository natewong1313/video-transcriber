use anyhow::Result;
use dotenv::dotenv;
use sqlx::PgPool;

use transcriber::{
    db::Db,
    models::{Task, TaskStatus},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let db_pool = PgPool::connect(&db_url).await?;

    let vid_url = "https://videos3.nate-wong.com/1/sample.mp3";

    let db = Db::new(db_pool.clone());
    let task_id = db.create_task(&vid_url).await?;
    let task = Task {
        id: task_id,
        url: vid_url.to_string(),
        status: TaskStatus::NotStarted.to_string(),
    };

    sqlx::query("SELECT pg_notify('transcriber_tasks', $1)")
        .bind(task.to_json_str())
        .execute(&db_pool)
        .await?;
    Ok(())
}
