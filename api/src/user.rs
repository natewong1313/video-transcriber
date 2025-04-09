use crate::models::AppState;
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
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
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

const UNKNOWN_ERR: (StatusCode, &str) =
    (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error occured");

async fn create(State(state): State<AppState>, Json(payload): Json<CreateUser>) -> Response {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(err) => {
            println!("Error occured hashing password: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password").into_response();
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
            None => return UNKNOWN_ERR.into_response(),
        };
        let response = match err_code {
            Some(code) if code == "2067" => {
                (StatusCode::BAD_REQUEST, "User with email already exists")
            }
            Some(_) => UNKNOWN_ERR,
            None => UNKNOWN_ERR,
        };
        return response.into_response();
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
