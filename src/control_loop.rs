use crate::{
    config::ControlConfig,
    client::OpenCodeClient,
    reviewer::{ReviewerClient, ReviewerContext, ReviewerDecision, ReviewerAction},
    sampler::Sampler,
    state::State,
};
use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

// Platform-specific imports
#[cfg(unix)]
use opencode_rs::sse::SseSubscription;
#[cfg(windows)]
use crate::opencode_stub::SseSubscription;

/// Result of a control loop run
#[derive(Debug)]
pub enum RunResult {
    /// Task completed successfully
    Completed,
    /// Task was aborted with reason
    Aborted(String),
    /// Maximum iterations reached
    MaxIterations,
}

/// Event sent to the TUI
#[derive(Debug, Clone)]
pub enum UiEvent {
    /// New output from worker
    WorkerOutput(String),
    /// New reviewer decision
    ReviewerDecision(ReviewerDecision),
    /// Status update
    StatusUpdate(String),
}

/// Main control loop orchestrating worker and reviewer
pub struct ControlLoop {
    client: OpenCodeClient,
    reviewer: ReviewerClient,
    sampler: Sampler,
    state: State,
    config: ControlConfig,
}

impl ControlLoop {
    /// Create a new control loop
    pub fn new(
        client: OpenCodeClient,
        reviewer: ReviewerClient,
        sampler: Sampler,
        state: State,
        config: ControlConfig,
    ) -> Self {
        Self {
            client,
            reviewer,
            sampler,
            state,
            config,
        }
    }

    /// Run the control loop
    pub async fn run(
        &mut self,
        event_sender: Option<mpsc::Sender<UiEvent>>,
    ) -> Result<RunResult> {
        info!("Starting control loop");

        // Create a session with the task
        let session_id = self
            .client
            .create_session(&self.config.task)
            .await
            .context("Failed to create session")?;

        info!("Created session: {}", session_id);

        // Subscribe to events
        let mut subscription = self
            .client
            .subscribe(&session_id)
            .await
            .context("Failed to subscribe to session events")?;

        info!("Subscribed to session events");

        // Main loop
        loop {
            // Check max iterations
            if self.state.is_max_iterations(self.config.max_iterations) {
                warn!("Maximum iterations ({}) reached", self.config.max_iterations);
                return Ok(RunResult::MaxIterations);
            }

            // Start new iteration
            self.state.start_iteration();
            let iteration = self.state.current_iteration();
            info!("Starting iteration {}/{}", iteration, self.config.max_iterations);

            // Send status update
            if let Some(ref sender) = event_sender {
                let _ = sender
                    .send(UiEvent::StatusUpdate(format!(
                        "Iteration {}/{}",
                        iteration, self.config.max_iterations
                    )))
                    .await;
            }

            // Stream events until review trigger
            match self.stream_until_review(&mut subscription, &event_sender).await {
                Ok(()) => {
                    debug!("Stream ended or timeout, proceeding to review");
                }
                Err(e) => {
                    error!("Error during streaming: {}", e);
                    // Continue to review what we have
                }
            }

            // Get the sample
            let sample = self.sampler.sample();
            let sample_size = self.sampler.line_count();
            debug!("Sample size: {} lines", sample_size);

            if sample_size == 0 {
                warn!("No output captured, waiting and retrying...");
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            // Build reviewer context
            let context = ReviewerContext {
                task_description: self.config.task.clone(),
                iteration,
                previous_summaries: self.state.get_previous_summaries(5),
                current_sample: sample,
            };

            // Call reviewer (with retry)
            let decision = self.reviewer.review_with_retry(&context).await?;
            
            // Send decision to TUI
            if let Some(ref sender) = event_sender {
                let _ = sender.send(UiEvent::ReviewerDecision(decision.clone())).await;
            }

            // Record the decision
            let retry_count = 0; // TODO: Track actual retry count
            self.state.record_decision(sample_size, decision.clone(), retry_count);

            info!(
                "Iteration {} decision: {:?} - {}",
                iteration, decision.action, decision.reason
            );

            // Handle decision
            match decision.action {
                ReviewerAction::Continue => {
                    debug!("Continuing to next iteration");
                    // Clear sampler for next iteration
                    self.sampler.clear();
                }
                ReviewerAction::Abort => {
                    info!("Aborting: {}", decision.reason);
                    return Ok(RunResult::Aborted(decision.reason));
                }
            }

            // Small delay between iterations
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Stream events until it's time to review
    async fn stream_until_review(
        &mut self,
        subscription: &mut SseSubscription,
        event_sender: &Option<mpsc::Sender<UiEvent>>,
    ) -> Result<()> {
        let start_time = Instant::now();
        let mut last_event_time = Instant::now();
        let mut event_count = 0;

        loop {
            // Check for inactivity timeout
            if last_event_time.elapsed() > self.config.inactivity_timeout {
                info!(
                    "Inactivity timeout after {:?}, triggering review",
                    self.config.inactivity_timeout
                );
                return Ok(());
            }

            // Use timeout to periodically check for inactivity
            match tokio::time::timeout(
                Duration::from_millis(100),
                subscription.recv()
            ).await {
                Ok(Some(event)) => {
                    event_count += 1;
                    last_event_time = Instant::now();

                    // Process event in sampler
                    self.sampler.process_event(&event);

                    // Send to TUI if available
                    if let Some(ref sender) = event_sender {
                        let event_text = format!("{:?}", event);
                        // Only send significant events to avoid flooding
                        if should_send_to_ui(&event) {
                            let _ = sender.send(UiEvent::WorkerOutput(event_text)).await;
                        }
                    }

                    // Check for natural completion indicators
                    if is_completion_event(&event) {
                        info!("Detected completion event");
                        return Ok(());
                    }
                }
                Ok(None) => {
                    // Stream closed
                    info!("Event stream closed");
                    return Ok(());
                }
                Err(_) => {
                    // Timeout - continue loop to check inactivity
                    continue;
                }
            }
        }
    }

    /// Get current state reference (for TUI)
    pub fn state(&self) -> &State {
        &self.state
    }
}

/// Check if an event indicates task completion
#[cfg(unix)]
fn is_completion_event(event: &opencode_rs::types::event::Event) -> bool {
    use opencode_rs::types::event::Event;
    
    match event {
        Event::SessionCompleted { .. } => true,
        Event::MessageCompleted { .. } => true,
        _ => false,
    }
}

#[cfg(windows)]
fn is_completion_event(event: &crate::opencode_stub::types::event::Event) -> bool {
    use crate::opencode_stub::types::event::Event;
    
    match event {
        Event::SessionCompleted { .. } => true,
        Event::MessageCompleted { .. } => true,
        _ => false,
    }
}

/// Check if an event should be sent to the UI
#[cfg(unix)]
fn should_send_to_ui(event: &opencode_rs::types::event::Event) -> bool {
    use opencode_rs::types::event::Event;
    
    match event {
        Event::PartAdded { .. } => true,
        Event::PartUpdated { .. } => true,
        Event::ToolCall { .. } => true,
        _ => false,
    }
}

#[cfg(windows)]
fn should_send_to_ui(event: &crate::opencode_stub::types::event::Event) -> bool {
    use crate::opencode_stub::types::event::Event;
    
    match event {
        Event::PartAdded { .. } => true,
        Event::PartUpdated { .. } => true,
        Event::ToolCall { .. } => true,
        _ => false,
    }
}
