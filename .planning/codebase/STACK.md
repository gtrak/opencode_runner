# Technology Stack

**Analysis Date:** 2026-02-05

## Languages

**Primary:**
- Rust 2021 Edition - Entire application codebase in `src/`

**Secondary:**
- JavaScript/TypeScript - OpenCode plugin configuration only (`.opencode/package.json`)

## Runtime

**Environment:**
- Rust async runtime via Tokio
- Multi-threaded executor with work-stealing

**Package Manager:**
- Cargo (Rust's official package manager)
- Lockfile: `Cargo.lock` present

## Frameworks

**Core:**
- **Tokio 1.35** - Async runtime with full feature set (`rt-multi-thread`, `macros`, `sync`, `time`, `process`)
- **Futures 0.3** - Async utilities and traits

**CLI:**
- **Clap 4.4** - Command-line argument parsing with derive macros

**HTTP:**
- **Reqwest 0.12** - HTTP client with JSON support for API calls

**TUI (Optional):**
- **Ratatui 0.29** - Terminal UI framework
- **Crossterm 0.28** - Cross-platform terminal manipulation

**Serialization:**
- **Serde 1.0** - Serialization/deserialization framework
- **Serde_json 1.0** - JSON support for serde

**Error Handling:**
- **Anyhow 1.0** - Simple error handling with context
- **Thiserror 1.0** - Derive macros for custom error types

**Logging/Observability:**
- **Tracing 0.1** - Structured logging and instrumentation
- **Tracing-subscriber 0.3** - Subscriber implementation with env-filter support

**Time Handling:**
- **Chrono 0.4** - Date and time library with serde support

**Utilities:**
- **Portpicker 0.1** - Find available network ports
- **Tokio-util 0.7** - Additional Tokio utilities

**Platform-Specific:**
- **opencode_rs 0.1** - OpenCode SDK (Unix platforms only)

## Key Dependencies

**Critical:**
- **tokio** - Powers all async operations, process management, and timeouts
- **reqwest** - HTTP client for reviewer API calls (OpenAI-compatible endpoints)
- **serde** - JSON serialization for API payloads and event parsing
- **opencode_rs** - Official SDK for OpenCode server communication (Unix only)

**Infrastructure:**
- **tracing + tracing-subscriber** - All logging uses `tracing` macros (info!, warn!, error!, debug!, trace!)
- **anyhow** - All fallible functions return `anyhow::Result<T>`

## Configuration

**Environment:**
- Configured via CLI arguments only
- No environment variable files detected
- Logging level controlled via `RUST_LOG` env var (tracing-subscriber with env-filter)

**CLI Arguments:**
- `-t, --task`: Task description (required)
- `-d, --working-dir`: Working directory (default: ".")
- `--worker-model`: Model for worker (default: "ollama/llama3.1")
- `--reviewer-url`: OpenAI-compatible API URL (default: "http://localhost:11434/v1")
- `--reviewer-model`: Model for reviewer (default: "ollama/llama3.1")
- `--max-iterations`: Max iterations before abort (default: 10)
- `--inactivity-timeout`: Inactivity timeout seconds (default: 30)
- `--headless`: Run without TUI
- Extra arguments passed to `opencode serve` (trailing args)

**Build:**
- `Cargo.toml` - Standard Rust configuration
- Features:
  - `tui` (default) - Enables TUI with ratatui and crossterm
  - Can be disabled for headless builds: `--no-default-features`

**Compilation:**
```bash
cargo build --release          # With TUI
cargo build --release --no-default-features  # Headless only
```

## Platform Requirements

**Development:**
- Rust 1.70+ (2021 edition)
- Cargo
- Full functionality requires Unix (Linux/macOS) for opencode_rs SDK
- Windows builds use stub implementation (`src/opencode_stub.rs`)

**Production/Operation:**
- External dependency: `opencode` CLI binary must be in PATH for server spawning
- LLM API endpoint at configurable URL (default: Ollama on localhost:11434)
- Network access for HTTP API calls

---

*Stack analysis: 2026-02-05*
