# External Integrations

**Analysis Date:** 2026-02-05

## APIs & External Services

**Local LLM API (Reviewer):**
- **Service**: Ollama (or any OpenAI-compatible LLM server)
- **Endpoint**: `http://localhost:11434/v1/chat/completions` (configurable)
- **Purpose**: Progress monitoring and decision-making for the worker
- **Client**: `reqwest::Client`
- **Model**: Configurable, defaults to "ollama/llama3.1"
- **Authentication**: None (local HTTP endpoint)
- **Rate limiting**: 3 retry attempts with exponential backoff (2s delay per retry)
- **Response format**: JSON with action ("continue" or "abort") and reason

**OpenCode Server (External Tool):**
- **Tool**: `opencode serve` CLI application
- **Spawned by**: `ServerManager::spawn()` in `src/server.rs`
- **Purpose**: Provides AI worker execution with file editing capabilities
- **Managed process**: Background process started with dynamic port allocation
- **Configuration**:
  - Port: Dynamically assigned via `portpicker` (random free port)
  - Hostname: `127.0.0.1`
  - Model: Configurable via `--model` flag
  - Working directory: User-specified directory
- **Health check**: HTTP GET request to `/health` endpoint on random port
- **Timeout**: 30 seconds to verify server is ready
- **Unix/Linux**: Uses official `opencode_rs` SDK (types, clients, SSE)
- **Windows**: Uses stub implementation (`src/opencode_stub.rs`) with limited API

## Data Storage

**Databases:**
- **None** - No database backend required; operations are memory-based or external API calls

**File Storage:**
- **Local Filesystem**:
  - Working directory: User-specified via `--working-dir` flag
  - File operations: Performed by external `opencode` worker process
  - Managed by: No persistence layer; files persist on filesystem

**Caching:**
- **Memory-based**:
  - Sampler buffer: Stores last N lines of worker output (100 lines by default)
  - State tracking: In-memory iteration history
  - TUI state: Shared Arc<Mutex<UiState>> across async tasks

## Authentication & Identity

**Authentication:**
- **None** - All integrations are local HTTP endpoints
- **OpenCode Server**: No authentication required (localhost)
- **LLM Reviewer**: No authentication required (localhost)

**Authorization:**
- **Not applicable** - No role-based access control

## Monitoring & Observability

**Error Tracking:**
- **None** - No external error tracking service configured

**Logs:**
- **Framework**: `tracing` for structured logging
- **Configuration**: Via `RUST_LOG` environment variable
- **Levels**: `trace`, `debug`, `info`, `warn`, `error`
- **Default**: `info` level
- **Output**: Console output (not external service)

**Metrics:**
- **Not tracked**: No metrics collection (iteration count, sample sizes, retry counts tracked internally only)

## CI/CD & Deployment

**Hosting:**
- **Local development**: Runs locally with Ollama LLM
- **Deployment target**: Not specified (assumed local development use)
- **Environment**: No cloud deployment target identified

**CI Pipeline:**
- **None** - No CI configuration detected
- **Build**: Native Rust builds via `cargo build`

## Environment Configuration

**Required env vars:**
- **`RUST_LOG`** (optional): Logging level (default: `info`)
- **`OPENCODE_PATH`** (optional): Path to `opencode` CLI binary (not explicitly coded, relies on PATH)

**Optional env vars:**
- No other environment variables detected

**Secrets location:**
- **None** - No secrets or credentials stored

**Configuration file:**
- **`Cargo.toml`**: Dependencies and build configuration
- **No app-level config file** (all configuration via CLI args)

## Webhooks & Callbacks

**Incoming:**
- **None** - No webhooks or HTTP endpoints exposed

**Outgoing:**
- **HTTP POST to Reviewer API**: Sends chat completion requests to local LLM
- **HTTP GET to OpenCode Server**: Sends health check requests
- **HTTP SSE Subscription**: Subscribes to OpenCode server event stream

## Integration Architecture

**Flow:**
1. **Server Management**: Application spawns `opencode serve` process
2. **Health Check**: Application verifies server is ready via HTTP GET `/health`
3. **Session Creation**: Application creates session via OpenCode API
4. **Event Streaming**: Application subscribes to SSE stream for worker output
5. **Output Sampling**: Sampler buffers worker output (text, tool calls)
6. **Reviewer API**: HTTP POST to LLM API with context and sample
7. **Decision**: LLM returns Continue/Abort with reasoning
8. **Loop**: Repeat from step 4 until abortion or completion

**Platform-Specific Behavior:**
- **Unix/macOS**: Full SSE event subscription via `opencode_rs` SDK
- **Windows**: Stub implementation with disabled SSE subscription (always returns "Not supported")
- **Error handling**: Reviewer uses retry logic with exponential backoff if API calls fail

---

*Integration audit: 2026-02-05*
