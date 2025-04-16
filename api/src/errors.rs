use axum::{
    Json,
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::JsonExtractorRejection(rejection) => {
                (rejection.status(), rejection.body_text())
            }
        };

        let payload = json!({"message": msg, "origin": "with_rejection"});
        (status, Json(payload)).into_response()
    }
}
