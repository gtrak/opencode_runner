use anyhow::{Context, Result};
use std::path::Path;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tokio::time::{timeout, Duration};
use tracing::{debug, error, info, warn};

pub struct ServerManager {
    process: Child,
    port: u16,
    base_url: String,
}

impl ServerManager {
    /// Spawn opencode serve with a random available port
    pub async fn spawn(
        working_dir: &Path,
        model: &str,
        extra_args: &[String],
    ) -> Result<Self> {
        // Find an available port
        let port = portpicker::pick_unused_port()
            .context("Failed to find an available port")?;

        info!("Spawning opencode serve on port {}", port);

        // Build command arguments
        let mut args = vec![
            "serve".to_string(),
            "--port".to_string(),
            port.to_string(),
            "--hostname".to_string(),
            "127.0.0.1".to_string(),
            "--model".to_string(),
            model.to_string(),
        ];

        // Add extra arguments passed by user
        args.extend(extra_args.iter().cloned());

        debug!("Command: opencode {}", args.join(" "));

        // Spawn the process
        let mut process = Command::new("opencode")
            .args(&args)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn opencode serve. Is opencode installed and in PATH?")?;

        // Wait a moment for the server to start
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Check if process is still running
        match process.try_wait() {
            Ok(None) => {
                // Process is still running, good
            }
            Ok(Some(status)) => {
                return Err(anyhow::anyhow!(
                    "OpenCode server exited immediately with status: {}",
                    status
                ));
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to check OpenCode server status: {}",
                    e
                ));
            }
        }

        let base_url = format!("http://127.0.0.1:{}", port);

        // Try to verify the server is responsive
        let client = reqwest::Client::new();
        let health_url = format!("{}/health", base_url);

        let server_ready = timeout(Duration::from_secs(30), async {
            loop {
                match client.get(&health_url).send().await {
                    Ok(response) if response.status().is_success() => {
                        info!("OpenCode server is ready at {}", base_url);
                        return Ok::<(), anyhow::Error>(());
                    }
                    Ok(response) => {
                        debug!("Server responded with status: {}", response.status());
                    }
                    Err(e) => {
                        debug!("Server not yet ready: {}", e);
                    }
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        })
        .await;

        match server_ready {
            Ok(Ok(())) => {
                // Server is ready
            }
            Ok(Err(e)) => {
                let _ = process.kill().await;
                return Err(anyhow::anyhow!("Failed to verify server health: {}", e));
            }
            Err(_) => {
                let _ = process.kill().await;
                return Err(anyhow::anyhow!(
                    "Timeout waiting for OpenCode server to start"
                ));
            }
        }

        Ok(Self {
            process,
            port,
            base_url,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Gracefully shutdown the server
    pub async fn shutdown(mut self) -> Result<()> {
        info!("Shutting down OpenCode server...");

        // Try graceful shutdown first
        match self.process.kill().await {
            Ok(()) => {
                info!("OpenCode server terminated");
            }
            Err(e) => {
                warn!("Failed to kill OpenCode server: {}", e);
            }
        }

        // Wait for process to exit
        match timeout(Duration::from_secs(5), self.process.wait()).await {
            Ok(Ok(status)) => {
                debug!("OpenCode server exited with status: {}", status);
            }
            Ok(Err(e)) => {
                warn!("Error waiting for OpenCode server: {}", e);
            }
            Err(_) => {
                warn!("Timeout waiting for OpenCode server to exit");
            }
        }

        Ok(())
    }
}

impl Drop for ServerManager {
    fn drop(&mut self) {
        // Best effort cleanup
        if let Some(id) = self.process.id() {
            debug!("Cleaning up OpenCode server process {}", id);
            // Note: Can't use async in drop, so we can't wait for it
        }
    }
}
