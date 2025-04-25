use crate::{
    common::errors::{ApiError, make_internal_err, make_user_err},
    models::{
        app::AppState,
        user::{User, UserCredentials},
    },
};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use axum_valid::Valid;
use serde_json::json;

pub async fn create(
    State(state): State<AppState>,
    WithRejection(Valid(Json(payload)), _): WithRejection<Valid<Json<UserCredentials>>, ApiError>,
) -> Response {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(_) => {
            return make_internal_err("Failed to hash password").into_response();
        }
    };

    let mut user = User {
        id: -1,
        email: payload.email,
        password_hash: password_hash.to_string(),
    };

    let user_id: i64 = match sqlx::query_scalar(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
    )
    .bind(&user.email)
    .bind(&user.password_hash)
    .fetch_one(&state.pool)
    .await
    {
        Ok(id) => id,
        Err(err) => {
            let (status, message) = match err.as_database_error() {
                Some(err) => match err.code() {
                    Some(code) if code == "2067" => (
                        StatusCode::BAD_REQUEST,
                        "User with email already exists".to_string(),
                    ),
                    Some(code) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        std::format!("Unknown error occured: {}", code),
                    ),
                    None => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Unknown error occured".to_string(),
                    ),
                },
                None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unknown error occured".to_string(),
                ),
            };
            return (status, Json(json!({"message": message}))).into_response();
        }
    };

    user.id = user_id;
    (StatusCode::CREATED, Json(user)).into_response()
}
