use std::collections::VecDeque;
use tracing::trace;
use opencode_rs::types::event::Event;

// Mock event type for testing
#[cfg(test)]
#[derive(Debug, Clone)]
pub enum SamplerEvent {
    PartAdded {
        text: String,
    },
    PartUpdated {
        delta: String,
    },
    ToolCall {
        name: String,
        params: serde_json::Value,
    },
    ToolResult {
        result: serde_json::Value,
    },
    Error {
        error: String,
    },
    Thinking {
        thought: String,
    },
}

/// Sampler that captures and buffers worker output
/// Keeps only the last N lines for review
pub struct Sampler {
    buffer: VecDeque<String>,
    max_lines: usize,
}

impl Sampler {
    /// Create a new sampler with specified max lines
    pub fn new(max_lines: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_lines),
            max_lines,
        }
    }

    /// Process an event from the SSE stream
    /// Captures text content and tool calls, skips tool outputs and thinking
    pub fn process_event(&mut self, event: &Event) {
        match event {
            // Capture text content
            Event::PartAdded { part } if part.part_type == "text" => {
                if let Some(text) = extract_text_content(&part.content) {
                    self.add_lines(&text);
                }
            }

            // Capture text updates (deltas)
            Event::PartUpdated { delta, .. } => {
                self.add_lines(delta);
            }

            // Capture tool invocations (but not their outputs)
            Event::ToolCall { name, params, .. } => {
                let summary = format!(
                    "[Tool: {}({})]",
                    name,
                    serde_json::to_string(params).unwrap_or_else(|_| "{}".to_string())
                );
                self.add_line(&summary);
            }

            // Skip: tool results/outputs (too verbose)
            Event::ToolResult { .. } => {
                trace!("Skipping tool result (too verbose)");
            }

            // Skip: error events (will be logged separately)
            Event::Error { error, .. } => {
                let error_line = format!("[Error: {}]", error);
                self.add_line(&error_line);
            }

            // Skip: thinking/reasoning content
            Event::Thinking { .. } => {
                trace!("Skipping thinking content");
            }

            // Skip: progress events (too noisy)
            Event::Progress { .. } => {
                trace!("Skipping progress event");
            }

            // Skip: other events
            _ => {
                trace!("Skipping event: {:?}", event);
            }
        }
    }

    /// Get the current sample (all buffered lines)
    pub fn sample(&self) -> String {
        self.buffer.iter().cloned().collect::<Vec<_>>().join("\n")
    }

    /// Get the number of lines in the buffer
    pub fn line_count(&self) -> usize {
        self.buffer.len()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Add lines from text content
    fn add_lines(&mut self, text: &str) {
        for line in text.lines() {
            self.add_line(line);
        }
    }

    /// Add a single line to the buffer
    fn add_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            // Remove oldest line if at capacity
            if self.buffer.len() >= self.max_lines {
                self.buffer.pop_front();
            }
            self.buffer.push_back(trimmed.to_string());
        }
    }
}

/// Extract text content from part content
#[cfg(unix)]
fn extract_text_content(content: &opencode_rs::types::message::PartContent) -> Option<String> {
    use opencode_rs::types::message::PartContent;

    match content {
        PartContent::Text(text) => Some(text.clone()),
        _ => None,
    }
}

#[cfg(windows)]
fn extract_text_content(
    content: &crate::opencode_stub::types::message::PartContent,
) -> Option<String> {
    use crate::opencode_stub::types::message::PartContent;

    match content {
        PartContent::Text(text) => Some(text.clone()),
