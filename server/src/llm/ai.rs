use reqwest::Client;
use dotenvy_macro::dotenv;

// Generate a client
fn gen_client() -> Client {
    reqwest::Client::new()
}

// Grabs OpenAI API key from .env file
fn get_openai_key() -> String {
    dotenv!("OPENAI_API_KEY").to_string()
}

// API call to GPT-4o-mini model
pub async fn call_neuro(prompt: &str) -> Result<String, String> {
    // HTTP POST to OpenAI
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

    // This bottom code to parse the response was produced by AI
    let body = response.text().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("⚠️ Could not extract message content")
        .to_string();

    Ok(content)
}
