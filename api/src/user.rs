use crate::{errors::ApiError, models::AppState};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    email: String,
    password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    id: Uuid,
    email: String,
    password_hash: String,
}

pub fn user_router(state: AppState) -> Router {
    let router = Router::new().route("/", post(create));
    Router::new().nest("/users", router).with_state(state)
}

async fn create(
    State(state): State<AppState>,
    WithRejection(Json(payload), _): WithRejection<Json<CreateUser>, ApiError>,
) -> Response {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to hash password"})),
            )
                .into_response();
        }
    };

    let user = User {
        id: Uuid::new_v4(),
        email: payload.email,
        password_hash: password_hash.to_string(),
    };

    if let Err(err) =
        sqlx::query("INSERT INTO USERS (id, email, password_hash) VALUES ($1, $2, $3)")
            .bind(user.id.to_string())
            .bind(&user.email)
            .bind(&user.password_hash)
            .execute(&state.pool)
            .await
    {
        println!("error occured creating user: {}", err);
        let err_code = match err.as_database_error() {
            Some(err) => err.code(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Unknown error occured"})),
                )
                    .into_response();
            }
        };
        let response = match err_code {
            Some(code) if code == "2067" => (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "User with email already exists"})),
            ),
            Some(code) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": format!("Unknown error occured: {}", code)})),
            ),
            None => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Unknown error occured"})),
            ),
        };
        return response.into_response();
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
