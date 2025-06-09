use anyhow::{Error, Result};
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgListener};
use std::io::ErrorKind;
use tokio::{
    fs,
    task::{self},
};
use transcriber::DOWNLOADS_FOLDER_PATH;
use whisper::ModelType;
mod converter;
mod db;
mod models;
mod transcriber;
mod utils;
mod whisper;

const MODEL_TYPE: ModelType = ModelType::Large;
const CHANNEL: &str = "transcriber_tasks";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    // init directories
    if let Err(e) = fs::remove_dir_all(transcriber::DOWNLOADS_FOLDER_PATH).await {
        if e.kind() != ErrorKind::NotFound {
            return Err(Error::new(e));
        }
    };
    fs::create_dir_all(DOWNLOADS_FOLDER_PATH).await?;

    let model_path = whisper::download_model(MODEL_TYPE).await?;

    let db_pool = PgPool::connect(&db_url).await?;
    let mut db_listener = PgListener::connect(&db_url).await?;
    db_listener.listen(CHANNEL).await?;
    loop {
        let new_task = db_listener.recv().await?;
        let transcriber_task: models::Task = serde_json::from_str(new_task.payload())?;
        let c_model_path = model_path.clone();
        let c_db_pool = db_pool.clone();
        task::spawn_blocking(move || {
            transcriber::start(MODEL_TYPE, c_model_path, transcriber_task, c_db_pool)
        });
    }
}
