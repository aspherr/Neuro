use redis::{AsyncCommands, RedisResult};
use super::client::conn;


pub async fn create_user(forename: String, email: String, password: String) -> RedisResult<String> {
    let mut connection = conn().await?;

    let id: i64 = connection.incr("user:counter", 1).await?;
    let key = format!("user:{}", id);
    
    let _: () = connection.hset_multiple(
        &key, 
        &[("name", forename), ("email", email), ("password", password)]
    ).await?;

    Ok(key)
}