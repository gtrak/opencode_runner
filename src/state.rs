use crate::reviewer::{ReviewerAction, ReviewerDecision};
use chrono::{DateTime, Utc};

/// Tracks the state of a control loop run
pub struct State {
    /// All completed iterations
    iterations: Vec<Iteration>,
    /// Current iteration number (1-indexed)
    current_iteration: usize,
    /// When the run started
    start_time: DateTime<Utc>,
}

/// Record of a single iteration
pub struct Iteration {
    /// Iteration number
    pub number: usize,
    /// When this iteration started
    pub timestamp: DateTime<Utc>,
    /// Number of lines in the sample
    pub sample_size: usize,
    /// The reviewer's decision
    pub decision: ReviewerDecision,
    /// How many retries were needed for the reviewer
    pub reviewer_retry_count: u8,
}

impl State {
    /// Create a new state tracker
    pub fn new() -> Self {
        Self {
            iterations: Vec::new(),
            current_iteration: 0,
            start_time: Utc::now(),
        }
    }

    /// Start a new iteration, incrementing the counter
    pub fn start_iteration(&mut self) {
        self.current_iteration += 1;
    }

    /// Get the current iteration number
    pub fn current_iteration(&self) -> usize {
        self.current_iteration
    }

    /// Record a completed iteration
    pub fn record_decision(
        &mut self,
        sample_size: usize,
        decision: ReviewerDecision,
        retry_count: u8,
    ) {
        let iteration = Iteration {
            number: self.current_iteration,
            timestamp: Utc::now(),
            sample_size,
            decision,
            reviewer_retry_count: retry_count,
        };
        self.iterations.push(iteration);
    }

    /// Get summaries of previous iterations for reviewer context
    pub fn get_previous_summaries(&self, count: usize) -> Vec<String> {
        self.iterations
            .iter()
            .rev()
            .take(count)
            .map(|iter| {
                format!(
                    "Iteration {} ({} lines): {:?} - {}",
                    iter.number, iter.sample_size, iter.decision.action, iter.decision.reason
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    /// Check if max iterations has been reached
    pub fn is_max_iterations(&self, max: usize) -> bool {
        self.current_iteration >= max
    }

    /// Get the total runtime
    pub fn runtime(&self) -> chrono::Duration {
        Utc::now() - self.start_time
    }

    /// Get all iterations
    pub fn iterations(&self) -> &[Iteration] {
        &self.iterations
    }

    /// Get the last iteration if any
    pub fn last_iteration(&self) -> Option<&Iteration> {
        self.iterations.last()
    }

    /// Get the start time
    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    /// Generate a formatted activity log for display
    pub fn format_activity_log(&self) -> String {
        if self.iterations.is_empty() {
            return "No iterations yet".to_string();
        }

        self.iterations
            .iter()
            .map(|iter| {
                let action_str = match iter.decision.action {
                    ReviewerAction::Continue => "✓ Continue",
                    ReviewerAction::Abort => "✗ Abort",
                };
                format!(
                    "[{}] Iter {}/{}: {} - {} ({} lines, {} retries)",
                    iter.timestamp.format("%H:%M:%S"),
                    iter.number,
                    self.current_iteration,
                    action_str,
                    iter.decision.reason,
                    iter.sample_size,
                    iter.reviewer_retry_count
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get a summary of the current status
    pub fn status_summary(&self) -> String {
        if self.iterations.is_empty() {
            return "Initializing...".to_string();
        }

        let last = self.iterations.last().unwrap();
        match last.decision.action {
            ReviewerAction::Continue => format!(
                "Iteration {} - Continuing: {}",
                self.current_iteration, last.decision.reason
            ),
            ReviewerAction::Abort => format!(
                "Iteration {} - Abort: {}",
                self.current_iteration, last.decision.reason
            ),
        }
    }

    /// Count total lines sampled across all iterations
    pub fn total_lines_sampled(&self) -> usize {
        self.iterations.iter().map(|i| i.sample_size).sum()
    }

    /// Count total retries
    pub fn total_retries(&self) -> u32 {
        self.iterations
            .iter()
            .map(|i| i.reviewer_retry_count as u32)
            .sum()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_new() {
        let state = State::new();
        assert_eq!(state.current_iteration(), 0);
        assert!(state.iterations().is_empty());
    }

    #[test]
    fn test_iteration_tracking() {
        let mut state = State::new();

        state.start_iteration();
        assert_eq!(state.current_iteration(), 1);

        state.record_decision(
            50,
            ReviewerDecision {
                action: ReviewerAction::Continue,
                reason: "Good progress".to_string(),
            },
            0,
        );

        assert_eq!(state.iterations().len(), 1);
        assert_eq!(state.total_lines_sampled(), 50);

        let summaries = state.get_previous_summaries(10);
        assert_eq!(summaries.len(), 1);
    }

    #[test]
    fn test_max_iterations() {
        let mut state = State::new();

        state.start_iteration();
        state.start_iteration();
        state.start_iteration();

        assert!(!state.is_max_iterations(5));
        assert!(state.is_max_iterations(3));
        assert!(state.is_max_iterations(2));
    }
}
