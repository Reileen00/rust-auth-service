use axum::{Json, extract::State};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{models::*, crypto::*, jwt::*, redis::*};

pub async fn register(State(db): State<PgPool>, Json(p): Json<RegisterPayload>) {
    let hash = hash_password(&p.password);
    sqlx::query("INSERT INTO users VALUES ($1,$2,$3)")
        .bind(Uuid::new_v4())
        .bind(p.email)
        .bind(hash)
        .execute(&db)
        .await.unwrap();
}

pub async fn login(State(db): State<PgPool>, Json(p): Json<LoginPayload>) -> Json<String> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE email=$1")
        .bind(p.email)
        .fetch_one(&db)
        .await.unwrap();

    if !verify_password(&p.password, &user.password_hash) {
        panic!("invalid");
    }

    Json(create_jwt(&user.id.to_string(), &std::env::var("JWT_SECRET").unwrap()))
}
