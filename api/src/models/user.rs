use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserCredentials {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 64))]
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    // don't return pw hash in a response
    #[serde(skip_serializing)]
    pub password_hash: String,
}
