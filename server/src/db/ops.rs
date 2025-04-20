use redis::{AsyncCommands, RedisError, ErrorKind, RedisResult};
use std::collections::HashMap;
use super::client::conn;
use super::client::generate_session_token;
use super::models::User;
use uuid::Uuid;

pub async fn create_user(forename: String, email: String, password: String) -> RedisResult<String> {
    let mut connection = conn().await?;

    let email_key = format!("user:{}", email);
    let id = Uuid::new_v4().to_string();
    let _: () = connection.set(&email_key, &id).await?;

    let id_key = format!("user:{}", &id);
    let _: () = connection.hset_multiple(
        &id_key, 
        &[("forename", forename), ("email", email), ("password", password)]
    ).await?;

    Ok(id_key)
}

pub async fn get_user(email: String, password: String) -> RedisResult<String> {
    let mut connection = conn().await?;

    let email_key: String = format!("user:{}", email);
    let search_key: String = connection.get(email_key).await?;
    
    let user_id: String = format!("user:{}", search_key);
    let user_data: HashMap<String, String> = connection.hgetall(&user_id).await?;

    let account = User::assign(
        user_data.get("forename").cloned().unwrap(),
        user_data.get("email").cloned().unwrap(),
        user_data.get("password").cloned().unwrap()
    );  

    // remove dead code warning (temp)
    let _ = &account.email;

    if account.verify_password(&password) {
        let session = generate_session_token(connection, account.forename).await?;
        Ok(session)
        
    } else {
        Err(RedisError::from((ErrorKind::TypeError, "Invalid Credentials")))
    }
}
