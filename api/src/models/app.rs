use aws_sdk_s3::Client;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub s3_client: Client,
}
