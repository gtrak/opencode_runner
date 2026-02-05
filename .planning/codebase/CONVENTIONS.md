# Coding Conventions

**Analysis Date:** 2026-02-05

## Naming Patterns

**Files:**
- Snake case: `control_loop.rs`, `opencode_stub.rs`
- Module entry: `mod.rs` (see `src/tui/mod.rs`)

**Structs:**
- Pascal case: `OpenCodeClient`, `ControlLoop`, `ServerManager`
- Descriptive names with suffixes indicating role: `*Client`, `*Manager`, `*Loop`, `*Config`

**Functions:**
- Snake case for all functions
- Constructor pattern: `new()` for primary initialization
- Builder pattern: `builder()` followed by chainable methods like `base_url()`, `build()`
- Async functions: Use `async fn` keyword (no naming convention needed)
- Example: `pub async fn connect(base_url: &str) -> Result<Self>` in `src/client.rs`

**Variables:**
- Snake case: `base_url`, `max_iterations`, `event_sender`
- Descriptive naming over abbreviations: `session_id` not `sid`
- Mutable variables use `mut` keyword explicitly

**Types:**
- Type aliases: Pascal case like `Result<T>` in stub
- Enums: Pascal case with descriptive variants: `ReviewerAction::Continue`, `ReviewerAction::Abort`
- Generic types: Single uppercase like `T`

**Constants:**
- Upper snake case for true constants (not detected in codebase)

## Code Style

**Formatting:**
- Rust standard formatting (rustfmt)
- No custom rustfmt.toml detected
- Standard 4-space indentation

**Import Organization:**
```rust
// 1. External crate imports (grouped by crate)
use anyhow::{Context, Result};
use tracing::{debug, info, warn};
use serde::{Deserialize, Serialize};

// 2. Standard library imports
use std::time::{Duration, Instant};
use std::path::Path;

// 3. Internal module imports
use crate::{
    client::OpenCodeClient,
    reviewer::{ReviewerClient, ReviewerContext},
    state::State,
};

// 4. Platform-specific conditional imports (last)
#[cfg(unix)]
use opencode_rs::sse::SseSubscription;
#[cfg(windows)]
use crate::opencode_stub::SseSubscription;
```

**Path Aliases:**
- None detected - uses explicit `crate::` prefix for internal modules

## Error Handling

**Primary Pattern - anyhow:**
- Use `anyhow::Result<T>` for most functions
- Context propagation with `.context()`:
```rust
let session = self.inner
    .sessions()
    .create(&request)
    .await
    .context("Failed to create session")?;
```

**Custom Errors - thiserror:**
- Use for domain-specific errors (see `src/opencode_stub.rs`):
```rust
#[derive(Debug, thiserror::Error)]
pub enum OpencodeError {
    #[error("OpenCode error: {0}")]
    Message(String),
    #[error("Not supported on Windows")]
    NotSupported,
}
```

**Error Levels:**
- `tracing::error!`: Fatal/unrecoverable errors
- `tracing::warn!`: Recoverable issues, retries
- `tracing::info!`: Normal operations
- `tracing::debug!`: Detailed diagnostics
- `tracing::trace!`: Very verbose (event filtering)

**Result Handling:**
- Use `?` operator for propagation
- Explicit match for critical paths:
```rust
match result {
    Ok(RunResult::Completed) => { ... }
    Ok(RunResult::Aborted(reason)) => { ... }
    Err(e) => { ... }
}
```

## Logging

**Framework:** `tracing` with `tracing-subscriber`

**Patterns:**
- Initialize at program start (`src/main.rs` lines 70-75)
- Environment-based filter: `RUST_LOG=info` or default to "info"
- Structured context in messages:
```rust
info!("Starting control loop");
info!("Task: {}", args.task);
warn!("Reviewer failed (attempt {}): {}", attempt + 1, e);
```

**When to Log:**
- INFO: Major state changes, iteration starts, decisions
- DEBUG: API calls, internal operations
- TRACE: Event filtering, high-frequency operations
- WARN: Retries, non-fatal failures
- ERROR: Unrecoverable errors before exit

## Comments

**When to Comment:**
- Module-level docs: `//!` for file-level documentation
- Function docs: Triple-slash `///` for public APIs
- Implementation notes for complex logic
- Platform-specific behavior explanation

**Examples:**
```rust
/// Sampler that captures and buffers worker output
/// Keeps only the last N lines for review
pub struct Sampler {
    buffer: VecDeque<String>,
    max_lines: usize,
}

// Platform-specific imports
#[cfg(unix)]
use opencode_rs::types::event::Event;
```

## Function Design

**Size:**
- Functions are focused (20-50 lines typical)
- Large functions broken into private helpers:
  - `stream_until_review()` - 57 lines
  - `review()` - 67 lines

**Parameters:**
- Prefer borrowing over ownership: `&str` over `String`
- Use `&self` for methods
- Config structs for multiple related parameters

**Return Values:**
- `Result<T>` for fallible operations
- Unit `()` for side-effect only
- Explicit enums for state: `RunResult::Completed`, `RunResult::Aborted`

## Module Design

**Organization:**
- One module per file: `client.rs`, `state.rs`, `reviewer.rs`
- TUI submodule: `tui/mod.rs`
- Platform stub: `opencode_stub.rs` (Windows compatibility)

**Exports:**
- Public structs/functions: `pub struct`, `pub fn`
- Internal-only: default visibility (module-private)
- Re-exports: `pub use types::*` in stub module

**Module Declaration:**
```rust
// In main.rs
mod client;
mod control_loop;
mod reviewer;
mod sampler;
mod server;
mod state;

#[cfg(windows)]
mod opencode_stub;

#[cfg(feature = "tui")]
mod tui;
```

## Platform-Specific Code

**Pattern:**
```rust
#[cfg(unix)]
use opencode_rs::types::event::Event;
#[cfg(windows)]
use crate::opencode_stub::types::event::Event;
```

**Platform stubs:**
- Full stub module for unsupported platforms (`src/opencode_stub.rs`)
- Same API surface, returns errors for operations

## Async Patterns

**Runtime:** Tokio with full features

**Pattern:**
- `async fn` for async operations
- `.await` at call sites
- `tokio::spawn()` for concurrent tasks
- `tokio::sync::mpsc` for channels

**Example:**
```rust
pub async fn run(&mut self) -> Result<RunResult> {
    let session_id = self.client.create_session(&self.config.task).await?;
    // ...
}
```

## Derive Macros

**Common:**
```rust
#[derive(Debug, Clone)]                    // For most types
#[derive(Debug, Clone, Serialize, Deserialize)]  // For API types
#[derive(Debug, thiserror::Error)]         // For errors
#[derive(Parser, Debug)]                   // For CLI args (clap)
#[derive(Default)]                         // When sensible defaults exist
```

## Serde Patterns

**Field renaming:**
```rust
#[serde(rename = "type")]
format_type: String,

#[serde(rename_all = "lowercase")]
pub enum ReviewerAction {
    Continue,
    Abort,
}

#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "part_added")]
    PartAdded { part: Part },
}
```

**Conditional serialization:**
```rust
#[serde(skip_serializing_if = "Option::is_none")]
response_format: Option<ResponseFormat>,
```

---

*Convention analysis: 2026-02-05*
