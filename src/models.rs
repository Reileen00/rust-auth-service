use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}
#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
