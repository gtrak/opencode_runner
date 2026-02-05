use opencode_rs::types::event::Event;
use opencode_rs::types::message::Part;
use std::collections::VecDeque;
use tracing::trace;

// Mock event type for testing
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
            // Capture text content from message part updates
            Event::MessagePartUpdated { properties } => {
                // Capture text from part content
                if let Some(ref part) = properties.part {
                    if let Part::Text { text, .. } = part {
                        self.add_lines(text);
                    }
                }
                // Capture delta updates
                if let Some(ref delta) = properties.delta {
                    self.add_lines(delta);
                }
            }

            // Capture tool commands
            Event::CommandExecuted { properties } => {
                // Extract tool name from command
                // Check if properties has a command field or if command is embedded
                let props_str = match serde_json::to_string(properties) {
                    Ok(s) if !s.is_empty() => s,
                    _ => {
                        let summary = format!("[Tool: unknown]");
                        self.add_line(&summary);
                        return;
                    }
                };

                // Try to deserialize the command directly
                let command_str = if let Ok(cmd) = serde_json::from_str::<String>(&props_str) {
                    cmd
                } else if let Ok(obj) = serde_json::from_str::<serde_json::Value>(&props_str) {
                    obj.get("command")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                } else {
                    "unknown".to_string()
                };
                let summary = format!("[Tool: {}]", command_str);
                self.add_line(&summary);
            }

            // Capture error events
            Event::SessionError { properties } => {
                if let Some(ref error) = properties.error {
                    let error_line = format!("[Error: {:?}]", error);
                    self.add_line(&error_line);
                }
            }

            // Skip: tool results/outputs (too verbose)
            Event::SessionCompacted { .. } | Event::SessionStatus { .. } => {
                trace!("Skipping verbose event");
            }

            // Skip: thinking/reasoning content
            Event::PtyUpdated { .. } => {
                trace!("Skipping PTY update");
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
    pub fn add_lines(&mut self, text: &str) {
        for line in text.lines() {
            self.add_line(line);
        }
    }

    /// Add a single line to the buffer
    pub fn add_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            // Remove oldest line if at capacity
            if self.buffer.len() >= self.max_lines {
                self.buffer.pop_front();
            }
            self.buffer.push_back(trimmed.to_string());
        }
    }

    /// Test-only method to process SamplerEvent for unit tests
    #[cfg(test)]
    pub fn process_sampler_event(&mut self, event: SamplerEvent) {
        match event {
            SamplerEvent::PartAdded { text } => {
                self.add_lines(&text);
            }
            SamplerEvent::PartUpdated { delta } => {
                self.add_lines(&delta);
            }
            SamplerEvent::ToolCall { name, params } => {
                let summary = format!(
                    "[Tool: {}({})]",
                    name,
                    serde_json::to_string(&params).unwrap_or_else(|_| "{}".to_string())
                );
                self.add_line(&summary);
            }
            SamplerEvent::ToolResult { .. } => {
                // Skip tool results in tests
            }
            SamplerEvent::Error { error } => {
                let error_line = format!("[Error: {}]", error);
                self.add_line(&error_line);
            }
            SamplerEvent::Thinking { .. } => {
                // Skip thinking in tests
            }
        }
    }
}
