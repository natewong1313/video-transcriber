use crate::models::AppState;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
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

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap();

    let user = User {
        id: Uuid::new_v4(),
        email: payload.email,
        password_hash: password_hash.to_string(),
    };

    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id VARCHAR(36) PRIMARY KEY NOT NULL, email TEXT NOT NULL, password_hash TEXT NOT NULL);").execute(&state.pool).await.unwrap();
    println!("create table result: {:?}", result);

    (StatusCode::CREATED, Json(user))
}
