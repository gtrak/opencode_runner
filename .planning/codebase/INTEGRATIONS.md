# External Integrations

**Analysis Date:** 2026-02-05

## APIs & External Services

**LLM/AI API (Reviewer):**
- **Service:** OpenAI-compatible API endpoint
- **Default URL:** `http://localhost:11434/v1` (Ollama)
- **Purpose:** Review worker progress, decide continue/abort
- **Client:** `reqwest::Client` in `src/reviewer.rs`
- **Endpoints used:**
  - `POST /chat/completions` - Send review prompts with JSON response format
- **Timeout:** 30 seconds per request
- **Retry:** Exponential backoff with 3 max retries (delays: 2s, 4s, 8s)
- **Configuration:** Via `--reviewer-url` and `--reviewer-model` CLI args

**OpenCode Server (Worker):**
- **Service:** OpenCode CLI server (`opencode serve`)
- **Protocol:** HTTP REST API + Server-Sent Events (SSE)
- **Purpose:** Execute AI agent tasks, stream events
- **Client:** `opencode_rs` SDK (Unix) or stub (Windows) in `src/client.rs`
- **Endpoints used:**
  - `GET /health` - Health check in `src/server.rs`
  - `POST /sessions` - Create new session
  - `POST /sessions/{id}/messages/prompt_async` - Send prompts
  - `GET /sessions/{id}/events` - SSE subscription for events
- **Connection:** Spawned as subprocess on random available port
- **Lifecycle:** Managed by `ServerManager` in `src/server.rs`

## Data Storage

**Databases:**
- None - Stateless application, no persistence layer

**File Storage:**
- Local filesystem only
- Working directory configurable via `--working-dir`
- No cloud storage integrations

**Caching:**
- None - No caching layer detected

**State Management:**
- In-memory only via `State` struct in `src/state.rs`
- Tracks iteration count, previous reviewer summaries, sampler data

## Authentication & Identity

**Auth Provider:**
- None - No authentication system
- OpenCode server assumed to handle auth internally
- Reviewer API assumes local/trusted endpoint (no API keys in code)

**Authorization:**
- None detected

## Monitoring & Observability

**Error Tracking:**
- Structured logging via `tracing` crate
- All modules use `#[tracing::instrument]` pattern (where applicable)
- Log levels: trace, debug, info, warn, error

**Logs:**
- Console output via `tracing-subscriber`
- Configurable via `RUST_LOG` environment variable
- Default filter: "info"
- TUI mode captures logs and displays in activity panel

**Metrics:**
- None - No metrics export

**Health Checks:**
- OpenCode server health: `GET /health` polled during startup
- HTTP client timeout: 30s for reviewer API
- Inactivity timeout: Configurable (default 30s) for worker events

## CI/CD & Deployment

**Hosting:**
- CLI tool, runs locally
- No deployment platform integration

**CI Pipeline:**
- Not detected - No `.github/workflows` or similar

**Distribution:**
- Cargo build for local installation
- Requires `opencode` binary in PATH

## Environment Configuration

**Required env vars:**
- `RUST_LOG` - Optional, controls tracing log level (default: "info")

**Required binaries:**
- `opencode` - Must be installed and in PATH for server spawning

**CLI Arguments (configuration):**
| Argument | Default | Purpose |
|----------|---------|---------|
| `--task` | (required) | Task description |
| `--working-dir` | `.` | Working directory |
| `--worker-model` | `ollama/llama3.1` | Model for OpenCode worker |
| `--reviewer-url` | `http://localhost:11434/v1` | Reviewer API endpoint |
| `--reviewer-model` | `ollama/llama3.1` | Model for reviewer |
| `--max-iterations` | `10` | Max loop iterations |
| `--inactivity-timeout` | `30` | Seconds before triggering review |
| `--headless` | false | Disable TUI |

**Secrets location:**
- No secrets management detected
- No `.env` files
- Assumes local/development use

## Webhooks & Callbacks

**Incoming:**
- None - No webhook receivers

**Outgoing:**
- None - No webhook calls

**Event Subscriptions:**
- **SSE (Server-Sent Events)** - Subscribes to OpenCode server events
- **Source:** OpenCode server (`opencode serve`)
- **Events processed:** `PartAdded`, `PartUpdated`, `ToolCall`, `ToolResult`, `MessageCompleted`, `SessionCompleted`
- **Handler:** `ControlLoop::stream_until_review()` in `src/control_loop.rs`

## Network Architecture

**Connections:**
1. **Outgoing HTTP** → Reviewer API (OpenAI-compatible)
   - Configurable URL, default localhost:11434
   - JSON POST requests
   
2. **Local HTTP** → OpenCode server (spawned subprocess)
   - Random port selected at runtime
   - Health checks + SSE subscription
   - Localhost only (127.0.0.1)

**No External Network Dependencies:**
- All services assumed to run locally
- No cloud APIs
- No database connections
- No message queues

---

*Integration audit: 2026-02-05*
