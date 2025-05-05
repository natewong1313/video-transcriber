use aws_sdk_s3::primitives::ByteStream;
use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    common::errors::ApiError,
    models::app::AppState,
    services::{auth::AuthSession, transcribe::TranscribeTask},
};

pub async fn upload(
    auth_session: AuthSession,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let user = auth_session.user.ok_or(ApiError::Unauthorized)?;
    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("file") {
            continue;
        }

        let file_name = field
            .file_name()
            .ok_or(ApiError::Unknown("Missing file name".to_owned()))?
            .to_string();
        tracing::debug!("uploading ${} to s3", file_name);
        let data = field.bytes().await?;
        let body = ByteStream::from(data);
        let object_key = format!("{}/{}", user.id, file_name);
        state
            .s3_client
            .put_object()
            .bucket("video-transcriber")
            .key(&object_key)
            .body(body)
            .send()
            .await?;
        tracing::debug!("uploaded ${} to s3, sending to consumer", file_name);
        let task = TranscribeTask {
            url: object_key.clone(),
            user_id: user.id,
        };
        let _result = state.tx.send(task).await?;
        return Ok((
            StatusCode::OK,
            Json(json!({"message": "File uploaded successfully", "location": object_key})),
        ));
    }
    Err(ApiError::Unknown("No file found".to_owned()))
}
