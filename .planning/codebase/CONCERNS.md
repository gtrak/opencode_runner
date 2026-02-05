# Codebase Concerns

**Analysis Date:** 2026-02-05

## Tech Debt

### Platform-Specific Code Duplication
- Issue: Extensive `#[cfg(unix)]`/`#[cfg(windows)]` blocks duplicate event handling logic across multiple files
- Files: `src/control_loop.rs` (lines 256-301), `src/client.rs` (lines 5-25), `src/sampler.rs` (lines 116-136)
- Impact: Maintenance burden, risk of divergence between platforms
- Fix approach: Create a platform abstraction trait to consolidate platform-specific logic

### Windows Stub Implementation
- Issue: `src/opencode_stub.rs` provides placeholder types that always return `NotSupported` error
- Impact: Application cannot run on Windows; only compiles
- Fix approach: Implement actual Windows-compatible OpenCode client or document Linux/macOS requirement clearly

### Unused Imports
- Issue: Multiple unused imports detected by compiler
- Files: `src/client.rs` (line 20), `src/server.rs` (line 6), `src/opencode_stub.rs` (lines 6-8, 279), `src/tui/mod.rs` (lines 18, 22)
- Impact: Code noise, slower compilation
- Fix approach: Run `cargo fix` to auto-remove unused imports

### Hardcoded Retry Count
- Issue: Line 163 in `src/control_loop.rs` hardcodes `retry_count = 0` with TODO comment
- Code: `let retry_count = 0; // TODO: Track actual retry count`
- Impact: State tracking is incomplete; metrics will always show 0 retries
- Fix approach: Modify `review_with_retry()` to return retry count alongside decision

## Known Bugs

### Unchecked Unwrap in State
- Issue: Line 142 in `src/state.rs` calls `.unwrap()` on `iterations.last()`
- Code: `let last = self.iterations.last().unwrap();`
- Impact: While currently guarded by prior check, this is fragile and could panic if logic changes
- Trigger: Call `status_summary()` or `format_activity_log()` after clearing iterations
- Workaround: Use `if let Some(last) = ...` pattern instead

### Server Process Leak on Drop
- Issue: `ServerManager::drop()` (lines 161-168 in `src/server.rs`) logs process ID but doesn't kill it
- Impact: Zombie processes may accumulate on abnormal termination
- Trigger: Panic or forced exit before `shutdown()` is called
- Workaround: Use explicit `shutdown()` call in all exit paths

### Reviewer Fallback Masking Failures
- Issue: `review_with_retry()` in `src/reviewer.rs` (lines 126-137) defaults to Continue after max retries
- Impact: System may continue indefinitely when reviewer is failing to assess progress
- Trigger: Reviewer API unavailable or returning errors
- Workaround: Consider Abort after max retries, or make fallback action configurable

## Security Considerations

### Command Injection Risk via Extra Args
- Risk: `server.rs` passes `extra_args` directly to `Command::new("opencode")` without validation
- Files: `src/server.rs` (lines 28-49)
- Impact: Malicious arguments could execute arbitrary commands
- Current mitigation: Arguments come from CLI args (requires local access)
- Recommendations: Validate args against whitelist, escape shell metacharacters, or use structured args

### No TLS Configuration for Reviewer Client
- Risk: HTTP client in `src/reviewer.rs` (lines 86-89) doesn't configure TLS
- Impact: Man-in-the-middle attacks possible on reviewer API communication
- Current mitigation: Defaults to localhost (127.0.0.1)
- Recommendations: Add TLS verification options for production deployments

### No Input Validation on Task Descriptions
- Risk: Task strings passed directly to session creation without sanitization
- Files: `src/client.rs` (line 57), `src/main.rs` (line 86)
- Impact: Potential injection into OpenCode server's prompt processing
- Current mitigation: OpenCode server should handle its own sanitization
- Recommendations: Add length limits and basic validation

## Performance Bottlenecks

### Polling-Based Event Stream
- Problem: `stream_until_review()` in `src/control_loop.rs` (lines 199-246) uses 100ms timeout polling
- Files: `src/control_loop.rs` (lines 210-212)
- Impact: CPU overhead from frequent wakeups; up to 100ms latency on events
- Cause: `tokio::time::timeout(Duration::from_millis(100), subscription.recv())`
- Improvement path: Use proper async event notification instead of polling when possible

### String Allocations in Sampler
- Problem: Every line addition clones strings in `src/sampler.rs` (lines 103-110)
- Code: `self.buffer.push_back(trimmed.to_string())`
- Impact: Memory pressure with high-volume event streams
- Improvement path: Consider using `Arc<str>` or a string pool for repeated content

### Synchronous File Operations
- Problem: No async file I/O; all operations are in-memory or over network
- Files: All modules
- Impact: Not applicable currently (no file operations)
- Note: Future features adding file I/O should use `tokio::fs`

## Fragile Areas

### Platform-Dependent Compilation
- Files: Entire codebase uses conditional compilation
- Why fragile: Small changes to event types require updates in multiple cfg blocks
- Safe modification: Always update both Unix and Windows code paths; run CI on both platforms
- Test coverage: No automated cross-platform testing visible

### Reviewer API Dependency
- Files: `src/reviewer.rs`, `src/control_loop.rs`
- Why fragile: Hard dependency on OpenAI-compatible API; no fallback for reviewer failures
- Safe modification: Add circuit breaker pattern for reviewer calls
- Test coverage: Mock reviewer client in tests

### Process Management
- Files: `src/server.rs`
- Why fragile: Relies on `opencode` binary in PATH; no version checking
- Safe modification: Add version check, better error messages for missing binary
- Test coverage: No integration tests for server lifecycle

### Event Stream Handling
- Files: `src/control_loop.rs`
- Why fragile: Completion detection relies on specific event types that may change
- Safe modification: Document event contract with OpenCode server
- Test coverage: No tests for completion detection logic

## Scaling Limits

### Single Session Limitation
- Current capacity: One session per runner instance
- Limit: Cannot orchestrate multiple parallel tasks
- Scaling path: Refactor to support multiple concurrent sessions with session pool

### Fixed Buffer Sizes
- Sampler: Hardcoded 100 lines (`src/sampler.rs` line 19, `src/main.rs` line 111)
- UiState: Hardcoded 100 lines (`src/tui/mod.rs` line 46)
- Limit: May lose important context in long-running tasks
- Scaling path: Make buffer sizes configurable via CLI args

### Synchronous Reviewer Calls
- Current: Blocks control loop while waiting for reviewer response (30s timeout)
- Limit: Cannot process events or update UI during review
- Scaling path: Make reviewer calls concurrent with event streaming

### No Horizontal Scaling Support
- Current: Single runner process
- Limit: Cannot distribute tasks across multiple workers
- Scaling path: Add queue-based architecture with worker pool

## Dependencies at Risk

### opencode_rs (Unix-only)
- Risk: Platform-specific dependency limits deployment options
- Impact: Cannot run on Windows servers
- Migration plan: Evaluate if cross-platform alternative exists, or containerize for Linux

### ratatui/crossterm (Optional but default)
- Risk: TUI dependencies add compile time and binary size
- Impact: Headless deployments still compile TUI code
- Migration plan: Keep feature flag, ensure headless mode works without TUI features

### portpicker
- Risk: Unmaintained crate (last update 2+ years ago)
- Impact: Port collision detection may not work correctly on all platforms
- Migration plan: Replace with custom port selection logic or bind to port 0

## Missing Critical Features

### Metrics and Observability
- Problem: No metrics export, no structured logging for monitoring
- Blocks: Production deployment without custom wrapper
- Priority: High for production use

### Configuration File Support
- Problem: All configuration via CLI args only
- Blocks: Complex configuration management
- Priority: Medium

### Graceful Degradation
- Problem: No fallback when reviewer is unavailable (beyond retry logic)
- Blocks: Running without reviewer in resource-constrained environments
- Priority: Medium

### Session Persistence
- Problem: No save/resume capability for long-running tasks
- Blocks: Recovery from crashes or restarts
- Priority: Low

### Authentication
- Problem: No auth on OpenCode server connection or reviewer API
- Blocks: Multi-user deployments
- Priority: High for shared environments

## Test Coverage Gaps

### Control Loop
- What's not tested: Main orchestration logic, event streaming, decision handling
- Files: `src/control_loop.rs` (301 lines, 0 tests)
- Risk: Core logic changes could break without detection
- Priority: High

### Server Manager
- What's not tested: Process spawning, health checking, shutdown
- Files: `src/server.rs` (169 lines, 0 tests)
- Risk: Platform differences in process management
- Priority: Medium (requires integration setup)

### TUI Module
- What's not tested: UI rendering, event handling, terminal management
- Files: `src/tui/mod.rs` (347 lines, 0 tests)
- Risk: UI regressions
- Priority: Low (visual testing challenging)

### Client Module
- What's not tested: Session creation, event subscription, message sending
- Files: `src/client.rs` (155 lines, 0 tests)
- Risk: API contract changes with OpenCode server
- Priority: High

### Integration Tests
- What's not tested: End-to-end workflow with real/mock OpenCode server
- Files: None found
- Risk: Module interactions untested
- Priority: High

---

*Concerns audit: 2026-02-05*
