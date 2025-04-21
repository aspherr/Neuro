use dotenvy::dotenv;
use std::env;
use redis::{AsyncCommands, Client, RedisResult};
use redis::aio::MultiplexedConnection;
use uuid::Uuid;
use super::models::User;
use std::collections::HashMap;

fn get_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("Missing URL for Redis Connection")
}

fn get_client() -> RedisResult<Client> {
    Client::open(get_url())
}

pub async fn conn() -> redis::RedisResult<MultiplexedConnection> {
    let client = get_client()?;
    client.get_multiplexed_async_connection().await
}

pub async fn generate_session_token(mut connection: MultiplexedConnection, user_id: String) -> RedisResult<String> {
    let token = Uuid::new_v4().to_string();
    let token_key = format!("session:{}", token);

    let _: () = connection.hset_multiple(
        &token_key,
        &[("user_id", user_id)] 
    ).await?;
    let _: () = connection.expire(&token_key, 28800).await?;

    Ok(token_key)
}

pub async fn get_user_session_data(session_token: String) -> RedisResult<User> {
    let mut connection = conn().await?;

    let user_id: Option<String> = connection.hget(&session_token, "user_id").await?;

    let user_data: HashMap<String, String> = connection.hgetall(&user_id).await?;

    let account = User::assign(
        user_data.get("forename").cloned().unwrap(),
        user_data.get("email").cloned().unwrap(),
        user_data.get("password").cloned().unwrap()
    );  

    Ok(account)
}  