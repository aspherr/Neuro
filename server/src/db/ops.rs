use redis::{AsyncCommands, RedisError, ErrorKind, RedisResult};
use std::collections::HashMap;
use super::client::conn;
use super::client::generate_session_token;
use super::models::User;
use uuid::Uuid;

// CREATE USER
pub async fn create_user(forename: String, email: String, password: String) -> RedisResult<String> {
    let mut connection = conn().await?;

    let email_key = format!("user:{}", email);
    let id = Uuid::new_v4().to_string();

    // create STRING
    let _: () = connection.set(&email_key, &id).await?;

    let id_key = format!("user:{}", &id);

    // create HASH 
    let _: () = connection.hset_multiple(
        &id_key, 
        &[("forename", forename), ("email", email), ("password", password)]
    ).await?;

    Ok(id_key)
}

// GET USER
pub async fn get_user(email: String, password: String) -> RedisResult<String> {
    let mut connection = conn().await?;

    let email_key: String = format!("user:{}", email);
    let search_key: String = connection.get(email_key).await?;
    
    let user_id: String = format!("user:{}", search_key);
    
    // get HASH
    let user_data: HashMap<String, String> = connection.hgetall(&user_id).await?;
    
    // assign hash to struct
    let account = User::assign(
        user_data.get("forename").cloned().unwrap(),
        user_data.get("email").cloned().unwrap(),
        user_data.get("password").cloned().unwrap()
    );  

    // remove dead code warning (temp)
    let _ = &account.get_email();

    if account.verify_password(&password) {
        let session_user = format!("session:{}", search_key);
        // get STRING
        let existing_session:  Option<String> = connection.get(session_user).await?;
        
        if let Some(session) = existing_session {
            let session = format!("session:{}", session);
            return Ok(session)
        }

        // create HASH
        let session = generate_session_token(connection, search_key).await?;
        Ok(session)
        
    } else {
        Err(RedisError::from((ErrorKind::TypeError, "Invalid Credentials")))
    }
}

// CREATE VAULT
pub async fn create_vault(name: String, user_id: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let id: String = Uuid::new_v4().to_string();
    let vault_key: String = format!("vault:{}", id);
    let vault_set_key: String = format!("vault:{}", user_id);

    let vault_name_key: String = format!("vault:{}", name);
    
    // create STRING
    let _: () = connection.set(vault_name_key, &id).await?;

    // create HASH
    let _: () = connection.hset_multiple(
        &vault_key, 
        &[("name", name), ("user_id", user_id)]
    ).await?;

    // create SET
    let _: () = connection.sadd(&vault_set_key, &id).await?;

    Ok(vault_key)
}

// GET VAULTS
pub async fn get_vaults(user_id: String) -> RedisResult<Vec<String>> {
    let mut connection = conn().await?;
    
    let vault_set_key: String = format!("vault:{}", user_id);
    
    // get SET
    let vault_keys: Vec<String> = connection.smembers(&vault_set_key).await?;

    let mut vault_names: Vec<String> = Vec::new();
    // extract id's
    for id in vault_keys {
        let id_key = format!("vault:{}", id);
        let name = connection.hget(id_key, "name").await?;
        vault_names.push(name);
    }

    Ok(vault_names)
}

// GET VAULT ID
pub async fn get_vault_id(name: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let vault_name_key: String = format!("vault:{}", name);
    // get STRING
    let vault_id: String = connection.get(&vault_name_key).await?;

    Ok(vault_id)
}


// DELETE VAULT
pub async fn delete_vault(vault_id: String, vault_name: String, user_id: String) -> RedisResult<()> {
    let mut connection = conn().await?;
    
    // delete HASH
    let hash_key = format!("vault:{}", &vault_id);
    let _: () = connection.del(&hash_key).await?;

    // delete STRING
    let string_key = format!("vault:{}", &vault_name);
    let _: () = connection.del(string_key).await?;

    // delete SET
    let set_key = format!("vault:{}", &user_id);
    let _: () = connection.srem(set_key,&vault_id).await?;

    Ok(())
}

// CREATE NOTEBOOK
pub async fn create_notebook(name: String, vault_id: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let id: String = Uuid::new_v4().to_string();
    let notebook_id: String = format!("notebook:{}", id);
    let notebook_set_key: String = format!("notebook:{}", vault_id);

    let notebook_name_key: String = format!("notebook:{}", name);

    // create STRING
    let _: () = connection.set(&notebook_name_key, &id).await?;

    // create HASH
    let _: () = connection.hset_multiple(
        &notebook_id, 
        &[("name", name), ("vault_id", vault_id)]
    ).await?;

    // create SET
    let _: () = connection.sadd(&notebook_set_key, &id).await?;

    Ok(notebook_id)
}

// GET NOTEBOOKS
pub async fn get_notebooks(vault_id: String) -> RedisResult<Vec<String>> {
    let mut connection = conn().await?;
    
    let notebook_set_key: String = format!("notebook:{}", vault_id);

    // get SET
    let notebook_keys: Vec<String> = connection.smembers(&notebook_set_key).await?;

    let mut notebook_names: Vec<String> = Vec::new();
    // extract ids
    for id in notebook_keys {
        let id_key = format!("notebook:{}", id);
        let name = connection.hget(id_key, "name").await?;
        notebook_names.push(name);
    }

    Ok(notebook_names)
}

// GET NOTEBOOK ID
pub async fn get_notebook_id(name: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let notebook_name_key: String = format!("notebook:{}", name);

    // get STRING
    let notebook_id: String = connection.get(&notebook_name_key).await?;

    Ok(notebook_id)
}

// DELETE NOTEBOOK
pub async fn delete_notebook(notebook_id: String, notebook_name: String, vault_id: String) -> RedisResult<()> {
    let mut connection = conn().await?;

    // delete HASH
    let hash_key = format!("notebook:{}", &notebook_id);
    let _: () = connection.del(hash_key).await?;

    // delete STRING
    let string_key = format!("notebook:{}", &notebook_name);
    let _: () = connection.del(string_key).await?;
    
    // delete SET
    let set_key = format!("notebook:{}", &vault_id);
    let _: () = connection.srem(set_key,&notebook_id).await?;

    Ok(())
}
 
// CREATE NOTE
pub async fn create_note(name: String, notebook_id: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let id: String = Uuid::new_v4().to_string();
    let note_key: String = format!("note:{}", id);
    let note_set_key: String = format!("note:{}", notebook_id);

    let note_name_key: String = format!("note:{}", name);

    // create STRING
    let _: () = connection.set(note_name_key, &id).await?;

    // create HASH
    let _: () = connection.hset_multiple(
        &note_key, 
        &[("name", name), ("notebook_id", notebook_id), ("content", "".to_string())]
    ).await?;

    // create SET
    let _: () = connection.sadd(note_set_key, &id).await?;

    Ok(note_key)
}

// GET NOTE
pub async fn get_notes(notebook_id: String) -> RedisResult<Vec<String>> {
    let mut connection = conn().await?;
    
    let notebook_set_key: String = format!("note:{}", notebook_id);

    // get SET
    let notebook_keys: Vec<String> = connection.smembers(&notebook_set_key).await?;

    let mut notebook_names: Vec<String> = Vec::new();
    // extract ids
    for id in notebook_keys {
        let id_key = format!("note:{}", id);
        let name = connection.hget(id_key, "name").await?;
        notebook_names.push(name);
    }

    Ok(notebook_names)
}

// GET NOTE ID
pub async fn get_note_id(name: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let notebook_name_key: String = format!("note:{}", name);
    // get STRING
    let notebook_id: String = connection.get(&notebook_name_key).await?;

    Ok(notebook_id)
}

// GET NOTE contents
pub async fn read_note(id: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let notebook_name_key: String = format!("note:{}", id);
    
    // get HASH
    let content: String = connection.hget(&notebook_name_key, "content").await?;

    Ok(content)
}

// WRITE TO NOTE
pub async fn save_note(id: String, content: String) -> RedisResult<String> {
    let mut connection = conn().await?;
    
    let notebook_name_key: String = format!("note:{}", id);
    
    // set HASH
    let content: String = connection.hset(&notebook_name_key, "content", content).await?;

    Ok(content)
}

// DELETE NOTE
pub async fn delete_note(note_id: String, note_name: String, notebook_id: String) -> RedisResult<()> {
    let mut connection = conn().await?;
    
    // delete HASH
    let hash_key = format!("note:{}", &note_id);
    let _: () = connection.del(hash_key).await?;

    // delete STRING
    let string_key = format!("note:{}", &note_name);
    let _: () = connection.del(string_key).await?;

    // delete SET    
    let set_key = format!("note:{}", &notebook_id);
    let _: () = connection.srem(set_key,&note_id).await?;

    Ok(())
}
