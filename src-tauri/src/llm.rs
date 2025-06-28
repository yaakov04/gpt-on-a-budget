use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait LLM: Send + Sync {
    async fn chat(&self, messages: Vec<Message>) -> Result<Message, String>;
    async fn stream_chat(&self, messages: Vec<Message>) -> Result<tokio::sync::mpsc::Receiver<Message>, String>;
}

pub struct OpenAIProvider {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LLM for OpenAIProvider {
    async fn chat(&self, messages: Vec<Message>) -> Result<Message, String> {
        let url = "https://api.openai.com/v1/chat/completions";
        let mut body = serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": messages,
        });

        let response = self.client.post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let response_json: serde_json::Value = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;

        let content = response_json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        let role = response_json["choices"][0]["message"]["role"].as_str().unwrap_or("assistant").to_string();

        Ok(Message { role, content })
    }

    async fn stream_chat(&self, _messages: Vec<Message>) -> Result<tokio::sync::mpsc::Receiver<Message>, String> {
        // TODO: Implement streaming for OpenAI
        Err("Streaming not yet implemented for OpenAI".to_string())
    }
}
