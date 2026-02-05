use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info};

use crate::{
    control_loop::UiEvent,
    reviewer::{ReviewerAction, ReviewerDecision},
};

/// State shared between control loop and TUI
pub struct UiState {
    /// Recent worker output lines
    worker_output: Vec<String>,
    /// Activity log entries
    activity_log: Vec<String>,
    /// Current status message
    status: String,
    /// Current iteration
    iteration: usize,
    /// Max iterations
    max_iterations: usize,
    /// Whether the run is complete
    completed: bool,
    /// Final result message
    final_result: Option<String>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            worker_output: Vec::with_capacity(100),
            activity_log: Vec::new(),
            status: "Initializing...".to_string(),
            iteration: 0,
            max_iterations: 10,
            completed: false,
            final_result: None,
        }
    }

    pub fn add_worker_output(&mut self, text: String) {
        // Keep only last 100 lines
        if self.worker_output.len() >= 100 {
            self.worker_output.remove(0);
        }
        self.worker_output.push(text);
    }

    pub fn add_activity(&mut self, text: String) {
        self.activity_log.push(text);
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn set_iteration(&mut self, current: usize, max: usize) {
        self.iteration = current;
        self.max_iterations = max;
    }

    pub fn set_completed(&mut self, result: Option<String>) {
        self.completed = true;
        self.final_result = result;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

/// TUI application
pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    /// Initialize the TUI with a new state
    pub fn new() -> Result<(Self, mpsc::Sender<UiEvent>)> {
        Self::init_terminal()?;
        let (event_sender, _event_receiver) = mpsc::channel(100);

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok((Self { terminal }, event_sender))
    }

    /// Initialize from existing state
    pub fn new_from_state(_state: Arc<Mutex<UiState>>) -> Result<Self> {
        Self::init_terminal()?;

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    fn init_terminal() -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    fn cleanup_terminal() -> Result<()> {
        disable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Run the TUI event loop
    pub async fn run(&mut self, state: Arc<Mutex<UiState>>) -> Result<()> {
        let mut last_tick = tokio::time::Instant::now();
        let tick_rate = tokio::time::Duration::from_millis(250);

        loop {
            // Draw UI
            {
                let state_guard = state.lock().await;
                self.terminal.draw(|f| draw_ui(f, &state_guard))?;
            }

            // Handle input
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| tokio::time::Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let CEvent::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('c') 
                            if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            info!("User requested quit");
                            break;
                        }
                        KeyCode::Esc => {
                            info!("User pressed ESC");
                            break;
                        }
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = tokio::time::Instant::now();
            }

            // Check if completed
            {
                let state_guard = state.lock().await;
                if state_guard.completed {
                    // Show final state briefly
                    drop(state_guard);
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    break;
                }
            }
        }

        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        // Cleanup terminal
        let _ = Self::cleanup_terminal();
    }
}

/// Process UI events and update state
pub async fn process_ui_event(event: UiEvent, state: &mut UiState) {
    match event {
        UiEvent::WorkerOutput(text) => {
            state.add_worker_output(text);
        }
        UiEvent::ReviewerDecision(decision) => {
            let action_str = match decision.action {
                ReviewerAction::Continue => "Continue",
                ReviewerAction::Abort => "Abort",
            };
            state.add_activity(format!(
                "[{}] {}: {}",
                chrono::Local::now().format("%H:%M:%S"),
                action_str,
                decision.reason
            ));
            
            // Also update status
            state.set_status(format!("{}: {}", action_str, decision.reason));
        }
        UiEvent::StatusUpdate(status) => {
            state.set_status(status);
        }
    }
}

/// Draw the UI
fn draw_ui(f: &mut ratatui::Frame, state: &UiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    // Header
    draw_header(f, chunks[0], state);

    // Main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    draw_worker_output(f, main_chunks[0], state);
    draw_activity_log(f, main_chunks[1], state);

    // Footer
    draw_footer(f, chunks[2], state);
}

/// Draw the header
fn draw_header(f: &mut ratatui::Frame, area: ratatui::layout::Rect, state: &UiState) {
    let status_style = if state.completed {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    };

    let header_text = format!(
        " OpenCode Runner | Iteration {}/{} | Status: {} ",
        state.iteration, state.max_iterations, state.status
    );

    let header = Paragraph::new(header_text)
        .style(status_style)
        .block(Block::default().borders(Borders::ALL).title(" Status "));

    f.render_widget(header, area);
}

/// Draw the worker output panel
fn draw_worker_output(f: &mut ratatui::Frame, area: ratatui::layout::Rect, state: &UiState) {
    let output_text = if state.worker_output.is_empty() {
        Text::from("Waiting for output...")
    } else {
        let lines: Vec<Line> = state
            .worker_output
            .iter()
            .map(|line| {
                // Truncate long lines
                let truncated = if line.len() > 200 {
                    format!("{}...", &line[..200])
                } else {
                    line.clone()
                };
                Line::from(truncated)
            })
            .collect();
        Text::from(lines)
    };

    let output = Paragraph::new(output_text)
        .block(Block::default().borders(Borders::ALL).title(" Worker Output "))
        .wrap(Wrap { trim: true });

    f.render_widget(output, area);
}

/// Draw the activity log panel
fn draw_activity_log(f: &mut ratatui::Frame, area: ratatui::layout::Rect, state: &UiState) {
    let log_text = if state.activity_log.is_empty() {
        Text::from("No activity yet...")
    } else {
        let lines: Vec<Line> = state
            .activity_log
            .iter()
            .map(|entry| {
                let style = if entry.contains("Abort") {
                    Style::default().fg(Color::Red)
                } else if entry.contains("Continue") {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                Line::from(Span::styled(entry.clone(), style))
            })
            .collect();
        Text::from(lines)
    };

    let log = Paragraph::new(log_text)
        .block(Block::default().borders(Borders::ALL).title(" Activity Log "))
        .wrap(Wrap { trim: true });

    f.render_widget(log, area);
}

/// Draw the footer
fn draw_footer(f: &mut ratatui::Frame, area: ratatui::layout::Rect, state: &UiState) {
    let footer_text = if state.completed {
        if let Some(ref result) = state.final_result {
            format!(" {} | Press 'q' to exit ", result)
        } else {
            " Completed | Press 'q' to exit ".to_string()
        }
    } else {
        " Press 'q' or ESC to exit ".to_string()
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
