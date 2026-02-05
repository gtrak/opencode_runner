# Coding Conventions

**Analysis Date:** 2026-02-05

## Naming Patterns

**Files:**
- `snake_case` - All source files use lowercase with underscores (e.g., `client.rs`, `control_loop.rs`, `state.rs`, `reviewer.rs`, `sampler.rs`, `server.rs`)
- `mod.rs` - Module definitions kept separate in `src/tui/mod.rs` for TUI functionality

**Functions:**
- `snake_case` - Public and private functions use lowercase with underscores (e.g., `connect()`, `run()`, `sample()`, `process_event()`, `build_prompt()`)
- `test_` prefix - Test functions prefix with `test_` for identification (e.g., `test_sampler_basic()`, `test_parse_decision()`)

**Variables:**
- `snake_case` - Variable names use lowercase with underscores (e.g., `event_sender`, `buffer`, `max_lines`, `start_time`)
- Constants not found in codebase (no const bindings observed)

**Types:**
- `PascalCase` - Structs and enums use PascalCase (e.g., `OpenCodeClient`, `ControlLoop`, `ReviewerDecision`, `State`, `Iteration`, `UiEvent`, `RunResult`, `ControlConfig`, `UiState`)

**Private items:**
- Use `#[cfg(test)] mod tests { ... }` pattern for test module within source files

## Code Style

**Formatting:**
- No explicit formatting configuration files found (`.eslintrc*`, `.prettierrc*`, `biome.json`, etc.)
- Uses Rust's standard formatting (`rustfmt` would apply)
- Indentation: 4 spaces
- Brace placement: K&R style with braces on same line as control statements (e.g., `if let Some(ref sender) = event_sender {`)
- Line length: ~80-100 characters (based on actual code)

**Linting:**
- No linting configuration files found
- No external linting tools configured (clippy not configured)
- Code uses idiomatic Rust patterns but no explicit linting enforced

**Documentation:**
- **Module doc comments:** Used for explaining module purpose (e.g., `src/opencode_stub.rs` lines 1-4)
- **Inline comments:** Used sparingly with `//` for explanations (e.g., `src/control_loop.rs` line 28 "/// Platform-specific imports")
- **No JSDoc/TSDoc:** Rust does not use JSDoc/TSDoc; comments follow Rust conventions

## Import Organization

**Order:**
1. Standard library imports (`use std::...`)
2. External crate imports (`use crate::...` for local modules)
3. Dependencies from `Cargo.toml`

**Path Aliases:**
- Uses `crate::` for local module imports (e.g., `use crate::reviewer::ReviewerAction`)
- No explicit path alias configuration (`use crate::` used directly)

**Example from `src/control_loop.rs`:**
```rust
use crate::{
    client::OpenCodeClient,
    reviewer::{ReviewerClient, ReviewerContext, ReviewerDecision, ReviewerAction},
    sampler::Sampler,
    state::State,
};
```

## Error Handling

**Patterns:**
- `anyhow::Result` - Primary error type used for propagating errors through the codebase (see `main.rs` line 1, `client.rs` line 1, etc.)
- `?` operator - Used for early error propagation (e.g., `client.create_session(&self.config.task).await?` in `control_loop.rs` line 84)
- `.context()` method - Attaches context to errors for better debugging (e.g., `.context("Failed to create session")?` in `client.rs` line 65)
- `thiserror` - Used for custom error types (in `opencode_stub.rs` lines 12-20)

**Custom Errors:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum OpencodeError {
    #[error("OpenCode error: {0}")]
    Message(String),
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Not supported on Windows")]
    NotSupported,
}
```

**Logging:**
- Uses `tracing` crate (configured via `tracing-subscriber`)
- Log levels used:
  - `trace!()` - Very detailed traces (e.g., `src/sampler.rs` line 54)
  - `debug!()` - Debug information (e.g., `src/client.rs` line 42)
  - `info!()` - Informational messages (e.g., `src/main.rs` line 79)
  - `warn!()` - Warnings (e.g., `src/control_loop.rs` line 105)
  - `error!()` - Errors (e.g., `src/reviewer.rs` line 127)

## Logging

**Framework:** `tracing` crate with `tracing-subscriber` for configuration

**Patterns:**
- **Initialization:** Main entry point sets up tracing (e.g., `src/main.rs` lines 69-75)
- **Environment-based filtering:** Uses `EnvFilter` from default environment or defaults to "info" level
- **Logging levels:** Used strategically at different points:
  - High-level flow: `info!()`
  - Debug details: `debug!()`
  - Warnings and issues: `warn!()`
  - Errors: `error!()`
  - Very detailed traces: `trace!()`
- **Structured logging:** Not explicitly used (basic string messages)

## Comments

**When to Comment:**
- Module-level comments for explaining module purpose (e.g., `src/opencode_stub.rs` lines 1-4)
- Brief inline comments for complex logic (e.g., `src/sampler.rs` line 28 "Process an event from the SSE stream")
- TODO comments for known issues (e.g., `src/control_loop.rs` line 163)
- No docstrings for public functions observed

**JSDoc/TSDoc:**
- Not applicable (Rust uses module doc comments and `///` for inline comments)

## Function Design

**Size:**
- Functions are moderately sized (15-50 lines for most public functions)
- Very long functions are avoided
- Helper functions extracted for clarity (e.g., `extract_text_content()`, `add_line()`, `should_send_to_ui()`)

**Parameters:**
- Functions with few parameters use positional arguments
- Multiple parameters organized logically (e.g., `ControlLoop::new(client, reviewer, sampler, state, config)`)

**Return Values:**
- Public functions return `anyhow::Result<T>` for error handling
- Helper functions return `Option<T>` for optional values (e.g., `extract_event_text()` returns `Option<String>`)
- Enum for result codes (e.g., `RunResult` enum with `Completed`, `Aborted(String)`, `MaxIterations`)

**Immutability:**
- Most functions take `&self` for reference to avoid mutating
- Methods with state mutation take `&mut self`
- Functions that need internal state use struct fields

## Module Design

**Exports:**
- Public types and functions exported from modules (e.g., `pub struct OpenCodeClient`, `pub fn connect()`)
- Tests hidden inside `#[cfg(test)] mod tests` blocks
- No barrel files (single file modules)

**Dependency Injection:**
- Components constructed separately and passed as dependencies (e.g., `ControlLoop::new(client, reviewer, sampler, state, config)`)
- No global state or statics

**Separation of Concerns:**
- Clear module separation by responsibility:
  - `client.rs` - OpenCode API client
  - `control_loop.rs` - Orchestration logic
  - `reviewer.rs` - External API calls for reviews
  - `sampler.rs` - Buffering output
  - `server.rs` - Process management
  - `state.rs` - State tracking
  - `tui/mod.rs` - UI rendering

---

*Convention analysis: 2026-02-05*
