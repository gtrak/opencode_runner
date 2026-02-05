use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, warn};

mod client;
mod config;
mod control_loop;
mod environment;
mod reviewer;
mod sampler;
mod server;
mod state;

#[cfg(feature = "tui")]
mod tui;

use client::OpenCodeClient;
use config::ControlConfig;
use control_loop::{ControlLoop, RunResult};
use environment::load_config_from_env;
use reviewer::ReviewerClient;
use sampler::Sampler;
use server::ServerManager;
use state::State;

#[derive(Parser, Debug)]
#[command(name = "opencode_runner")]
#[command(about = "Control loop for OpenCode agent execution with review")]
struct Args {
    /// Task description for the worker
    #[arg(short, long)]
    task: String,

    /// Working directory for the task
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,

    /// Model for the worker (e.g., "ollama/llama3.1")
    #[arg(long, default_value = "ollama/llama3.1")]
    worker_model: String,

    /// OpenAI-compatible API URL for reviewer
    #[arg(long, default_value = "http://localhost:11434/v1")]
    reviewer_url: String,

    /// Model for the reviewer
    #[arg(long, default_value = "ollama/llama3.1")]
    reviewer_model: String,

    /// Maximum iterations before forcing abort
    #[arg(long, default_value = "10")]
    max_iterations: usize,

    /// Inactivity timeout in seconds
    #[arg(long, default_value = "30")]
    inactivity_timeout: u64,

    /// Run without TUI (headless mode)
    #[arg(long)]
    headless: bool,

    /// Additional arguments passed to `opencode serve`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    extra_args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    info!("Starting OpenCode Runner");
    info!("Task: {}", args.task);
    info!("Working directory: {}", args.working_dir.display());
    info!("Worker model: {}", args.worker_model);
    info!("Reviewer URL: {}", args.reviewer_url);
    info!("Max iterations: {}", args.max_iterations);

    // Spawn the OpenCode server
    info!("Spawning OpenCode server...");
    let server =
        ServerManager::spawn(&args.working_dir, &args.worker_model, &args.extra_args).await?;

    info!("Server spawned on port {}", server.port());

    // Give the server a moment to fully initialize
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Connect to the server
    info!("Connecting to OpenCode server at {}...", server.base_url());
    let client = OpenCodeClient::connect(server.base_url()).await?;
    info!("Connected to OpenCode server");

    // Create components
    let reviewer = ReviewerClient::new(args.reviewer_url, args.reviewer_model);

    let sampler = Sampler::new(100);
    let state = State::new();

    // Create control loop configuration
    let config =
        ControlConfig::from_args(&args.task, args.max_iterations, args.inactivity_timeout)?;

    // Create control loop
    let mut control_loop = ControlLoop::new(client, reviewer, sampler, state, config);

    // Run in TUI or headless mode
    let result = if args.headless {
        info!("Running in headless mode");
        control_loop.run(None).await
    } else {
        #[cfg(feature = "tui")]
        {
            info!("Starting TUI");
            run_tui_mode(control_loop).await
        }
        #[cfg(not(feature = "tui"))]
        {
            warn!("TUI feature not enabled, falling back to headless mode");
            control_loop.run(None).await
        }
    };

    // Cleanup
    info!("Shutting down server...");
    server.shutdown().await?;

    // Print result
    match result {
        Ok(RunResult::Completed) => {
            info!("Task completed successfully");
            Ok(())
        }
        Ok(RunResult::Aborted(reason)) => {
            warn!("Task aborted: {}", reason);
            std::process::exit(1);
        }
        Ok(RunResult::MaxIterations) => {
            warn!("Task reached maximum iterations");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "tui")]
async fn run_tui_mode(mut control_loop: ControlLoop) -> Result<RunResult> {
    use control_loop::UiEvent;
    use std::sync::Arc;
    use tokio::sync::{mpsc, Mutex};

    let (event_sender, mut event_receiver) = mpsc::channel(100);
    let ui_state = Arc::new(Mutex::new(tui::UiState::new()));

    // Run control loop
    let control_handle = {
        let ui_state = ui_state.clone();
        tokio::spawn(async move {
            let result = control_loop.run(Some(event_sender)).await;

            // Update final state
            if let Ok(ref run_result) = result {
                let mut state = ui_state.lock().await;
                match run_result {
                    RunResult::Completed => {
                        state.set_status("Completed".to_string());
                        state.set_completed(Some("Task completed".to_string()));
                    }
                    RunResult::Aborted(reason) => {
                        state.set_status(format!("Aborted: {}", reason));
                        state.set_completed(Some(format!("Aborted: {}", reason)));
                    }
                    RunResult::MaxIterations => {
                        state.set_status("Max iterations reached".to_string());
                        state.set_completed(Some("Max iterations reached".to_string()));
                    }
                }
            }

            result
        })
    };

    // Run TUI in parallel with event processing
    let ui_handle = {
        let ui_state = ui_state.clone();
        tokio::spawn(async move {
            // Process events from control loop
            let event_processor = {
                let ui_state = ui_state.clone();
                tokio::spawn(async move {
                    while let Some(event) = event_receiver.recv().await {
                        let mut state = ui_state.lock().await;
                        tui::process_ui_event(event, &mut state).await;
                    }
                })
            };

            // Run TUI
            let tui_result = {
                let mut tui = tui::Tui::new_from_state(ui_state.clone())?;
                tui.run(ui_state).await
            };

            // Wait for event processor to finish
            let _ = event_processor.await;

            tui_result
        })
    };

    // Wait for control loop to finish
    let result = control_handle.await??;

    // Give TUI a moment to render final state
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Cleanup
    let _ = ui_handle.await;

    Ok(result)
}
