use anyhow::{Context, Result};
use tracing::{debug, info, warn};

// Platform-specific imports
use opencode_rs::{
    sse::SseSubscription,
    types::{
        event::Event,
        message::{Part, PartContent, PromptRequest},
        session::{CreateSessionRequest, Session},
    },
    Client as OpencodeClient,
};

pub struct OpenCodeClient {
    inner: OpencodeClient,
}

impl OpenCodeClient {
    /// Create a new client connected to the OpenCode server
    pub async fn connect(base_url: &str) -> Result<Self> {
        let client = OpencodeClient::builder()
            .base_url(base_url)
            .build()
            .context("Failed to build OpenCode client")?;

        // Verify connection by checking health
        match client.misc().health().await {
            Ok(health) => {
                debug!("OpenCode server health: {:?}", health);
            }
            Err(e) => {
                warn!("Could not check server health: {}", e);
            }
        }

        Ok(Self { inner: client })
    }

    /// Create a new session with an initial task
    pub async fn create_session(&self, task: &str) -> Result<String> {
        info!("Creating new session with task");

        let request = CreateSessionRequest {
            title: Some(format!("Runner task: {}", &task[..task.len().min(50)])),
            ..Default::default()
        };

        let session = self
            .inner
            .sessions()
            .create(&request)
            .await
            .context("Failed to create session")?;

        info!("Created session: {}", session.id);

        // Send the initial task as a prompt
        let prompt_request = PromptRequest {
            parts: vec![Part {
                part_type: "text".to_string(),
                content: PartContent::Text(task.to_string()),
            }],
            ..Default::default()
        };

        let response = self
            .inner
            .messages()
            .prompt_async(&session.id, &prompt_request)
            .await
            .context("Failed to send initial prompt")?;

        debug!("Prompt response: {:?}", response);

        Ok(session.id)
    }

    /// Subscribe to session events (SSE stream)
    pub async fn subscribe(&self, session_id: &str) -> Result<SseSubscription> {
        debug!("Subscribing to session events for {}", session_id);

        let subscription = self
            .inner
            .subscribe_session(session_id)
            .await
            .context("Failed to subscribe to session events")?;

        Ok(subscription)
    }

    /// Send a message to the session (for future feedback feature)
    pub async fn send_message(&self, session_id: &str, text: &str) -> Result<()> {
        debug!("Sending message to session {}: {}", session_id, text);

        let request = PromptRequest {
            parts: vec![Part {
                part_type: "text".to_string(),
                content: PartContent::Text(text.to_string()),
            }],
            ..Default::default()
        };

        self.inner
            .messages()
            .prompt_async(session_id, &request)
            .await
            .context("Failed to send message")?;

        Ok(())
    }

    /// Get the inner client (for advanced usage)
    pub fn inner(&self) -> &OpencodeClient {
        &self.inner
    }
}

/// Helper function to extract text content from an event
pub fn extract_event_text(event: &Event) -> Option<String> {
    match event {
        Event::PartAdded { part } => {
            if part.part_type == "text" {
                if let PartContent::Text(text) = &part.content {
                    return Some(text.clone());
                }
            }
        }
        Event::PartUpdated { delta, .. } => {
            return Some(delta.clone());
        }
        _ => {}
    }
    None
}

/// Helper function to extract tool call info from an event
pub fn extract_tool_call(event: &Event) -> Option<(String, serde_json::Value)> {
    match event {
        Event::ToolCall { name, params, .. } => Some((name.clone(), params.clone())),
        _ => None,
    }
}
