use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use axum_valid::Valid;
use serde_json::json;

use crate::{
    common::errors::ApiError,
    models::{app::AppState, user::UserCredentials},
    services::auth::AuthSession,
};

pub async fn login(
    mut auth_session: AuthSession,
    State(_): State<AppState>,
    WithRejection(Valid(Json(payload)), _): WithRejection<Valid<Json<UserCredentials>>, ApiError>,
) -> Response {
    let user = match auth_session.authenticate(payload.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            let payload = json!({"message": "Invalid credentials"});
            return (StatusCode::BAD_REQUEST, Json(payload)).into_response();
        }
        Err(err) => {
            let payload = json!({"message": err.to_string()});
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
        }
    };
    if auth_session.login(&user).await.is_err() {
        let payload = json!({"message": "Error logging in"});
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
    }
    (StatusCode::CREATED, Json(user)).into_response()
}
