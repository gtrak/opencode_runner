# OpenCode Runner

## What This Is

A Rust-based control loop system that orchestrates OpenCode AI agent execution with periodic review by an external LLM. It spawns an OpenCode server, manages task execution through SSE event streaming, samples worker output, and uses an OpenAI-compatible reviewer API to detect looping or completion. Features an optional TUI for real-time monitoring or runs headless for CI/automation.

## Core Value

The control loop automatically detects when an AI worker is stuck in a loop or has completed its task, preventing runaway processes and providing actionable feedback about task execution.

## Requirements

### Validated

- ✓ CLI argument parsing with clap (task, working dir, models, timeouts, headless mode)
- ✓ OpenCode server process spawning with random port allocation
- ✓ Server health check polling with timeout
- ✓ OpenCode client connection and session creation
- ✓ SSE event subscription and streaming
- ✓ Sampler with 100-line ring buffer for worker output
- ✓ Event filtering (captures PartAdded, PartUpdated, ToolCall, Error; skips ToolResult, Thinking)
- ✓ Reviewer client with OpenAI-compatible API integration
- ✓ Exponential backoff retry logic for reviewer calls (3 retries)
- ✓ Reviewer decision parsing (Continue/Abort with reason)
- ✓ Control loop orchestration with iteration management
- ✓ Inactivity timeout handling (30s default)
- ✓ Max iterations guard (10 default)
- ✓ State management with iteration history
- ✓ Activity log generation
- ✓ TUI with real-time worker output and activity log display
- ✓ Headless mode for CI/automation
- ✓ Cross-platform support (Unix native, Windows stubs)
- ✓ Graceful server shutdown

### Active

- [ ] Implementation audit against plan.md specification
- [ ] Error handling completeness review
- [ ] Testing coverage assessment
- [ ] Documentation completeness
- [ ] Code quality and refactoring opportunities
- [ ] Edge case handling verification

### Out of Scope

- Feedback action (injecting reviewer guidance back to worker) — requires opencode_rs message support
- Completion verification via file tree exploration — deferred to future milestone
- Multi-session support forking — complex, not core value
- Plugin architecture for custom samplers/reviewers — premature abstraction
- Persistent state save/load — not required for current use cases
- Scheduled/background execution — separate tool concern
- Configuration file support — CLI args sufficient

## Context

**Implementation Status:** Initial implementation complete (~2,196 lines across 9 modules). Based on detailed specification in `plan.md`.

**Architecture Pattern:** Actor-oriented control loop with SSE event streaming, reviewer-driven iteration control, and optional TUI visualization.

**Key Dependencies:** 
- Tokio for async runtime
- opencode_rs (Unix only) for OpenCode server communication
- reqwest for reviewer API calls
- ratatui/crossterm for optional TUI

**Platform Constraints:** 
- Full functionality requires Unix (Linux/macOS) for opencode_rs SDK
- Windows builds use stub implementation for development/testing only

**Current State:** 
- All modules implemented per plan.md
- Ready for refinement, verification, and testing
- No known blockers

## Constraints

- **Tech Stack:** Rust 2021 edition, locked dependencies in Cargo.lock
- **Platform:** Unix required for production use (OpenCode SDK limitation)
- **External Dependencies:** Requires `opencode` CLI binary in PATH
- **LLM API:** OpenAI-compatible endpoint required for reviewer (defaults to Ollama)
- **No Breaking Changes:** Must maintain CLI compatibility with current interface

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Use `anyhow` for error handling | Simple error propagation with context | ✓ Good — consistent error handling throughout |
| Platform abstraction with stubs | Enable Windows development/compilation | ✓ Good — cross-platform builds work |
| Feature-gated TUI | Allow headless builds for CI | ✓ Good — `--no-default-features` works |
| Fixed 100-line sampler buffer | Balance context vs noise for reviewer | — Pending — needs validation with real usage |
| Exponential backoff for reviewer | Handle transient API failures gracefully | ✓ Good — prevents failures on temporary issues |
| Separate sampler clearing per iteration | Fresh context for each review | — Pending — may need tuning |

---
*Last updated: 2026-02-05 after initial codebase mapping*
