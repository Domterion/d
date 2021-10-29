use std::sync::Mutex;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub client: Client,
    pub sent: Mutex<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub username: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub username: String,
    pub content: String,
    pub embeds: Vec<Embed>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub author: EmbedAuthor,
    pub description: String,
    pub color: String,
    pub footer: EmbedFooter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
}
