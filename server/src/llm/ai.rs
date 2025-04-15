use reqwest::Client;
use dotenvy::dotenv;
use std::env;

fn gen_client() -> Client {
    reqwest::Client::new()
}

fn get_openai_key() -> String {
    dotenv().ok();
    env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY")
}

pub async fn call_neuro(prompt: &str) -> Result<String, String> {
    let response = gen_client()
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(get_openai_key())
        .json(&serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [{
                "role": "user",
                "content": prompt
            }]
        }))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let body = response.text().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("⚠️ Could not extract message content")
        .to_string();

    Ok(content)
}
