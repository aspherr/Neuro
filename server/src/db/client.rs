use dotenvy::dotenv;
use std::env;
use redis::{AsyncCommands, Client, RedisResult};
use redis::aio::MultiplexedConnection;

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
