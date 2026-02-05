# Requirements: OpenCode Runner

**Defined:** 2026-02-05
**Core Value:** The control loop automatically detects when an AI worker is stuck in a loop or has completed its task, preventing runaway processes and providing actionable feedback about task execution.

## v1 Requirements (Implementation Complete)

### CLI (main.rs)

- [x] **CLI-01**: Parse command-line arguments with clap derive macros
- [x] **CLI-02**: Support --task (required) for task description
- [x] **CLI-03**: Support --working-dir (default: ".") for working directory
- [x] **CLI-04**: Support --worker-model (default: "ollama/llama3.1") for worker LLM
- [x] **CLI-05**: Support --reviewer-url (default: "http://localhost:11434/v1") for OpenAI-compatible API
- [x] **CLI-06**: Support --reviewer-model (default: "ollama/llama3.1") for reviewer LLM
- [x] **CLI-07**: Support --max-iterations (default: 10) as iteration guard
- [x] **CLI-08**: Support --inactivity-timeout (default: 30) in seconds
- [x] **CLI-09**: Support --headless flag to disable TUI
- [x] **CLI-10**: Support trailing arguments passed to `opencode serve`
- [x] **CLI-11**: Initialize tracing subscriber for logging
- [x] **CLI-12**: Spawn server and start control loop (TUI or headless)
- [x] **CLI-13**: Handle shutdown signals and cleanup

### Server Management (server.rs)

- [x] **SRV-01**: Spawn opencode serve with random available port
- [x] **SRV-02**: Use portpicker to find available port
- [x] **SRV-03**: Set working directory with current_dir()
- [x] **SRV-04**: Capture stdout to detect server readiness
- [x] **SRV-05**: Wait for server ready with configurable timeout
- [x] **SRV-06**: Implement graceful shutdown on drop
- [x] **SRV-07**: Provide base URL for client connections

### Client (client.rs)

- [x] **CLT-01**: Connect to OpenCode server using opencode_rs
- [x] **CLT-02**: Create session with initial task description
- [x] **CLT-03**: Subscribe to session events (SSE stream)
- [x] **CLT-04**: Provide send_message for future feedback capability
- [x] **CLT-05**: Handle authentication from environment if needed
- [x] **CLT-06**: Windows stub implementation for cross-compilation

### Sampler (sampler.rs)

- [x] **SMP-01**: Implement 100-line ring buffer (VecDeque)
- [x] **SMP-02**: Capture PartAdded events (text type only)
- [x] **SMP-03**: Capture PartUpdated events (deltas)
- [x] **SMP-04**: Capture ToolCall events (summarized format)
- [x] **SMP-05**: Capture Error events
- [x] **SMP-06**: Skip ToolResult events (too verbose)
- [x] **SMP-07**: Skip Thinking/Reasoning events
- [x] **SMP-08**: Skip System messages
- [x] **SMP-09**: Provide sample() to get buffered content
- [x] **SMP-10**: Provide clear() to reset buffer

### Reviewer (reviewer.rs)

- [x] **REV-01**: Implement ReviewerClient with HTTP client
- [x] **REV-02**: Support configurable base_url and model
- [x] **REV-03**: Build prompt with task, iteration, history, and sample
- [x] **REV-04**: Call OpenAI-compatible /chat/completions endpoint
- [x] **REV-05**: Parse JSON response into ReviewerDecision
- [x] **REV-06**: Support Continue and Abort actions
- [x] **REV-07**: Implement exponential backoff retry (3 retries)
- [x] **REV-08**: Default to Continue after max retries
- [x] **REV-09**: Provide reason for each decision

### State Management (state.rs)

- [x] **STA-01**: Track iterations with number, timestamp, sample size
- [x] **STA-02**: Record reviewer decisions with retry count
- [x] **STA-03**: Provide get_previous_summaries() for context
- [x] **STA-04**: Check is_max_iterations() guard
- [x] **STA-05**: Generate format_activity_log() for display
- [x] **STA-06**: Track start time and current iteration

### Control Loop (control_loop.rs)

- [x] **CTL-01**: Connect to server and create session
- [x] **CTL-02**: Subscribe to events and initialize sampler
- [x] **CTL-03**: Stream events until inactivity timeout or completion
- [x] **CTL-04**: Detect MessageCompleted as natural break point
- [x] **CTL-05**: Sample output and call reviewer with retry
- [x] **CTL-06**: Record decision in state
- [x] **CTL-07**: Continue on Continue action (loop)
- [x] **CTL-08**: Abort on Abort action with reason
- [x] **CTL-09**: Respect max_iterations limit
- [x] **CTL-10**: Handle inactivity timeout (no events)
- [x] **CTL-11**: Send events to TUI via channel if available
- [x] **CTL-12**: Return RunResult (Completed, Aborted, MaxIterations)

### TUI (tui/mod.rs)

- [x] **TUI-01**: Initialize ratatui terminal with crossterm
- [x] **TUI-02**: Create mpsc channel for UiEvent
- [x] **TUI-03**: Three-pane layout: header, worker output, activity log, footer
- [x] **TUI-04**: Display iteration count, status, uptime in header
- [x] **TUI-05**: Show last 100 lines of worker output
- [x] **TUI-06**: Show activity log with reviewer decisions
- [x] **TUI-07**: Show worker model and port in footer
- [x] **TUI-08**: Handle 'q' key to quit
- [x] **TUI-09**: 250ms tick rate for refresh
- [x] **TUI-10**: Feature-gated compilation (tui feature)

## v2 Requirements (Future)

### Feedback Action

- **FDB-01**: Allow reviewer to provide specific guidance to worker
- **FDB-02**: Inject feedback as user message in session
- **FDB-03**: Requires opencode_rs messages.prompt_async() support

### Completion Verification

- **VER-01**: Explore file tree to verify task completion
- **VER-02**: Run tests or linting to validate output
- **VER-03**: Check for expected file changes

### Notifications

- **NOT-01**: Send notification on task completion/failure
- **NOT-02**: Support webhook callbacks
- **NOT-03**: Configurable notification endpoints

## Out of Scope

| Feature | Reason |
|---------|--------|
| Multi-session forking | Complex parallel execution, not core to control loop value |
| Plugin architecture | Premature abstraction, current use cases covered |
| Persistent state save/load | Not required for current CI/automation use cases |
| Scheduled/background execution | Separate cron/systemd concern |
| Configuration file support | CLI args sufficient for current needs |
| Real-time collaboration | Single-user tool by design |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CLI-01 | Initial Implementation | Complete |
| CLI-02 | Initial Implementation | Complete |
| CLI-03 | Initial Implementation | Complete |
| CLI-04 | Initial Implementation | Complete |
| CLI-05 | Initial Implementation | Complete |
| CLI-06 | Initial Implementation | Complete |
| CLI-07 | Initial Implementation | Complete |
| CLI-08 | Initial Implementation | Complete |
| CLI-09 | Initial Implementation | Complete |
| CLI-10 | Initial Implementation | Complete |
| CLI-11 | Initial Implementation | Complete |
| CLI-12 | Initial Implementation | Complete |
| CLI-13 | Initial Implementation | Complete |
| SRV-01 | Initial Implementation | Complete |
| SRV-02 | Initial Implementation | Complete |
| SRV-03 | Initial Implementation | Complete |
| SRV-04 | Initial Implementation | Complete |
| SRV-05 | Initial Implementation | Complete |
| SRV-06 | Initial Implementation | Complete |
| SRV-07 | Initial Implementation | Complete |
| CLT-01 | Initial Implementation | Complete |
| CLT-02 | Initial Implementation | Complete |
| CLT-03 | Initial Implementation | Complete |
| CLT-04 | Initial Implementation | Complete |
| CLT-05 | Initial Implementation | Complete |
| CLT-06 | Initial Implementation | Complete |
| SMP-01 | Initial Implementation | Complete |
| SMP-02 | Initial Implementation | Complete |
| SMP-03 | Initial Implementation | Complete |
| SMP-04 | Initial Implementation | Complete |
| SMP-05 | Initial Implementation | Complete |
| SMP-06 | Initial Implementation | Complete |
| SMP-07 | Initial Implementation | Complete |
| SMP-08 | Initial Implementation | Complete |
| SMP-09 | Initial Implementation | Complete |
| SMP-10 | Initial Implementation | Complete |
| REV-01 | Initial Implementation | Complete |
| REV-02 | Initial Implementation | Complete |
| REV-03 | Initial Implementation | Complete |
| REV-04 | Initial Implementation | Complete |
| REV-05 | Initial Implementation | Complete |
| REV-06 | Initial Implementation | Complete |
| REV-07 | Initial Implementation | Complete |
| REV-08 | Initial Implementation | Complete |
| REV-09 | Initial Implementation | Complete |
| STA-01 | Initial Implementation | Complete |
| STA-02 | Initial Implementation | Complete |
| STA-03 | Initial Implementation | Complete |
| STA-04 | Initial Implementation | Complete |
| STA-05 | Initial Implementation | Complete |
| STA-06 | Initial Implementation | Complete |
| CTL-01 | Initial Implementation | Complete |
| CTL-02 | Initial Implementation | Complete |
| CTL-03 | Initial Implementation | Complete |
| CTL-04 | Initial Implementation | Complete |
| CTL-05 | Initial Implementation | Complete |
| CTL-06 | Initial Implementation | Complete |
| CTL-07 | Initial Implementation | Complete |
| CTL-08 | Initial Implementation | Complete |
| CTL-09 | Initial Implementation | Complete |
| CTL-10 | Initial Implementation | Complete |
| CTL-11 | Initial Implementation | Complete |
| CTL-12 | Initial Implementation | Complete |
| TUI-01 | Initial Implementation | Complete |
| TUI-02 | Initial Implementation | Complete |
| TUI-03 | Initial Implementation | Complete |
| TUI-04 | Initial Implementation | Complete |
| TUI-05 | Initial Implementation | Complete |
| TUI-06 | Initial Implementation | Complete |
| TUI-07 | Initial Implementation | Complete |
| TUI-08 | Initial Implementation | Complete |
| TUI-09 | Initial Implementation | Complete |
| TUI-10 | Initial Implementation | Complete |

**Coverage:**
- v1 requirements: 69 total
- Implemented: 69
- Pending verification: 69
- Unmapped: 0 ✓

---
*Requirements defined: 2026-02-05*
*Last updated: 2026-02-05 after initial implementation*
