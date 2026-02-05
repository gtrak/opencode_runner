# Architecture

**Analysis Date:** 2026-02-05

## Pattern Overview

**Overall:** Actor-Oriented Control Loop with Observer Pattern

The `opencode_runner` implements an intelligent monitoring system that orchestrates an AI agent (worker) through a control loop with automated review checkpoints. The architecture follows an actor-style pattern where distinct components communicate through messages and state updates.

**Key Characteristics:**
- Platform-agnostic abstraction layer for cross-compilation (Windows stubs, Unix native)
- Event-driven SSE streaming from worker to sampler
- Review-driven iteration control with configurable thresholds
- Optional TUI with async event multiplexing
- Async/await throughout using Tokio runtime

## Layers

**CLI / Entry Layer:**
- Purpose: Parse arguments, bootstrap components, coordinate startup/shutdown
- Location: `src/main.rs`
- Contains: CLI argument parsing with clap, component initialization, feature-gated TUI orchestration
- Depends on: All other modules
- Used by: Operating system as entry point

**Control Loop Layer:**
- Purpose: Orchestrate worker-reviewer iteration cycle, enforce termination conditions
- Location: `src/control_loop.rs`
- Contains: `ControlLoop` struct, iteration management, inactivity timeout handling, event streaming
- Depends on: `client`, `reviewer`, `sampler`, `state`
- Used by: `main` (direct), `tui` (indirect via events)

**Worker Client Layer:**
- Purpose: Interface with OpenCode server via HTTP/SSE
- Location: `src/client.rs`
- Contains: `OpenCodeClient` wrapper around `opencode_rs::Client`
- Depends on: `opencode_rs` (Unix), `opencode_stub` (Windows)
- Used by: `control_loop`

**Reviewer Layer:**
- Purpose: Evaluate worker progress using external LLM (OpenAI-compatible API)
- Location: `src/reviewer.rs`
- Contains: `ReviewerClient`, retry logic with exponential backoff, prompt construction
- Depends on: `reqwest`, `serde`
- Used by: `control_loop`

**Sampling Layer:**
- Purpose: Capture and buffer worker output for review
- Location: `src/sampler.rs`
- Contains: `Sampler` with fixed-size ring buffer (VecDeque), event filtering
- Depends on: Event types from `opencode_rs`/`opencode_stub`
- Used by: `control_loop`

**State Management Layer:**
- Purpose: Track iteration history and runtime metrics
- Location: `src/state.rs`
- Contains: `State` struct, `Iteration` records, activity log formatting
- Depends on: `reviewer` types
- Used by: `control_loop`, `tui`

**Server Management Layer:**
- Purpose: Spawn and manage OpenCode server subprocess
- Location: `src/server.rs`
- Contains: `ServerManager` with health check polling, graceful shutdown
- Depends on: `tokio::process`, `reqwest`
- Used by: `main`

**TUI Layer (Optional Feature):**
- Purpose: Real-time terminal visualization
- Location: `src/tui/mod.rs`
- Contains: `Tui`, `UiState`, event processing, ratatui widgets
- Depends on: `ratatui`, `crossterm`, `control_loop::UiEvent`
- Used by: `main` (when `tui` feature enabled)

**Platform Abstraction Layer:**
- Purpose: Enable Windows compilation by stubbing Unix-only SDK
- Location: `src/opencode_stub.rs`
- Contains: Type-compatible stub implementations returning `NotSupported` errors
- Depends on: `serde`, `thiserror`
- Used by: `client`, `control_loop`, `sampler` (via conditional compilation)

## Data Flow

**Main Execution Flow:**

1. **Initialization Phase:**
   - `main` parses CLI arguments via `clap`
   - `ServerManager::spawn` starts OpenCode server subprocess with random port
   - Health check polling verifies server readiness (30s timeout)
   - `OpenCodeClient::connect` establishes connection to server
   - `ReviewerClient`, `Sampler`, `State` instantiated
   - `ControlLoop` assembled with all components

2. **Control Loop Execution:**
   - `ControlLoop::run` creates session and subscribes to SSE events
   - For each iteration:
     a. Check max iterations guard
     b. Stream events via `stream_until_review`
     c. `Sampler` captures worker output (last 100 lines)
     d. Build `ReviewerContext` with task, history, and sample
     e. `ReviewerClient::review_with_retry` calls LLM API (3 retries, exponential backoff)
     f. Record decision and either continue or abort

3. **Event Streaming:**
   - `SseSubscription::recv` receives `Event` from OpenCode server
   - `Sampler::process_event` filters and buffers relevant content:
     - Captures: `PartAdded` (text), `PartUpdated` (deltas), `ToolCall`, `Error`
     - Skips: `ToolResult` (verbose), `Thinking` (internal), `Progress` (noisy)
   - Significant events forwarded to TUI via `mpsc` channel

4. **Review Decision Flow:**
   - `ReviewerClient` builds prompt with task context and sample
   - POST to OpenAI-compatible endpoint (`/chat/completions`)
   - Parse JSON response into `ReviewerDecision { action, reason }`
   - Return `Continue` (loop again) or `Abort` (terminate with reason)

5. **TUI Rendering (if enabled):**
   - `main` spawns control loop in one task, TUI in another
   - `UiEvent` channel bridges async control loop to TUI
   - 250ms tick rate for UI refresh
   - Three-pane layout: header, worker output (60%), activity log (40%), footer

6. **Termination:**
   - Natural: Completion event detected → success
   - Reviewer Abort: LLM determines stuck/looping → failure with reason
   - Max iterations: Configurable limit exceeded → failure
   - Inactivity: No events within timeout → triggers review (may continue)
   - `ServerManager::shutdown` kills subprocess and waits for exit

**State Management:**
- `State` persists across iterations in `ControlLoop`
- `Sampler` cleared between iterations (only retains current sample)
- `UiState` in TUI accumulates display data (last 100 worker output lines)

## Key Abstractions

**ControlLoop:**
- Purpose: Central orchestrator managing the worker-reviewer lifecycle
- Key Methods: `new()`, `run()`, `stream_until_review()`
- Located in: `src/control_loop.rs`
- Pattern: State machine with iteration counter

**OpenCodeClient:**
- Purpose: Thin wrapper over `opencode_rs::Client` with health verification
- Key Methods: `connect()`, `create_session()`, `subscribe()`, `send_message()`
- Located in: `src/client.rs`
- Pattern: Adapter/Facade

**ReviewerClient:**
- Purpose: LLM-based progress evaluation with resilience
- Key Methods: `new()`, `review_with_retry()`, `review()`
- Located in: `src/reviewer.rs`
- Pattern: Retry with exponential backoff, structured output (JSON)

**Sampler:**
- Purpose: Ring buffer for recent worker output
- Key Methods: `new()`, `process_event()`, `sample()`, `clear()`
- Located in: `src/sampler.rs`
- Pattern: Fixed-size circular buffer (VecDeque)

**ServerManager:**
- Purpose: Process lifecycle management with health verification
- Key Methods: `spawn()`, `shutdown()`, `port()`, `base_url()`
- Located in: `src/server.rs`
- Pattern: RAII with Drop cleanup

## Entry Points

**Application Entry:**
- Location: `src/main.rs::main()`
- Triggers: CLI invocation with `opencode_runner` binary
- Responsibilities:
  1. Initialize tracing subscriber
  2. Parse CLI arguments (Args struct)
  3. Spawn OpenCode server via ServerManager
  4. Connect client and initialize components
  5. Run control loop (headless or TUI mode)
  6. Cleanup and exit with appropriate code

**TUI Entry (feature-gated):**
- Location: `src/main.rs::run_tui_mode()`
- Triggers: When `--headless` not specified and `tui` feature enabled
- Responsibilities:
  1. Create mpsc channel for UiEvents
  2. Spawn control loop in background task
  3. Run TUI event loop with async input handling
  4. Coordinate shutdown between tasks

## Error Handling

**Strategy:** Propagate errors with context using `anyhow`

**Patterns:**
- All async functions return `Result<T>` (anyhow)
- `.context("message")` for operation-level context
- Graceful degradation in reviewer (default to Continue after retries)
- TUI implements `Drop` for terminal cleanup
- ServerManager implements `Drop` for process cleanup

**Error Types:**
- `anyhow::Error`: General errors with context chain
- `OpencodeError` (stub): Platform-specific stub errors
- Custom `ReviewerDecision` for semantic control flow (not failure)

## Cross-Cutting Concerns

**Logging:**
- Framework: `tracing` with `tracing_subscriber`
- Configuration: Environment filter (`RUST_LOG` or default "info")
- Patterns: `info!` for lifecycle, `debug!` for diagnostics, `warn!` for recoverable, `error!` for failures

**Configuration:**
- Source: CLI arguments via clap derive macros
- Runtime: `ControlConfig` struct passed to control loop
- Defaults: Defined in `Args` struct (max_iterations: 10, inactivity_timeout: 30s)

**Platform Abstraction:**
- Conditional compilation via `#[cfg(unix)]` / `#[cfg(windows)]`
- `opencode_stub.rs` provides type-compatible stubs for Windows builds
- Production use requires Unix (Linux/macOS) for actual OpenCode SDK

**Async Runtime:**
- Tokio with `full` features
- `tokio::main` macro for entry
- `tokio::spawn` for concurrent tasks (TUI + control loop)
- `tokio::sync::mpsc` for event channels
- `tokio::time` for timeouts and delays

---

*Architecture analysis: 2026-02-05*
