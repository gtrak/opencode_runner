# Architecture

**Analysis Date:** 2026-02-05

## Pattern Overview

**Overall:** Command-Driven Async Control Loop

**Key Characteristics:**
- Orchestrates external OpenCode server process
- Streams worker events and samples output periodically
- Reviews progress using external OpenAI-compatible API
- Manages iterative task execution with abort/continue logic
- Optional terminal UI for real-time visualization

## Layers

**CLI/Entry Layer:**
- Purpose: Parse arguments, initialize system, handle mode selection
- Location: `src/main.rs`
- Contains: Command-line argument parsing, tracing initialization, mode selection (TUI vs headless)
- Depends on: `main()`, `run_tui_mode()` functions
- Used by: None (application entry point)

**Process Orchestration Layer:**
- Purpose: Manages external OpenCode server lifecycle
- Location: `src/server.rs`
- Contains: `ServerManager` struct with process spawning, health checking, shutdown
- Depends on: `tokio::process::Command`, `reqwest` for health checks
- Used by: `main.rs` (initialization)

**Client Integration Layer:**
- Purpose: Communicates with OpenCode server via SDK
- Location: `src/client.rs`
- Contains: `OpenCodeClient` wrapper around `opencode_rs::Client`
- Depends on: `opencode_rs` (Unix) or `opencode_stub` (Windows)
- Used by: `control_loop.rs` for session creation and SSE streaming

**Core Control Logic Layer:**
- Purpose: Orchestrates iterative task execution and decision making
- Location: `src/control_loop.rs`
- Contains: `ControlLoop`, `ControlConfig`, `RunResult`, `UiEvent` enums
- Depends on: `client.rs`, `reviewer.rs`, `sampler.rs`, `state.rs`
- Used by: `main.rs` (executed via `control_loop.run()`)

**Data Processing Layer:**
- Purpose: Captures and samples worker output
- Location: `src/sampler.rs`
- Contains: `Sampler` buffer with line filtering and sampling
- Depends on: `opencode_rs::types::event::Event`
- Used by: `control_loop.rs` during SSE streaming

**External Review Layer:**
- Purpose: Analyzes worker progress and makes decisions
- Location: `src/reviewer.rs`
- Contains: `ReviewerClient` with OpenAI-compatible API client, retry logic
- Depends on: `reqwest` HTTP client, `opencode_rs` SDK
- Used by: `control_loop.rs` after sampling

**State Management Layer:**
- Purpose: Tracks iteration history and runtime state
- Location: `src/state.rs`
- Contains: `State` with `Iteration` records, summary generation
- Depends on: `chrono` for timestamps
- Used by: `control_loop.rs`, `tui/mod.rs` for UI updates

**UI Presentation Layer:**
- Purpose: Provides terminal-based visualization and event handling
- Location: `src/tui/mod.rs`
- Contains: `Tui` terminal application, `UiState` shared state
- Depends on: `ratatui`, `crossterm` terminal libraries
- Used by: `main.rs` (via `run_tui_mode()`)

**Platform Abstraction Layer:**
- Purpose: Provides cross-platform SDK support
- Location: `src/opencode_stub.rs`
- Contains: Stub implementations that return `NotSupported` errors on Windows
- Depends on: None (standalone module)
- Used by: Unix and Windows builds via `#[cfg(unix)]` / `#[cfg(windows)]`

## Data Flow

**Session Creation Flow:**
1. `main.rs` spawns `ServerManager` with port and model arguments
2. `ServerManager.spawn()` starts external `opencode serve` process
3. Server health checked via HTTP `/health` endpoint
4. `OpenCodeClient.connect()` establishes SDK connection to server
5. `OpenCodeClient.create_session()` creates session with task description

**Iteration Flow:**
1. `ControlLoop::run()` starts iteration cycle
2. `ControlLoop.stream_until_review()` subscribes to SSE event stream
3. `Sampler::process_event()` buffers text, tool calls, filters noise
4. SSE events continue until timeout (inactivity) or completion detected
5. `Sampler::sample()` extracts latest N lines for review
6. `ReviewerClient::review()` sends sample + context to reviewer API
7. Reviewer returns decision (Continue or Abort)
8. `State::record_decision()` saves iteration data
9. Based on decision, loop continues or terminates
10. TUI updates displayed via `UiEvent` channel

**TUI Integration Flow:**
1. `main.rs` spawns control loop and TUI in parallel
2. `ControlLoop` sends `UiEvent` via mpsc channel
3. `run_tui_mode()` spawns event processor coroutine
4. `Tui::run()` event loop renders UI and polls input
5. `process_ui_event()` updates shared `UiState`
6. UI redraws based on current state

**State Management Flow:**
1. `State::start_iteration()` increments counter
2. `State::record_decision()` appends `Iteration` struct with timestamp
3. `State::get_previous_summaries()` provides review context
4. TUI displays formatted `format_activity_log()` output

**State Management:** Stateful iteration tracking with chronological records, supports historical context for review decisions and UI display.

## Key Abstractions

**ControlLoop:**
- Purpose: Orchestrate worker execution, sampling, and review decisions
- Examples: `src/control_loop.rs`
- Pattern: Central orchestrator with composed client/reviewer/sampler/state dependencies

**ServerManager:**
- Purpose: Lifecycle management for external OpenCode server process
- Examples: `src/server.rs`
- Pattern: Resource wrapper with spawn/health-check/shutdown lifecycle methods

**OpenCodeClient:**
- Purpose: SDK wrapper for OpenCode server communication
- Examples: `src/client.rs`
- Pattern: Adapter pattern for `opencode_rs` SDK abstraction

**ReviewerClient:**
- Purpose: External API client for progress review with retry logic
- Examples: `src/reviewer.rs`
- Pattern: HTTP client with exponential backoff retry pattern

**Sampler:**
- Purpose: Event buffer with selective content extraction
- Examples: `src/sampler.rs`
- Pattern: Circular buffer with filtering strategy

**State:**
- Purpose: Historical iteration tracking and summary generation
- Examples: `src/state.rs`
- Pattern: Record keeper with query methods for review context

**Tui:**
- Purpose: Terminal UI rendering and event handling
- Examples: `src/tui/mod.rs`
- Pattern: Terminal application with terminal state management

## Entry Points

**main.rs:**
- Location: `src/main.rs`
- Triggers: CLI argument parsing via `clap`, `main()` function
- Responsibilities:
  - Parse `Args` struct with all configuration options
  - Initialize tracing with `tracing_subscriber`
  - Spawn OpenCode server via `ServerManager::spawn()`
  - Connect to server via `OpenCodeClient::connect()`
  - Create control loop components (`ControlLoop`, `Sampler`, `ReviewerClient`, `State`)
  - Execute in TUI or headless mode
  - Cleanup and exit with appropriate status

**run_tui_mode():**
- Location: `src/main.rs`
- Triggers: `args.headless` flag, spawned within `main()`
- Responsibilities:
  - Spawn control loop with optional event channel
  - Spawn TUI application in parallel
  - Wait for both to complete
  - Display final status

## Error Handling

**Strategy:** Graceful degradation with retry and fallback

**Patterns:**
- **Exponential Backoff:** `ReviewerClient::review_with_retry()` uses 2^attempt second delays
- **Fallback Decisions:** Reviewer failure defaults to `Continue` after max retries
- **Process Health Checks:** `ServerManager::spawn()` validates server before proceeding
- **Error Logging:** `tracing` crate used throughout for debug/info/error levels
- **Context Propagation:** `anyhow::Result` with `Context` trait provides error chain
- **Platform-Specific Handling:** Unix uses SDK, Windows uses stub (compiles but errors)

**Graceful Degradation Examples:**
1. Server health check fails → abort with error message
2. OpenCode SDK unavailable (Windows) → compilation error during build
3. Reviewer API fails → retry 3x, default to Continue
4. SSE stream closes → proceed to review with current sample
5. Max iterations reached → terminate with appropriate exit code

## Cross-Cutting Concerns

**Logging:** `tracing` crate with environment-based filtering, levels: debug/info/warn/error

**Validation:** CLI argument parsing via `clap`, all arguments validated with defaults

**Authentication:** None (server and reviewer run locally on localhost)

**Concurrency:** `tokio` async runtime, `tokio::sync::mpsc` channels for inter-task communication, `Arc<Mutex>` for shared state between control loop and TUI

**Platform Abstraction:** Unix uses `opencode_rs` SDK, Windows uses stub with `NotSupported` errors, feature gate `#[cfg(unix)]`

**Configuration:** CLI arguments override defaults, environment filters via `TRACING_LOG_LEVEL`

**Testing:** Unit tests in `control_loop.rs`, `sampler.rs`, `state.rs`, `reviewer.rs` using `#[cfg(test)]` modules

---

*Architecture analysis: 2026-02-05*
