use axum::{http::Request, middleware::Next, response::Response};
use crate::jwt::validate;

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Response {
    let token = req.headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "");

    let user_id = validate(&token, std::env::var("JWT_SECRET").unwrap().as_str());
    req.extensions_mut().insert(user_id);
    next.run(req).await
}
