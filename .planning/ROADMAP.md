# Project Roadmap

## Phase 01: Core Infrastructure
**Goal:** Establish project foundation, CLI infrastructure, and basic control loop components.

**Progress:** 5/5 plans created (100% complete)
- [x] 01-core-infrastructure-core-setup - Basic structure, Cargo.toml, modules
- [x] 02-core-infrastructure-configuration - CLI args, review system
- [x] 03-core-infrastructure-testing - Tests, documentation, CI (gaps found)
- [x] 04-core-infrastructure-gap-closure - Fix testing and documentation gaps
- [x] 05-core-infrastructure-gap-closure - Complete documentation and CI setup

**Plans:**
- [x] 01-core-infrastructure-core-setup — Initialize project, build core infrastructure
- [x] 02-core-infrastructure-configuration — Configure CLI, add review system
- [x] 03-core-infrastructure-testing — Add tests, documentation, CI pipeline
- [x] 04-core-infrastructure-gap-closure — Fix unit tests and integration tests
- [x] 05-core-infrastructure-gap-closure — Create README and CI pipeline

**Deliverables:**
- Working CLI with argument parsing
- OpenCode server spawning and management
- Basic control loop framework
- Test suite with integration tests
- README documentation

---

## Phase 02: Control Loop
**Goal:** Implement event streaming, sampler, and core loop logic.

**Planned for:** After Phase 01 completion

**Plans:**
- [ ] 02-control-loop-event-streaming — SSE event subscription and streaming
- [ ] 02-control-loop-sampler — Event filtering and buffering
- [ ] 02-control-loop-core-loop — Main control loop orchestration

**Deliverables:**
- SSE event subscription
- Event filtering/sampling logic
- Main control loop with review cycle
- Activity logging

---

## Phase 03: Resilience
**Goal:** Add error handling, retry logic, and state management.

**Planned for:** After Phase 02 completion

**Plans:**
- [ ] 03-resilience-retry-mechanism — Exponential backoff retry logic
- [ ] 03-resilience-error-handling — Graceful error recovery
- [ ] 03-resilience-state-management — Iteration tracking and history

**Deliverables:**
- Retry logic with backoff
- Comprehensive error handling
- State persistence and tracking
- Activity logs

---

## Phase 04: UI
**Goal:** Build TUI and headless logging capabilities.

**Planned for:** After Phase 03 completion

**Plans:**
- [ ] 04-ui-tui-implementation — Ratatui-based TUI
- [ ] 04-ui-headless-mode — Enhanced headless logging
- [ ] 04-ui-diagnostics — Progress display and stats

**Deliverables:**
- Real-time TUI with progress bars
- Headless mode with enhanced logging
- Progress statistics and diagnostics

---

## Phase 05: Polish
**Goal:** Documentation, testing, examples, and deployment.

**Planned for:** After Phase 04 completion

**Plans:**
- [ ] 05-polish-documentation — User guide, API docs, examples
- [ ] 05-polish-comprehensive-testing — Unit and integration tests
- [ ] 05-polish-deployment — Release binaries, packaging

**Deliverables:**
- Complete documentation
- Comprehensive test coverage
- Release binaries for Linux, macOS, Windows
- Example workflows and usage
