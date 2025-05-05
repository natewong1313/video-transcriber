use crate::services::transcribe::TranscribeTask;
use aws_sdk_s3::Client;
use sqlx::{Pool, Sqlite};
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub s3_client: Client,
    pub tx: Sender<TranscribeTask>,
}
