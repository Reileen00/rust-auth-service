use chrono::{Utc, Duration};
use jsonwebtoken::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str, secret: &str) -> String {
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    encode(
        &Header::default(),
        &Claims { sub: user_id.into(), exp },
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap()
}

pub fn validate(token: &str, secret: &str) -> String {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).unwrap();
    data.claims.sub
}
