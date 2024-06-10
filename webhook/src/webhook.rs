use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{fmt, error};

/// Custom error type for webhook errors
#[derive(Debug)]
pub enum WebhookError {
    Reqwest(reqwest::Error),
    InvalidResponse(String),
    MissingField(String),
}

impl fmt::Display for WebhookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebhookError::Reqwest(e) => write!(f, "Request error: {}", e),
            WebhookError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            WebhookError::MissingField(field) => write!(f, "Missing required field: {}", field),
        }
    }
}

impl error::Error for WebhookError {}

impl From<reqwest::Error> for WebhookError {
    fn from(err: reqwest::Error) -> WebhookError {
        WebhookError::Reqwest(err)
    }
}

/// Config for the webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub webhook_url: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub content: Option<String>,
}

impl WebhookConfig {
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            username: None,
            avatar_url: None,
            content: None,
        }
    }
}

/// Sends a message to a Discord webhook asynchronously.
pub async fn send_discord_webhook(config: &WebhookConfig) -> Result<(), WebhookError> {
    if config.webhook_url.is_empty() {
        return Err(WebhookError::MissingField("webhook_url".to_string()));
    }

    let mut json_payload = json!({
        "content": config.content.clone().unwrap_or_default(),
    });

    if let Some(username) = &config.username {
        json_payload["username"] = json!(username);
    }

    if let Some(avatar_url) = &config.avatar_url {
        json_payload["avatar_url"] = json!(avatar_url);
    }

    let client = Client::new();
    let response = client.post(&config.webhook_url)
        .json(&json_payload)
        .send()
        .await?;

    if response.status().is_success() {
        //println!("Message sent to webhook");
        Ok(())
    } else {
        let status = response.status();
        let error_msg = format!("Failed to send message: HTTP {}", status);
        Err(WebhookError::InvalidResponse(error_msg))
    }
}

// Path: webhook/src/webhook.rs
