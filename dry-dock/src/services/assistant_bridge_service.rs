// src/services/assistant_bridge_service.rs
use crate::models::ChatMessage;
use serde::{Deserialize, Serialize};
use super::log_service;

#[derive(Serialize, Deserialize, Debug)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct OllamaChatResponse {
    message: OllamaMessage,
    done: bool,
}

pub struct AssistantService;

impl AssistantService {
    const OLLAMA_API_URL: &'static str = "http://localhost:11434/api/chat";
    const DEFAULT_MODEL: &'static str = "gemma3";

    /// Send a chat message to Ollama and get a response
    pub async fn send_message(messages: &[ChatMessage]) -> Result<String, String> {
        if messages.is_empty() {
            return Err("No messages provided".to_string());
        }

        // Convert ChatMessage to OllamaMessage format
        let ollama_messages: Vec<OllamaMessage> = messages
            .iter()
            .map(|msg| OllamaMessage {
                role: msg.role.as_str().to_string(),
                content: msg.content.clone(),
            })
            .collect();

        let request = OllamaChatRequest {
            model: Self::DEFAULT_MODEL.to_string(),
            messages: ollama_messages,
            stream: false,
        };

        log_service::add_log_entry("INFO", "Sending chat request to Ollama...");

        let client = reqwest::Client::new();
        let response = client
            .post(Self::OLLAMA_API_URL)
            .json(&request)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(|e| format!("Failed to send request to Ollama: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            log_service::add_log_entry("ERROR", &format!("Ollama API error: {} - {}", status, error_text));
            return Err(format!("Ollama API error: {} - {}", status, error_text));
        }

        let chat_response: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

        log_service::add_log_entry("INFO", "Received response from Ollama");

        Ok(chat_response.message.content)
    }

    /// Check if Ollama server is available
    pub async fn check_server_status() -> bool {
        let client = reqwest::Client::new();
        client
            .get("http://localhost:11434/api/tags")
            .timeout(std::time::Duration::from_secs(2))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
