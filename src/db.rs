use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect(url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .unwrap()
}
