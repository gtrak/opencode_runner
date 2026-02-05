use anyhow::Result;
use std::time::Duration;

/// Configuration for the control loop
#[derive(Debug, Clone)]
pub struct ControlConfig {
    /// The task description
    pub task: String,
    /// Maximum iterations before forcing abort
    pub max_iterations: usize,
    /// Timeout for inactivity (no events)
    pub inactivity_timeout: Duration,
}

impl ControlConfig {
    /// Create a new ControlConfig from args
    pub fn new(task: String, max_iterations: usize, inactivity_timeout: Duration) -> Self {
        Self {
            task,
            max_iterations,
            inactivity_timeout,
        }
    }

    /// Create ControlConfig from CLI arguments
    pub fn from_args(task: &str, max_iterations: usize, inactivity_timeout: u64) -> Result<Self> {
        // Validate arguments
        if task.is_empty() {
            anyhow::bail!("Task cannot be empty");
        }
        if max_iterations == 0 {
            anyhow::bail!("Max iterations must be greater than 0");
        }
        if inactivity_timeout == 0 {
            anyhow::bail!("Inactivity timeout must be greater than 0");
        }

        Ok(Self::new(
            task.to_string(),
            max_iterations,
            tokio::time::Duration::from_secs(inactivity_timeout),
        ))
    }

    /// Create ControlConfig from environment variables
    pub fn from_env() -> Result<Self> {
        // Check for environment variable overrides
        let task = std::env::var("OPCODE_TASK").unwrap_or_else(|_| "".to_string());
        let max_iterations = std::env::var("OPCODE_MAX_ITERATIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);
        let inactivity_timeout = std::env::var("OPCODE_INACTIVITY_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);

        Ok(Self::new(
            task,
            max_iterations,
            tokio::time::Duration::from_secs(inactivity_timeout),
        ))
    }
}
