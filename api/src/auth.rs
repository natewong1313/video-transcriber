use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use axum_extra::extract::WithRejection;
use axum_valid::Valid;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::{AppState, User, UserCredentials},
};

pub fn auth_router(state: AppState) -> Router {
    let router = Router::new().route("/login", post(login));
    Router::new().nest("/auth", router).with_state(state)
}

async fn login(
    State(state): State<AppState>,
    WithRejection(Valid(Json(payload)), _): WithRejection<Valid<Json<UserCredentials>>, ApiError>,
) -> Response {
    let user = User {
        id: Uuid::new_v4(),
        email: String::from(""),
        password_hash: String::from(""),
    };
    (StatusCode::CREATED, Json(user)).into_response()
}
