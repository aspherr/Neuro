use dotenvy_macro::dotenv;
use redis::{AsyncCommands, Client, RedisResult};
use redis::aio::MultiplexedConnection;
use uuid::Uuid;
use super::models::User;
use std::collections::HashMap;

// grab DB url from .env file
fn get_url() -> String {
    dotenv!("DATABASE_URL").to_string()
}

// generates a new (cloneable) redis client
fn get_client() -> RedisResult<Client> {
    Client::open(get_url())
}

// Starts connection
pub async fn conn() -> redis::RedisResult<MultiplexedConnection> {
    let client = get_client()?;
    client.get_multiplexed_async_connection().await
}

// gets user session token based on connection
pub async fn generate_session_token(mut connection: MultiplexedConnection, user_id: String) -> RedisResult<String> {
    let token = Uuid::new_v4().to_string();

    let token_key = format!("session:{}", token);
    let user_key = format!("session:{}", &user_id);
    
    // create STRING
    let _: () = connection.set(&user_key, &token).await?;
    // set EXPIRY
    let _: () = connection.expire(&user_key, 28800).await?;

    // create HASH
    let _: () = connection.hset_multiple(
        &token_key,
        &[("user_id", user_id)] 
    ).await?;
    // set EXPIRY
    let _: () = connection.expire(&token_key, 28800).await?;

    Ok(token_key)
}

pub async fn get_user_session_data(session_token: String) -> RedisResult<User> {
    let mut connection = conn().await?;

    let id: String = connection.hget(&session_token, "user_id").await?;

    let user_id = format!("user:{}", id);

    // get HASH
    let user_data: HashMap<String, String> = connection.hgetall(&user_id).await?;

    // assign hash to struct
    let account = User::assign(
        user_data.get("forename").cloned().unwrap(),
        user_data.get("email").cloned().unwrap(),
        user_data.get("password").cloned().unwrap()
    );  

    Ok(account)
}

pub async fn get_user_id(email: String) -> RedisResult<String> {
    let mut connection = conn().await?;

    let email_key: String = format!("user:{}", email);

    // get STRING
    let user_id: String = connection.get(email_key).await?;
    
    Ok(user_id)
}

pub async fn delete_session(token: String, user_id: String) -> RedisResult<bool> {
    let mut connection = conn().await?;

    let user_key = format!("session:{}", &user_id);

    // delete STRING
    let _: () = connection.del(&user_key).await?;
    let _: () = connection.del(&token).await?;
    
    Ok(true)
}   