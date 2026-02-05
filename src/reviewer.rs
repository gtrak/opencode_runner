use anyhow::{Context, Result};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

/// Decision from the reviewer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewerDecision {
    pub action: ReviewerAction,
    pub reason: String,
}

/// Possible reviewer actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReviewerAction {
    /// Worker is making progress, continue
    Continue,
    /// Worker is stuck or looping, abort
    Abort,
}

/// Context provided to the reviewer
pub struct ReviewerContext {
    /// The original task description
    pub task_description: String,
    /// Current iteration number
    pub iteration: usize,
    /// Previous reviewer summaries
    pub previous_summaries: Vec<String>,
    /// Current sample of worker output (last 100 lines)
    pub current_sample: String,
}

/// Client for the reviewer API (OpenAI-compatible)
pub struct ReviewerClient {
    pub http_client: HttpClient,
    pub base_url: String,
    pub model: String,
    pub max_retries: u8,
}

/// OpenAI-compatible chat message
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI-compatible request
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

/// OpenAI-compatible response
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

impl ReviewerClient {
    /// Create a new reviewer client
    pub fn new(base_url: String, model: String) -> Self {
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            base_url,
            model,
            max_retries: 3,
        }
    }

    /// Review progress with exponential backoff retry
    /// Returns Continue if all retries fail
    pub async fn review_with_retry(&self, context: &ReviewerContext) -> Result<ReviewerDecision> {
        for attempt in 0..self.max_retries {
            match self.review(context).await {
                Ok(decision) => {
                    if attempt > 0 {
                        info!(
                            "Reviewer succeeded after {} retries",
                            attempt
                        );
                    }
                    return Ok(decision);
                }
                Err(e) => {
                    let delay = Duration::from_secs(2u64.pow(attempt as u32));
                    warn!(
                        "Reviewer failed (attempt {}): {}, retrying in {:?}",
                        attempt + 1,
                        e,
                        delay
                    );
                    sleep(delay).await;
                }
            }
        }

        // Default to Continue after max retries
        error!(
            "Reviewer failed after {} retries, defaulting to Continue",
            self.max_retries
        );
        Ok(ReviewerDecision {
            action: ReviewerAction::Continue,
            reason: format!(
                "Reviewer API unavailable after {} retries, continuing based on last known state",
                self.max_retries
            ),
        })
    }

    /// Single review attempt
    async fn review(&self, context: &ReviewerContext) -> Result<ReviewerDecision> {
        let prompt = self.build_prompt(context);

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a progress monitoring assistant. Analyze the AI assistant's work and determine if it is making progress or stuck in a loop.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            response_format: Some(ResponseFormat {
                format_type: "json_object".to_string(),
            }),
        };

        debug!("Sending review request to {}", self.base_url);

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send review request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "Reviewer API returned error {}: {}",
                status,
                text
            ));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse reviewer response")?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .context("No choices in reviewer response")?;

        debug!("Reviewer raw response: {}", content);

        // Parse the JSON response
        let decision: ReviewerDecision = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse reviewer decision from: {}", content))?;

        info!(
            "Reviewer decision: {:?} - {}",
            decision.action, decision.reason
        );

        Ok(decision)
    }

    /// Build the prompt for the reviewer
    pub fn build_prompt(&self, context: &ReviewerContext) -> String {
        let previous_summaries = if context.previous_summaries.is_empty() {
            "No previous assessments.".to_string()
        } else {
            context
                .previous_summaries
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{}. {}", i + 1, s))
                .collect::<Vec<_>>()
                .join("\n")
        };

        format!(
            r#"You are monitoring an AI assistant's progress on a task.

Task: {}

Current iteration: {}

Previous progress assessments:
{}

Current output (last {} lines):
```
{}
```

Assess whether the assistant is:
1. Making meaningful progress (continue) - the assistant is generating code, making changes, or working toward the goal
2. Stuck in a loop or not progressing (abort) - the assistant is repeating itself, going in circles, or clearly failing to make progress

Respond with JSON in this exact format:
{{
  "action": "continue|abort",
  "reason": "Brief explanation of your assessment"
}}"#,
            context.task_description,
            context.iteration,
            previous_summaries,
            context.current_sample.lines().count(),
            context.current_sample
        )
    }

    /// Get a summary string for the activity log
    pub fn format_decision_summary(decision: &ReviewerDecision, iteration: usize) -> String {
        format!(
            "Iter {}: {:?} - {}",
            iteration,
            decision.action,
            decision.reason
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decision() {
        let json = r#"{"action": "continue", "reason": "Making progress"}"#;
        let decision: ReviewerDecision = serde_json::from_str(json).unwrap();
        
        match decision.action {
            ReviewerAction::Continue => {}
            _ => panic!("Expected Continue"),
        }
        assert_eq!(decision.reason, "Making progress");
    }

    #[test]
    fn test_parse_abort() {
        let json = r#"{"action": "abort", "reason": "Stuck in loop"}"#;
        let decision: ReviewerDecision = serde_json::from_str(json).unwrap();
        
        match decision.action {
            ReviewerAction::Abort => {}
            _ => panic!("Expected Abort"),
        }
    }
}
