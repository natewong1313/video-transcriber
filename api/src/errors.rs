use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_valid::{ValidRejection, ValidationRejection};
use serde_json::{Value, json};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Valid(#[from] ValidRejection<JsonRejection>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Valid(rejection) => handle_validation_rejection(rejection),
        }
    }
}

fn handle_validation_rejection(
    rejection: ValidationRejection<ValidationErrors, JsonRejection>,
) -> Response {
    match rejection {
        ValidationRejection::Valid(validation_errors) => {
            let field_errors = validation_errors.field_errors();
            let invalid_fields: Vec<&str> = field_errors.keys().map(|cow| cow.as_ref()).collect();
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Validation error", "fields": invalid_fields})),
            )
                .into_response()
        }
        ValidationRejection::Inner(rejection) => {
            let payload = json!({"message": rejection.body_text()});
            (rejection.status(), Json(payload)).into_response()
        }
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
