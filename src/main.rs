mod db;
mod config;
mod routes;
mod models;
mod crypto;
mod jwt;
mod redis;
mod middleware;
mod openapi;

use axum::{Router, routing::*, middleware as axum_mw};
use dotenvy::dotenv;
use config::Config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cfg = Config::from_env();
    let db = db::connect(&cfg.database_url).await;

    let app = Router::new()
        .route("/register", post(routes::auth::register))
        .route("/login", post(routes::auth::login))
        .route("/me", get(routes::user::me))
        .layer(axum_mw::from_fn(middleware::auth::auth));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
