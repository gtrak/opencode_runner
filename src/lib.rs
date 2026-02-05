// Library exports for testing
// This allows tests to import modules from src/

pub mod client;
pub mod config;
pub mod control_loop;
pub mod environment;
pub mod reviewer;
pub mod sampler;
pub mod server;
pub mod state;

#[cfg(feature = "tui")]
pub mod tui;

pub use client::OpenCodeClient;
pub use config::ControlConfig;
pub use control_loop::{ControlLoop, RunResult};
pub use environment::load_config_from_env;
pub use reviewer::{ReviewerAction, ReviewerClient, ReviewerContext, ReviewerDecision};
pub use sampler::{Sampler, SamplerEvent};
pub use server::ServerManager;
pub use state::State;
