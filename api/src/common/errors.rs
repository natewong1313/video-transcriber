use aws_sdk_s3::{error::SdkError, operation::put_object::PutObjectError};
use axum::{
    Json,
    extract::{multipart::MultipartError, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_valid::{ValidRejection, ValidationRejection};
use serde_json::{Value, json};
use thiserror::Error;
use validator::ValidationErrors;

use crate::services::transcribe::TranscribeTask;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Valid(#[from] ValidRejection<JsonRejection>),

    #[error("{0}")]
    Unknown(String),
    #[error("Valid login required")]
    Unauthorized,
    #[error(transparent)]
    MultiPart(#[from] MultipartError),
    #[error(transparent)]
    S3Error(#[from] SdkError<PutObjectError>),
    #[error(transparent)]
    TranscribeChannelError(#[from] tokio::sync::mpsc::error::SendError<TranscribeTask>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::Valid(rejection) => handle_validation_rejection(rejection),
            ApiError::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, json!({"message": msg})),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                json!({"message": "Valid login required"}),
            ),
            ApiError::MultiPart(err) => (
                StatusCode::BAD_REQUEST,
                json!({"message": format!("Error parsing form: {}", err)}),
            ),
            ApiError::S3Error(err) => (
                StatusCode::BAD_REQUEST,
                json!({"message": format!("Upload error: {}", err)}),
            ),
            ApiError::TranscribeChannelError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message": format!("Upload error: {}", err)}),
            ),
        };
        (status, Json(body)).into_response()
    }
}

fn handle_validation_rejection(
    rejection: ValidationRejection<ValidationErrors, JsonRejection>,
) -> (StatusCode, Value) {
    match rejection {
        ValidationRejection::Valid(validation_errors) => {
            let field_errors = validation_errors.field_errors();
            let invalid_fields: Vec<&str> = field_errors.keys().map(|cow| cow.as_ref()).collect();
            (
                StatusCode::BAD_REQUEST,
                json!({"message": "Validation error", "fields": invalid_fields}),
            )
        }
        ValidationRejection::Inner(rejection) => (
            rejection.status(),
            json!({"message": rejection.body_text()}),
        ),
    }
}

pub fn make_internal_err(message: &str) -> (StatusCode, Json<Value>) {
    let message = json!({"message": message});
    (StatusCode::INTERNAL_SERVER_ERROR, Json(message))
}

pub fn make_user_err(message: &str) -> (StatusCode, Json<Value>) {
    let message = json!({"message": message});
    (StatusCode::BAD_REQUEST, Json(message))
}
