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
use sqlx::types::Uuid;

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
        let err_code = match err.as_database_error() {
            Some(err) => err.code(),
            None => {
                return make_internal_err("Unknown error occured creating user").into_response();
            }
        };
        let response = match err_code {
            Some(code) if code == "2067" => make_user_err("User with email already exists"),
            Some(code) => make_internal_err(&format!("Unknown error occured: {code}")),
            None => make_internal_err("Unknown error occured creating user"),
        };
        return response.into_response();
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
