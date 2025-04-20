use dotenvy::dotenv;
use std::env;
use redis::{AsyncCommands, Client, RedisResult};
use redis::aio::MultiplexedConnection;
use uuid::Uuid;

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

pub async fn generate_session_token(mut connection: MultiplexedConnection, forename: String) -> RedisResult<String> {
    let token = Uuid::new_v4().to_string();
    let token_key = format!("session:{}", token);

    let _: () = connection.hset_multiple(
        &token_key,
        &[("forename", forename)] 
    ).await?;
    let _: () = connection.expire(&token_key, 28800).await?;

    Ok(token_key)
}