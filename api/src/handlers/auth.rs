use std::collections::HashMap;

use async_trait::async_trait;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use axum_login::{AuthUser, AuthnBackend, UserId};
use axum_valid::Valid;
use password_auth::verify_password;
use serde_json::json;
use tokio::task;

use crate::{
    common::errors::ApiError,
    models::{
        app::AppState,
        user::{User, UserCredentials},
    },
};

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash.as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    state: AppState,
}

impl Backend {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = UserCredentials;
    type Error = AuthError;

    async fn authenticate(
        &self,
        UserCredentials { email, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.state.pool)
            .await?;
        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(password, &user.password_hash).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("SELECT * FROM users where id = $1")
            .bind(user_id)
            .fetch_optional(&self.state.pool)
            .await?;
        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn login(
    mut auth_session: AuthSession,
    State(state): State<AppState>,
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
