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
use tokio::task;
use uuid::Uuid;

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
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash.as_bytes()
    }
}

#[derive(Debug, Clone)]
struct Backend {
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
        let user: Option<Self::User> = sqlx::query_as("SELECT FROM users WHERE email = $1")
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

pub async fn login(
    State(state): State<AppState>,
    WithRejection(Valid(Json(payload)), _): WithRejection<Valid<Json<UserCredentials>>, ApiError>,
) -> Response {
    let user = User {
        id: Uuid::new_v4(),
        email: payload.email,
        password_hash: String::from(""),
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
