//! Stub module for opencode_rs on Windows
//! 
//! This module provides type stubs that allow the project to compile on Windows.
//! For actual operation, use Linux or macOS.

use std::future::Future;
use std::pin::Pin;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, OpencodeError>;

#[derive(Debug, thiserror::Error)]
pub enum OpencodeError {
    #[error("OpenCode error: {0}")]
    Message(String),
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Not supported on Windows")]
    NotSupported,
}

#[derive(Clone)]
pub struct Client;

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder
    }

    pub fn sessions(&self) -> SessionsApi {
        SessionsApi
    }

    pub fn messages(&self) -> MessagesApi {
        MessagesApi
    }

    pub fn misc(&self) -> MiscApi {
        MiscApi
    }

    pub async fn subscribe_session(&self, _session_id: &str) -> Result<SseSubscription> {
        Err(OpencodeError::NotSupported)
    }

    pub fn sse_subscriber(&self) -> SseSubscriber {
        SseSubscriber
    }
}

pub struct ClientBuilder;

impl ClientBuilder {
    pub fn base_url(self, _url: &str) -> Self {
        self
    }

    pub fn build(self) -> Result<Client> {
        Ok(Client)
    }
}

pub struct SessionsApi;

impl SessionsApi {
    pub async fn create(&self, _req: &types::session::CreateSessionRequest) -> Result<types::session::Session> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn get(&self, _id: &str) -> Result<types::session::Session> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn list(&self) -> Result<Vec<types::session::Session>> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn delete(&self, _id: &str) -> Result<()> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn abort(&self, _id: &str) -> Result<()> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn status(&self) -> Result<types::session::SessionStatus> {
        Err(OpencodeError::NotSupported)
    }
}

pub struct MessagesApi;

impl MessagesApi {
    pub async fn prompt(&self, _session_id: &str, _req: &types::message::PromptRequest) -> Result<types::api::PromptResponse> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn prompt_async(&self, _session_id: &str, _req: &types::message::PromptRequest) -> Result<types::api::PromptResponse> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn list(&self, _session_id: &str) -> Result<Vec<types::message::Message>> {
        Err(OpencodeError::NotSupported)
    }

    pub async fn get(&self, _session_id: &str, _message_id: &str) -> Result<types::message::Message> {
        Err(OpencodeError::NotSupported)
    }
}

pub struct MiscApi;

impl MiscApi {
    pub async fn health(&self) -> Result<types::misc::HealthResponse> {
        Err(OpencodeError::NotSupported)
    }
}

pub struct SseSubscription;

impl SseSubscription {
    pub async fn recv(&mut self) -> Option<types::event::Event> {
        None
    }

    pub fn close(&self) {}
}

pub struct SseSubscriber;

impl SseSubscriber {
    pub async fn subscribe(&self) -> Result<SseSubscription> {
        Err(OpencodeError::NotSupported)
    }
}

pub mod types {
    pub mod session {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Session {
            pub id: String,
            pub title: Option<String>,
            pub created_at: String,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct CreateSessionRequest {
            pub title: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SessionStatus {
            pub active: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateSessionRequest;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SummarizeRequest;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RevertRequest;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SessionDiff;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TodoItem;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ShareInfo;
    }

    pub mod message {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Message {
            pub id: String,
            pub role: String,
            pub content: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PromptRequest {
            pub parts: Vec<Part>,
        }

        impl Default for PromptRequest {
            fn default() -> Self {
                Self { parts: Vec::new() }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Part {
            #[serde(rename = "type")]
            pub part_type: String,
            pub content: PartContent,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum PartContent {
            Text(String),
            Object(serde_json::Value),
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CommandRequest;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ShellRequest;
    }

    pub mod api {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PromptResponse {
            pub message_id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CommandResponse;

        pub type ShellResponse = String;
    }

    pub mod event {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(tag = "type")]
        pub enum Event {
            #[serde(rename = "part_added")]
            PartAdded { part: super::message::Part },
            
            #[serde(rename = "part_updated")]
            PartUpdated { delta: String },
            
            #[serde(rename = "tool_call")]
            ToolCall { name: String, params: serde_json::Value },
            
            #[serde(rename = "tool_result")]
            ToolResult { result: serde_json::Value },
            
            #[serde(rename = "error")]
            Error { error: String },
            
            #[serde(rename = "thinking")]
            Thinking { content: String },
            
            #[serde(rename = "progress")]
            Progress { message: String },
            
            #[serde(rename = "message_completed")]
            MessageCompleted { message_id: String },
            
            #[serde(rename = "session_completed")]
            SessionCompleted { session_id: String },
        }
    }

    pub mod misc {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct HealthResponse {
            pub healthy: bool,
            pub version: String,
        }
    }
}

pub use types::*;
