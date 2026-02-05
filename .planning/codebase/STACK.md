# Technology Stack

**Analysis Date:** 2026-02-05

## Languages

**Primary:**
- **Rust** (edition 2021) - Core application logic, async runtime, CLI interface

**Secondary:**
- **Not applicable** - Single language project

## Runtime

**Environment:**
- **Tokio** (1.35+) - Full-featured async runtime for concurrent operations

**Package Manager:**
- **Cargo** - Rust package manager
- Lockfile: `Cargo.lock` (exists)

## Frameworks

**Core:**
- **None** - Pure Rust implementation with no frameworks

**UI/Terminal:**
- **Ratatui** (0.29) - Terminal User Interface library (optional feature)
- **Crossterm** (0.28) - Cross-platform terminal handling (optional feature, required by ratatui)

**CLI:**
- **Clap** (4.4) - Command-line argument parsing with derive macros

**Testing:**
- **Not applicable** - No testing framework detected in source

**Build/Dev:**
- **Cargo** - Native Rust build system

## Key Dependencies

**Critical:**
- **tokio** (1.35+, "full" features) - Async runtime for all concurrent operations
- **opencode_rs** (0.1.2) - Rust SDK for OpenCode server (Unix only)
- **reqwest** (0.12, "json") - HTTP client for reviewer API and server health checks
- **serde** (1.0+, "derive") - Serialization/deserialization for JSON API interactions
- **serde_json** (1.0) - JSON parsing support
- **clap** (4.4, "derive") - CLI argument parsing

**Infrastructure:**
- **anyhow** (1.0) - Error handling with context
- **thiserror** (1.0) - Error type definitions

**Logging/Tracing:**
- **tracing** (0.1) - Structured logging framework
- **tracing-subscriber** (0.3, "env-filter") - Logging subscriber configuration

**Utilities:**
- **chrono** (0.4+, "serde") - Time/date handling for iteration timestamps
- **portpicker** (0.1) - Port allocation for local server instances
- **tokio-util** (0.7) - Additional Tokio utilities
- **futures** (0.3) - Future utilities and compatibility

**Optional UI:**
- **ratatui** (0.29) - Terminal UI framework (enabled by default feature)
- **crossterm** (0.28) - Terminal I/O (used by ratatui)

## Configuration

**Environment:**
- **Logging**: Configured via `RUST_LOG` environment variable using tracing-subscriber
- **Default**: `info` level if env var not set

**Build:**
- **Cargo features** in `Cargo.toml`:
  - `default = ["tui"]` - Includes TUI mode by default
  - `tui = ["ratatui", "crossterm"]` - Optional TUI feature flag

**CLI Arguments:**
- Task description (`--task`)
- Working directory (`--working-dir`)
- Worker model (`--worker-model`, default: "ollama/llama3.1")
- Reviewer API URL (`--reviewer-url`, default: "http://localhost:11434/v1")
- Reviewer model (`--reviewer-model`, default: "ollama/llama3.1")
- Maximum iterations (`--max_iterations`, default: 10)
- Inactivity timeout (`--inactivity_timeout`, default: 30 seconds)
- Headless mode (`--headless`)

## Platform Requirements

**Development:**
- **Rust**: Requires Rust toolchain (rustup)
- **Ollama**: Required for local LLM inference (worker model and reviewer model)
- **opencode CLI**: Required external tool for worker execution

**Production:**
- **Target platforms**:
  - **Unix/macOS**: Full functionality via `opencode_rs` SDK
  - **Windows**: Limited functionality (stub implementation, SSE subscription not supported)
- **Local HTTP**: Reviewer API expected at local address (`localhost:11434/v1`)

## Dependencies at Risk

**opencode_rs (0.1.2)**:
- Platform-specific: Only available on Unix/macOS
- Windows requires stub implementation with limited functionality
- Risk: Breaks on Windows builds or when SDK updates without compatibility

**opencode CLI (External Tool)**:
- External dependency managed by spawning system process
- Risk: If `opencode` is not installed or in PATH, application fails
- Fix: Add validation/checks during server spawning

---

*Stack analysis: 2026-02-05*
