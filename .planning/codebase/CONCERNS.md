# Codebase Concerns

**Analysis Date:** 2026-02-05

## Tech Debt

**Retry Count Tracking:**
- Issue: Retry count is hardcoded to 0 instead of being tracked
- Files: `src/control_loop.rs:163`
- Impact: State tracking is incomplete, preventing proper analysis of retry behavior
- Fix approach: Implement retry counting in ControlLoop and pass actual count to state.record_decision()

**Windows Support Stub Implementation:**
- Issue: Full Windows support requires opencode_rs native library, using stub
- Files: `src/opencode_stub.rs`
- Impact: Feature is unusable on Windows, only useful for type compilation
- Fix approach: Add native Windows support or provide clear Windows-specific instructions

**Hardcoded Values Without Configuration:**
- Issue: Many timeouts, delays, and sizes are hardcoded
- Files: `src/tui/mod.rs:180` (3s), `src/tui/mod.rs:139` (250ms), `src/server.rs:80` (30s), `src/server.rs:53` (1000ms)
- Impact: Cannot adjust behavior for different network conditions or use cases
- Fix approach: Extract to Configuration struct with sensible defaults

## Known Bugs

**Panic on Invalid JSON Response:**
- Symptoms: Code panics when reviewer response doesn't match expected format
- Files: `src/reviewer.rs:274, 278, 286, 290`
- Trigger: Invalid or malformed JSON from reviewer API
- Workaround: None currently
- Fix approach: Replace panic!() with proper error handling using context()

**Unwrapped Option Access:**
- Symptoms: Code calls unwrap() on Option<T> that could be None
- Files: `src/state.rs:142` (last iteration), `src/reviewer.rs:274, 286` (JSON parsing)
- Impact: Program crashes if unexpected format received
- Fix approach: Use .context() or with_context() for proper error propagation

**Ignored Sender Errors:**
- Symptoms: Send errors to UI channels are silently ignored
- Files: `src/control_loop.rs:116, 159, 226`
- Impact: Events may be lost if sender is full, breaking UI responsiveness
- Fix approach: Log send failures or implement backpressure/resend mechanism

**Health Check Non-Critical:**
- Symptoms: Health check errors only warn, don't fail connection
- Files: `src/client.rs:45`
- Impact: May connect to unresponsive server
- Fix approach: Make health check failure cause connection error

**Manual Error Construction:**
- Symptoms: Some errors created with anyhow::anyhow!() instead of .context()
- Files: `src/server.rs:61, 67, 105, 109`
- Impact: Less helpful error messages, harder debugging
- Fix approach: Replace with .context() for proper error context

**Task Title Truncation:**
- Symptoms: Task title truncated to 50 chars without warning
- Files: `src/client.rs:57`
- Impact: Loss of potentially important task information
- Fix approach: Log truncation or make truncation configurable

## Security Considerations

**URL Input Validation:**
- Risk: Base URL and task inputs not validated for malicious content
- Files: `src/client.rs:33`, CLI arguments
- Current mitigation: None
- Recommendations: Validate URLs match expected pattern, sanitize task text

**Input Sanitization in Reviewer Prompt:**
- Risk: Task description and current sample sent to external API without sanitization
- Files: `src/reviewer.rs:244-253`
- Current mitigation: JSON escaping handled by serde
- Recommendations: Add content length limits, check for potential injection

**Process Spawning:**
- Risk: External opencode serve spawned without sandboxing
- Files: `src/server.rs:44-50`
- Current mitigation: Only runs on localhost
- Recommendations: Consider sandboxing, command argument validation

**Error Information Disclosure:**
- Risk: Full error messages and stack traces sent to external reviewer API
- Files: `src/reviewer.rs:196`
- Current mitigation: Limited logging
- Recommendations: Sanitize error messages before sending to external API

## Performance Bottlenecks

**Inadequate Channel Capacity:**
- Problem: Event channels have capacity of 100, may cause backpressure
- Files: `src/main.rs:177`, `src/tui/mod.rs:102`
- Cause: 100 items may not be enough for high-volume streaming
- Improvement path: Make channel capacity configurable or calculate based on expected volume

**Blocking Sleeps During Server Startup:**
- Problem: 1-second delays waiting for server startup
- Files: `src/server.rs:53`, `src/main.rs:98`
- Cause: Arbitrary sleep times instead of polling with timeout
- Improvement path: Implement exponential backoff with proper timeout

**Unbounded Vector Growth:**
- Problem: Iterations vector can grow unbounded
- Files: `src/state.rs:32`
- Cause: No limit on number of iterations tracked
- Impact: Memory grows indefinitely across long-running sessions
- Improvement path: Implement circular buffer or max iterations cap

**Worker Output Buffer Truncation:**
- Problem: Fixed 100-line buffer without size limits or compression
- Files: `src/tui/mod.rs:46`, `src/main.rs:111`
- Cause: Hardcoded max_lines=100
- Impact: May lose important context if worker produces more than 100 lines
- Improvement path: Make buffer size configurable, consider compression

## Fragile Areas

**State Management:**
- Files: `src/state.rs`
- Why fragile: Relies on manual unwrap() calls, could crash on edge cases
- Safe modification: Replace unwrap() with proper Option handling
- Test coverage: Missing tests for edge cases

**Event Stream Processing:**
- Files: `src/sampler.rs`
- Why fragile: Complex event filtering logic, could miss or mishandle events
- Safe modification: Add event count checks, validation, and tests
- Test coverage: Limited, missing error path tests

**Control Loop State:**
- Files: `src/control_loop.rs`
- Why fragile: Multiple points where retry_count is hardcoded to 0
- Safe modification: Implement actual retry tracking
- Test coverage: Missing tests for retry scenarios

**Review Decision Parsing:**
- Files: `src/reviewer.rs`
- Why fragile: Exact JSON format expected, no graceful degradation
- Safe modification: Add more robust parsing, versioning support
- Test coverage: Limited to unit tests only

## Scaling Limits

**Connection Pool:**
- Current capacity: Single HTTP client, one SSE subscription
- Limit: Cannot handle multiple concurrent sessions
- Scaling path: Add connection pooling, separate subscriptions per session

**Memory Usage:**
- Current capacity: Unbounded iteration storage
- Limit: Memory grows indefinitely over long sessions
- Scaling path: Implement circular buffer or max iterations cap

**Channel Backpressure:**
- Current capacity: mpsc channel with fixed size
- Limit: May block event sending if full
- Scaling path: Make capacity configurable, implement backpressure handling

**Buffer Sizes:**
- Current capacity: 100 lines for sampler, 100 items for channels
- Limit: May overflow under high-volume workloads
- Scaling path: Calculate buffer sizes based on expected workload

## Dependencies at Risk

**opencode_rs:**
- Risk: Native library may break with updates, not available on Windows
- Impact: Cross-platform compatibility breaks
- Migration plan: Use native platform bindings, or provide detailed Windows fallback

**reqwest:**
- Risk: HTTP client version compatibility
- Impact: Breaking changes in future versions
- Migration plan: Pin to specific version, audit before updates

**chrono:**
- Risk: datetime library version stability
- Impact: Minor changes in formatting or parsing
- Migration plan: Pin to specific version

## Missing Critical Features

**Network Interruption Handling:**
- Problem: No recovery mechanism for network drops
- Impact: Sessions fail completely on network issues
- Fix approach: Implement reconnection logic, retry failed requests

**Graceful Shutdown:**
- Problem: No timeout for server shutdown, may hang indefinitely
- Files: `src/server.rs:145`
- Impact: Long shutdown times, unresponsive terminal
- Fix approach: Add timeout to process.wait()

**Review API Timeout:**
- Problem: No individual timeout for review API calls
- Files: `src/reviewer.rs:87`
- Impact: Could hang forever on slow reviewer API
- Fix approach: Add per-request timeout with exponential backoff

**Configuration File:**
- Problem: All settings hardcoded in CLI args
- Impact: Cannot reuse configurations across runs
- Fix approach: Add TOML/YAML configuration file support

**Export Results:**
- Problem: No way to export iteration history
- Impact: Lost information after completion
- Fix approach: Add JSON export functionality

## Test Coverage Gaps

**Event Stream Edge Cases:**
- What's not tested: Invalid events, malformed JSON, dropped events
- Files: `src/sampler.rs`, `src/control_loop.rs`
- Risk: Event processing bugs go undetected
- Priority: High

**Concurrency Scenarios:**
- What's not tested: Race conditions, channel backpressure, deadlocks
- Files: `src/main.rs`, `src/tui/mod.rs`
- Risk: UI and control loop interaction bugs
- Priority: High

**Error Recovery:**
- What's not tested: Server failure, network timeout, API failure
- Files: `src/control_loop.rs`, `src/server.rs`, `src/reviewer.rs`
- Risk: Program crashes or hangs on failures
- Priority: High

**Integration Tests:**
- What's not tested: Full control loop execution
- Files: Integration of server, client, loop, reviewer
- Risk: Component-level errors not caught
- Priority: Medium

**Test Data Factory:**
- What's not tested: Mock data generation
- Files: Tests in sampler.rs, state.rs, reviewer.rs
- Risk: Tests use limited scenarios
- Priority: Low

**Error Path Tests:**
- What's not tested: Invalid JSON, missing choices, errors in review flow
- Files: `src/reviewer.rs`
- Risk: Unhandled error cases may panic
- Priority: Medium

---

*Concerns audit: 2026-02-05*
