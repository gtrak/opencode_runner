use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::ControlConfig;

/// Load configuration from environment variables
pub fn load_config_from_env() -> Result<ControlConfig> {
    let task = std::env::var("OPCODE_TASK").unwrap_or_else(|_| "".to_string());
    let working_dir = std::env::var("OPCODE_WORKING_DIR")
        .unwrap_or_else(|_| ".".to_string())
        .parse::<PathBuf>()?;
    let worker_model =
        std::env::var("OPCODE_WORKER_MODEL").unwrap_or_else(|_| "ollama/llama3.1".to_string());
    let reviewer_url = std::env::var("OPCODE_REVIEWER_URL")
        .unwrap_or_else(|_| "http://localhost:11434/v1".to_string());
    let reviewer_model =
        std::env::var("OPCODE_REVIEWER_MODEL").unwrap_or_else(|_| "ollama/llama3.1".to_string());
    let max_iterations = std::env::var("OPCODE_MAX_ITERATIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10);
    let inactivity_timeout = std::env::var("OPCODE_INACTIVITY_TIMEOUT")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .unwrap_or(30);
    let headless = std::env::var("OPCODE_HEADLESS")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false);

    // Validate required environment variables
    if task.is_empty() {
        anyhow::bail!("OPCODE_TASK environment variable is required");
    }

    Ok(ControlConfig {
        task,
        max_iterations,
        inactivity_timeout: std::time::Duration::from_secs(inactivity_timeout),
    })
}

/// Validate that all required environment variables are set
pub fn validate_env_vars() -> Result<()> {
    let task =
        std::env::var("OPCODE_TASK").context("OPCODE_TASK environment variable is required")?;
    if task.is_empty() {
        anyhow::bail!("OPCODE_TASK cannot be empty");
    }

    Ok(())
}

/// Get environment variable or fallback to default
pub fn get_env_with_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Get environment variable as integer with fallback
pub fn get_env_with_default_int(key: &str, default: i64) -> i64 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// Get environment variable as boolean with fallback
pub fn get_env_with_default_bool(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .and_then(|s| match s.to_lowercase().as_str() {
            "1" | "true" | "yes" => Some(true),
            "0" | "false" | "no" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}
