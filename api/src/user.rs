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

async fn create(State(state): State<AppState>, Json(payload): Json<CreateUser>) -> Response {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };

    let user = User {
        id: Uuid::new_v4(),
        email: payload.email,
        password_hash: password_hash.to_string(),
    };

    let user_id = user.id.to_string();

    match sqlx::query!(
        r#"
    INSERT INTO users (id, email, password_hash)
    VALUES ( ?1, ?2, ?3 )
    "#,
        user_id,
        user.email,
        user.password_hash
    )
    .execute(&state.pool)
    .await
    {
        Ok(res) => println!("{}", res.rows_affected()),
        Err(err) => {
            // let err = err.into_database_error();
            return (StatusCode::BAD_REQUEST, "User with email already exists").into_response();
        }
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
