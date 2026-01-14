use std::env;

pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap(),
            redis_url: env::var("REDIS_URL").unwrap(),
            jwt_secret: env::var("JWT_SECRET").unwrap(),
        }
    }
}
