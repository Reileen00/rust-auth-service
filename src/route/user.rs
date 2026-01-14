use axum::{extract::Extension, Json};

pub async fn me(Extension(user_id): Extension<String>) -> Json<String> {
    Json(user_id)
}
