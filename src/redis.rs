use redis::AsyncCommands;

pub async fn store_refresh(client: &redis::Client, token: &str, user_id: &str) {
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con.set_ex(token, user_id, 60 * 60 * 24 * 7).await.unwrap();
}

pub async fn get_user(client: &redis::Client, token: &str) -> Option<String> {
    let mut con = client.get_async_connection().await.unwrap();
    con.get(token).await.ok()
}

pub async fn revoke(client: &redis::Client, token: &str) {
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con.del(token).await.unwrap();
}
