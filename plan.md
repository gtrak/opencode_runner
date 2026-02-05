# OpenCode Runner - Implementation Plan

## Overview

A control loop system that spawns an OpenCode server, communicates with it using `opencode_rs`, and manages task execution with periodic review by an external OpenAI-compatible agent API. The system detects looping and completion, displaying progress in a TUI or headless mode.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           OpenCode Runner                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐     ┌──────────────┐     ┌─────────────────────────────┐   │
│  │   CLI Args  │────▶│ Spawn Server │────▶│  opencode serve --port ...  │   │
│  └─────────────┘     └──────────────┘     └─────────────────────────────┘   │
│                                                   │                          │
│                                                   ▼                          │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │                        Control Loop                                 │    │
│  │  ┌────────────┐    ┌────────────┐    ┌────────────┐    ┌─────────┐ │    │
│  │  │   Create   │───▶│   Stream   │───▶│   Sample   │───▶│ Review  │ │    │
│  │  │  Session   │    │   Events   │    │   Output   │    │  (API)  │ │    │
│  │  └────────────┘    └────────────┘    └────────────┘    └────┬────┘ │    │
│  │                     ▲                                       │      │    │
│  │                     │            ┌──────────────────────────┘      │    │
│  │                     │            │         (retry w/ backoff)      │    │
│  │                     │            ▼                                 │    │
│  │              ┌────────────┐   ┌────────────┐                       │    │
│  │              │  Continue  │◀──│   Parse    │                       │    │
│  │              │   (loop)   │   │  Decision  │                       │    │
│  │              └────────────┘   └─────┬──────┘                       │    │
│  │                                     │                              │    │
│  │                              ┌──────┴──────┐                       │    │
│  │                              ▼             ▼                       │    │
│  │                        ┌────────┐    ┌────────┐                    │    │
│  │                        │ Abort  │    │ Max    │                    │    │
│  │                        │        │    │ Iter   │                    │    │
│  │                        └────────┘    └────────┘                    │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                              │                                               │
│                              ▼                                               │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │                           TUI / Logs                                │    │
│  │  • Worker output stream                                           │    │
│  │  • Activity log (reviewer decisions)                              │    │
│  │  • Current status                                                 │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components

### 1. CLI (`main.rs`)

**Purpose**: Parse command-line arguments and initialize the system.

**Arguments**:
```rust
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
```

**Responsibilities**:
- Parse and validate arguments
- Initialize logging/tracing
- Spawn server process
- Start control loop (TUI or headless)
- Handle shutdown signals

### 2. Server Manager (`server.rs`)

**Purpose**: Manage the lifecycle of the OpenCode server process.

**Interface**:
```rust
struct ServerManager {
    process: Child,
    port: u16,
    base_url: String,
}

impl ServerManager {
    /// Spawn opencode serve with random port
    async fn spawn(
        working_dir: &Path,
        model: &str,
        extra_args: &[String],
    ) -> Result<Self>;
    
    /// Wait for server to be ready (health check)
    async fn wait_for_ready(&self, timeout: Duration) -> Result<()>;
    
    /// Gracefully shutdown the server
    async fn shutdown(self) -> Result<()>;
    
    /// Get base URL for client connections
    fn base_url(&self) -> &str;
}
```

**Implementation Details**:
- Use `portpicker` to find an available port
- Spawn: `opencode serve --port {port} --hostname 127.0.0.1 --model {model} {extra_args...}`
- Capture stdout to detect "Server ready" or parse port from logs
- Set working directory with `current_dir()`
- Implement graceful shutdown on drop

### 3. OpenCode Client (`client.rs`)

**Purpose**: Wrapper around `opencode_rs` for simplified interaction.

**Interface**:
```rust
struct OpenCodeClient {
    inner: opencode_rs::Client,
    session_id: String,
}

impl OpenCodeClient {
    /// Create client and connect to server
    async fn connect(base_url: &str) -> Result<Self>;
    
    /// Create a new session with initial task
    async fn create_session(&self, task: &str) -> Result<String>;
    
    /// Subscribe to session events (SSE stream)
    async fn subscribe(&self, session_id: &str) -> Result<SseSubscription>;
    
    /// Send a message to the session (for future feedback feature)
    async fn send_message(&self, session_id: &str, text: &str) -> Result<()>;
}
```

**Implementation Details**:
- Use `Client::builder()` with server URL
- Create session via `client.sessions().create()`
- Subscribe via `client.subscribe_session(session_id)`
- Handle authentication if needed (from env vars)

### 4. Sampler (`sampler.rs`)

**Purpose**: Filter and buffer worker output, keeping last 100 lines.

**Interface**:
```rust
struct Sampler {
    buffer: VecDeque<String>,
    max_lines: usize,
}

impl Sampler {
    fn new(max_lines: usize) -> Self;
    
    /// Process an event from the SSE stream
    fn process_event(&mut self, event: &Event);
    
    /// Get the current sample (last N lines)
    fn sample(&self) -> String;
    
    /// Clear the buffer
    fn clear(&mut self);
}
```

**Event Filtering Rules**:

| Event Type | Capture? | Notes |
|------------|----------|-------|
| `PartAdded` (text) | ✅ Yes | Main content |
| `PartUpdated` (text) | ✅ Yes | Content updates |
| `ToolCall` | ✅ Yes | Invocation only: `[Tool: name(params)]` |
| `ToolResult` | ❌ No | Too verbose, skip outputs |
| `Thinking` / `Reasoning` | ❌ No | Skip internal reasoning |
| `System` messages | ❌ No | Skip system events |
| `Error` events | ✅ Yes | Error messages |

**Implementation**:
```rust
fn process_event(&mut self, event: &Event) {
    match event {
        Event::PartAdded { part } if part.part_type == "text" => {
            self.add_lines(&part.text);
        }
        Event::PartUpdated { delta } => {
            self.add_lines(delta);
        }
        Event::ToolCall { name, params, .. } => {
            let summary = format!("[Tool: {}({})]", name, 
                serde_json::to_string(params).unwrap_or_default());
            self.add_line(&summary);
        }
        _ => {} // Skip other events
    }
}
```

### 5. Reviewer Client (`reviewer.rs`)

**Purpose**: Interface with OpenAI-compatible API for progress assessment.

**Interface**:
```rust
struct ReviewerClient {
    http_client: reqwest::Client,
    base_url: String,
    model: String,
    max_retries: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReviewerDecision {
    action: ReviewerAction,
    reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ReviewerAction {
    Continue,
    Abort,
}

impl ReviewerClient {
    fn new(base_url: String, model: String) -> Self;
    
    /// Review progress with exponential backoff retry
    async fn review_with_retry(
        &self, 
        context: &ReviewerContext
    ) -> Result<ReviewerDecision>;
    
    /// Single review attempt
    async fn review(&self, context: &ReviewerContext) -> Result<ReviewerDecision>;
}

struct ReviewerContext {
    task_description: String,
    iteration: usize,
    previous_summaries: Vec<String>,
    current_sample: String,
}
```

**Retry Logic**:
```rust
async fn review_with_retry(&self, context: &ReviewerContext) -> Result<ReviewerDecision> {
    for attempt in 0..self.max_retries {
        match self.review(context).await {
            Ok(decision) => return Ok(decision),
            Err(e) => {
                let delay = Duration::from_secs(2u64.pow(attempt as u32));
                tracing::warn!(
                    "Reviewer failed (attempt {}): {}, retrying in {:?}",
                    attempt + 1, e, delay
                );
                tokio::time::sleep(delay).await;
            }
        }
    }
    
    // Default to Continue after max retries
    tracing::error!("Reviewer failed after {} retries, defaulting to Continue", self.max_retries);
    Ok(ReviewerDecision {
        action: ReviewerAction::Continue,
        reason: "Reviewer unavailable, continuing based on last known state".to_string(),
    })
}
```

**Prompt Template**:
```
You are monitoring an AI assistant's progress on a task.

Task: {task_description}

Current iteration: {iteration}

Previous progress assessments:
{previous_summaries}

Current output (last 100 lines):
{current_sample}

Assess whether the assistant is:
1. Making meaningful progress (continue)
2. Stuck in a loop or not progressing (abort)

Respond with JSON:
{
  "action": "continue|abort",
  "reason": "Brief explanation of your assessment"
}
```

**API Request**:
```json
{
  "model": "llama3.1",
  "messages": [
    {"role": "system", "content": "You are a progress monitoring assistant."},
    {"role": "user", "content": "<prompt above>"}
  ],
  "response_format": {"type": "json_object"}
}
```

### 6. State Management (`state.rs`)

**Purpose**: Track activity log and iteration state.

**Interface**:
```rust
struct State {
    iterations: Vec<Iteration>,
    current_iteration: usize,
    start_time: DateTime<Utc>,
}

struct Iteration {
    number: usize,
    timestamp: DateTime<Utc>,
    sample_size: usize,           // Lines in sample
    decision: ReviewerDecision,
    reviewer_retry_count: u8,
}

impl State {
    fn new() -> Self;
    
    /// Start a new iteration
    fn start_iteration(&mut self);
    
    /// Record reviewer decision
    fn record_decision(&mut self, decision: ReviewerDecision, retries: u8);
    
    /// Get summaries of previous iterations for context
    fn get_previous_summaries(&self, count: usize) -> Vec<String>;
    
    /// Check if max iterations reached
    fn is_max_iterations(&self, max: usize) -> bool;
    
    /// Generate activity log for display
    fn format_activity_log(&self) -> String;
}
```

### 7. Control Loop (`control_loop.rs`)

**Purpose**: Main orchestration logic.

**Flow**:
```
1. Connect to server
2. Create session with task
3. Subscribe to events
4. Initialize sampler
5. Start event loop:
   a. Stream events until:
      - Inactivity timeout (no events for N seconds)
      - Session completes (detected via event type)
   b. Sample output from buffer
   c. Call reviewer API (with retry)
   d. Record decision in state
   e. If Abort → exit loop
   f. If Continue → continue streaming
6. Cleanup and exit
```

**Interface**:
```rust
struct ControlLoop {
    client: OpenCodeClient,
    reviewer: ReviewerClient,
    sampler: Sampler,
    state: State,
    config: ControlConfig,
}

struct ControlConfig {
    task: String,
    max_iterations: usize,
    inactivity_timeout: Duration,
}

impl ControlLoop {
    async fn run(&mut self, event_sender: Option<mpsc::Sender<UiEvent>>) -> Result<RunResult>;
}

enum RunResult {
    Completed,      // Task completed successfully
    Aborted(String), // Aborted with reason
    MaxIterations,  // Hit iteration limit
}
```

**Event Streaming Logic**:
```rust
async fn stream_until_review(&mut self, subscription: &mut SseSubscription) -> Result<()> {
    let mut last_event_time = Instant::now();
    let mut activity_detected = false;
    
    loop {
        tokio::select! {
            event = subscription.recv() => {
                match event {
                    Some(Event::MessageCompleted) => {
                        // Natural break point for review
                        return Ok(());
                    }
                    Some(event) => {
                        self.sampler.process_event(&event);
                        last_event_time = Instant::now();
                        activity_detected = true;
                        
                        // Send to TUI if available
                        if let Some(ref sender) = self.event_sender {
                            let _ = sender.send(UiEvent::WorkerOutput(
                                format!("{:?}", event)
                            )).await;
                        }
                    }
                    None => {
                        // Stream closed
                        return Ok(());
                    }
                }
            }
            _ = tokio::time::sleep(self.config.inactivity_timeout) => {
                if activity_detected && last_event_time.elapsed() >= self.config.inactivity_timeout {
                    // Inactivity timeout
                    return Ok(());
                }
            }
        }
    }
}
```

### 8. TUI (`tui/`)

**Purpose**: Real-time display of worker output and system status.

**Layout**:
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ 🖥️  OpenCode Runner    Iter: 3/10    Status: RUNNING    Uptime: 00:05:23   │
├─────────────────────────────┬────────────────────────────────────────────────┤
│                             │                                                │
│  💬 Worker Output           │  📊 Activity Log                               │
│  ───────────────            │  ────────────────                              │
│                             │                                                │
│  > Initializing...          │  • [00:01:12] Iter 1: Started task             │
│  > Analyzing requirements   │  • [00:02:45] Iter 2: Progress detected        │
│  > Generating solution...   │  • [00:04:30] Iter 3: Reviewing...             │
│                             │                                                │
│  fn sort(arr: &mut [i32]) { │  Last Review: 5s ago                           │
│      for i in 0..arr.len() {│  Action: Continue                              │
│          for j in 0..arr    │  Reason: Code generation in progress           │
│                             │                                                │
│  [scrolling: last 100 lines]│  ─────────────────────────────                 │
│                             │  Reviewer: POST /v1/chat/completions           │
│                             │  Model: llama3.1                               │
│                             │                                                │
├─────────────────────────────┴────────────────────────────────────────────────┤
│ ⚡ Worker: ollama/llama3.1 @ http://127.0.0.1:54892    Press 'q' to quit      │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Implementation**:
```rust
// tui/mod.rs
pub mod app;
pub mod ui;

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    events: mpsc::Receiver<UiEvent>,
}

pub enum UiEvent {
    WorkerOutput(String),
    ReviewerDecision(ReviewerDecision),
    StatusUpdate(String),
    Tick,
}

impl Tui {
    pub fn new() -> Result<(Self, mpsc::Sender<UiEvent>)>;
    pub async fn run(&mut self, state: Arc<Mutex<UiState>>) -> Result<()>;
}

pub struct UiState {
    worker_output: Vec<String>,
    activity_log: Vec<String>,
    current_status: String,
    iteration: usize,
    max_iterations: usize,
}
```

## Data Flow

```
User Input
    │
    ▼
┌──────────┐
│   CLI    │
└────┬─────┘
     │
     ▼
┌──────────────────┐
│  Server Manager  │──▶ Spawn `opencode serve`
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Control Loop    │
└────┬─────────────┘
     │
     ├──▶ Create Session ──▶ opencode_rs
     │                           │
     ├──▶ Subscribe SSE ◀────────┘
     │      │
     │      ▼
     │  ┌──────────┐
     │  │  Events  │──▶ Sampler (filter & buffer)
     │  └────┬─────┘         │
     │       │                ▼
     │       │          ┌──────────┐
     │       │          │  Sample  │──▶ Reviewer Context
     │       │          └──────────┘
     │       │                │
     │       │                ▼
     │       │          ┌──────────┐
     │       │          │ Reviewer │──▶ OpenAI API (w/ retry)
     │       │          └────┬─────┘
     │       │               │
     │       │               ▼
     │       │         ┌───────────┐
     │       │         │ Decision  │──▶ State (activity log)
     │       │         └─────┬─────┘
     │       │               │
     │       │         ┌─────┴─────┐
     │       │         ▼           ▼
     │       │    Continue      Abort
     │       │         │           │
     │       │         ▼           ▼
     │       │    (loop)       Cleanup
     │       │
     ▼       ▼
┌──────────────────┐
│  TUI / Logs      │◀─── UiEvent channel
└──────────────────┘
```

## Error Handling Strategy

### Critical Errors (Abort)
- Server fails to start
- Cannot connect to OpenCode server
- Session creation fails

### Recoverable Errors (Retry with Backoff)
- Reviewer API timeout/failure
- Network transient errors

### Default Behaviors
| Scenario | Default Action |
|----------|---------------|
| Reviewer fails after retries | Continue with warning |
| SSE stream disconnects | Attempt reconnect |
| Inactivity timeout | Trigger review |
| Max iterations reached | Abort with log |

## Dependencies

```toml
[package]
name = "opencode_runner"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core async runtime
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"

# OpenCode SDK
opencode_rs = "0.1"

# HTTP client for reviewer
reqwest = { version = "0.12", features = ["json"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# CLI parsing
clap = { version = "4.4", features = ["derive"] }

# TUI (optional)
ratatui = { version = "0.29", optional = true }
crossterm = { version = "0.28", optional = true }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging/tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Time handling
chrono = "0.4"

# Utilities
portpicker = "0.1"

[features]
default = ["tui"]
tui = ["ratatui", "crossterm"]
headless = []
```

## Configuration

### Environment Variables
- `OPENCODE_CONFIG_CONTENT`: Inline JSON config for opencode
- `OPENCODE_API_KEY`: API key for opencode server (if auth enabled)
- `REVIEWER_API_KEY`: API key for reviewer endpoint

### Example Usage

**Basic usage**:
```bash
opencode_runner \
  --task "Implement a bubble sort function in Rust" \
  --working-dir ./my-project \
  --worker-model "ollama/llama3.1" \
  --reviewer-url "http://localhost:11434/v1" \
  --reviewer-model "ollama/llama3.1"
```

**With extra opencode options**:
```bash
opencode_runner \
  --task "Refactor the authentication module" \
  --worker-model "anthropic/claude-3-5-sonnet" \
  -- \
  --agent "refactor-agent" \
  --temperature 0.7
```

**Headless mode for CI**:
```bash
opencode_runner \
  --task "Fix compilation errors" \
  --headless \
  --max-iterations 5 \
  --working-dir ./project \
  --reviewer-url "$REVIEWER_URL"
```

## Future Enhancements

1. **Feedback Action**: Allow reviewer to provide specific guidance to worker
   - Inject feedback as user message in session
   - Requires `messages.prompt_async()` support

2. **Completion Verification**: 
   - File tree exploration to verify task completion
   - Run tests or linting to validate output

3. **Multi-Session Support**:
   - Fork sessions on different approaches
   - Compare results and pick best

4. **Plugin Architecture**:
   - Custom sampler plugins
   - Custom reviewer adapters

5. **Persistent State**:
   - Save/load activity logs
   - Resume interrupted sessions

## Testing Strategy

1. **Unit Tests**:
   - Sampler event filtering
   - Reviewer retry logic
   - State management

2. **Integration Tests**:
   - Mock OpenCode server (using wiremock)
   - Mock reviewer API
   - Full control loop with test fixtures

3. **Manual Testing**:
   - Real OpenCode server with ollama
   - Various task types (coding, writing, analysis)
   - Edge cases (empty output, rapid events)

## Implementation Phases

### Phase 1: Core Infrastructure
- [ ] CLI argument parsing
- [ ] Server spawning and management
- [ ] Basic opencode_rs integration

### Phase 2: Control Loop
- [ ] Event streaming
- [ ] Sampler implementation
- [ ] Basic reviewer integration

### Phase 3: Resilience
- [ ] Retry logic with backoff
- [ ] Error handling and recovery
- [ ] State management

### Phase 4: UI
- [ ] TUI implementation
- [ ] Headless mode
- [ ] Logging and diagnostics

### Phase 5: Polish
- [ ] Documentation
- [ ] Testing
- [ ] Examples
