# Roadmap: OpenCode Runner

**Created:** 2026-02-05
**Milestone:** v1.0 Refinement & Verification
**Core Value:** The control loop automatically detects when an AI worker is stuck in a loop or has completed its task, preventing runaway processes and providing actionable feedback about task execution.

## Overview

| Phase | Name | Status | Requirements | Success Criteria |
|-------|------|--------|--------------|------------------|
| 1 | Specification Audit | ○ Not started | 6 | 4 |
| 2 | Testing Infrastructure | ○ Not started | 3 | 3 |
| 3 | Error Handling | ○ Not started | 3 | 3 |
| 4 | Code Quality | ○ Not started | 3 | 3 |
| 5 | Documentation | ○ Not started | 3 | 3 |
| 6 | Release Preparation | ○ Not started | 4 | 4 |

**Total:** 6 phases | 22 requirements | 20 success criteria

---

## Phase 1: Specification Audit

**Goal:** Verify implementation matches plan.md specification and identify gaps

### Requirements Covered

| ID | Requirement | Status |
|----|-------------|--------|
| AUD-01 | Implementation audit against plan.md specification | Pending |
| AUD-02 | Error handling completeness review | Pending |
| AUD-03 | Testing coverage assessment | Pending |
| AUD-04 | Documentation completeness | Pending |
| AUD-05 | Code quality and refactoring opportunities | Pending |
| AUD-06 | Edge case handling verification | Pending |

### Success Criteria

1. **Specification Alignment Verified** — Every plan.md requirement has corresponding implementation in source code (100% coverage)
2. **Gap Report Generated** — Document any missing features, discrepancies, or deviations from specification
3. **Implementation Audit Complete** — All 69 v1 requirements verified as implemented
4. **Edge Cases Identified** — List of edge cases not covered by current implementation

### Key Outcomes

- Confirmed alignment between plan.md and implementation
- Prioritized list of gaps and refinements needed
- Baseline for subsequent phases

---

## Phase 2: Testing Infrastructure

**Goal:** Establish comprehensive test suite for core functionality

### Requirements Covered

| ID | Requirement | Status |
|----|-------------|--------|
| TEST-01 | Unit tests for all modules | Pending |
| TEST-02 | Integration tests for control loop | Pending |
| TEST-03 | End-to-end test scenarios | Pending |

### Success Criteria

1. **Unit Tests Pass** — All modules have >80% unit test coverage
2. **Integration Tests Pass** — Control loop with reviewer interactions tested
3. **E2E Scenarios Verified** — Complete task execution flows tested

### Key Outcomes

- CI/CD pipeline configured with test execution
- Test coverage reports generated
- Regression testing enabled

---

## Phase 3: Error Handling

**Goal:** Enhance error handling and improve resilience

### Requirements Covered

| ID | Requirement | Status |
|----|-------------|--------|
| ERR-01 | Improve error messages for common failures | Pending |
| ERR-02 | Add retry logic for transient failures | Pending |
| ERR-03 | Implement graceful degradation | Pending |

### Success Criteria

1. **User-Friendly Errors** — All error messages actionable and clear
2. **Transient Failure Handling** — Automatic retry for network/API failures
3. **Graceful Degradation** — System remains usable under partial failures

### Key Outcomes

- Better debugging experience
- Improved reliability under failure conditions
- Clear error escalation paths

---

## Phase 4: Code Quality

**Goal:** Improve code quality, reduce technical debt

### Requirements Covered

| ID | Requirement | Status |
|----|-------------|--------|
| QUAL-01 | Address linting issues | Pending |
| QUAL-02 | Refactor complex functions | Pending |
| QUAL-03 | Optimize performance bottlenecks | Pending |

### Success Criteria

1. **Lint Clean** — Zero warnings from clippy and cargo check
2. **Clean Code** — All functions <50 lines, clear naming, documented complexity
3. **Performance Optimized** — No blocking operations in async contexts

### Key Outcomes

- Maintainable codebase
- Clear code structure for future contributors
- Performance baseline established

---

## Phase 5: Documentation

**Goal:** Complete documentation for users and developers

### Requirements Covered

| ID | Requirement | Status |
|----|-------------|--------|
| DOCS-01 | User documentation (README, usage examples) | Pending |
| DOCS-02 | API documentation (public interfaces) | Pending |
| DOCS-03 | Architecture documentation | Pending |

### Success Criteria

1. **User Guide Complete** — README with installation, usage, and examples
2. **API Docs Complete** — All public functions documented with rustdoc
3. **Architecture Docs Complete** — System design documented for contributors

### Key Outcomes

- Usable documentation for end users
- Clear onboarding for new developers
- Architecture decisions recorded

---

## Phase 6: Release Preparation

**Goal:** Prepare for v1.0 release

### Requirements Covered

| ID | Requirement | Status |
|----|-------------|--------|
| REL-01 | Version bump and changelog | Pending |
| REL-02 | Build verification (all platforms) | Pending |
| REL-03 | Security review | Pending |
| REL-04 | Release artifacts published | Pending |

### Success Criteria

1. **Version Updated** — Version bumped to v1.0.0 with proper changelog
2. **Builds Verified** — Successful builds on Unix (primary) and Windows (stubs)
3. **Security Audited** — No critical or high vulnerabilities
4. **Artifacts Published** — Binaries available via cargo install or releases

### Key Outcomes

- Production-ready v1.0 release
- Clear versioning and release notes
- Security baseline established

---

## v2 Requirements (Future)

### Feedback Action

| ID | Requirement | Description |
|----|-------------|-------------|
| FDB-01 | Reviewer feedback injection | Allow reviewer to provide specific guidance to worker |
| FDB-02 | Message injection | Inject feedback as user message in session |
| FDB-03 | Prompt support | Requires opencode_rs messages.prompt_async() support |

### Completion Verification

| ID | Requirement | Description |
|----|-------------|-------------|
| VER-01 | File tree verification | Explore file tree to verify task completion |
| VER-02 | Test validation | Run tests or linting to validate output |
| VER-03 | Change detection | Check for expected file changes |

### Notifications

| ID | Requirement | Description |
|----|-------------|-------------|
| NOTF-01 | Completion notifications | Send notification on task completion/failure |
| NOTF-02 | Webhook support | Support webhook callbacks |
| NOTF-03 | Configurable endpoints | Configurable notification endpoints |

---

## Out of Scope

| Feature | Reason |
|---------|--------|
| Multi-session forking | Complex parallel execution, not core to control loop value |
| Plugin architecture | Premature abstraction, current use cases covered |
| Persistent state save/load | Not required for current CI/automation use cases |
| Scheduled/background execution | Separate cron/systemd concern |
| Configuration file support | CLI args sufficient for current needs |
| Real-time collaboration | Single-user tool by design |
| Feedback action | Requires opencode_rs message support |
| Completion verification via file tree | Deferred to future milestone |

---

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| AUD-01 | Phase 1 | Pending |
| AUD-02 | Phase 1 | Pending |
| AUD-03 | Phase 1 | Pending |
| AUD-04 | Phase 1 | Pending |
| AUD-05 | Phase 1 | Pending |
| AUD-06 | Phase 1 | Pending |
| TEST-01 | Phase 2 | Pending |
| TEST-02 | Phase 2 | Pending |
| TEST-03 | Phase 2 | Pending |
| ERR-01 | Phase 3 | Pending |
| ERR-02 | Phase 3 | Pending |
| ERR-03 | Phase 3 | Pending |
| QUAL-01 | Phase 4 | Pending |
| QUAL-02 | Phase 4 | Pending |
| QUAL-03 | Phase 4 | Pending |
| DOCS-01 | Phase 5 | Pending |
| DOCS-02 | Phase 5 | Pending |
| DOCS-03 | Phase 5 | Pending |
| REL-01 | Phase 6 | Pending |
| REL-02 | Phase 6 | Pending |
| REL-03 | Phase 6 | Pending |
| REL-04 | Phase 6 | Pending |

**Coverage:**
- v1 refinement requirements: 22 total
- Mapped to phases: 22
- Unmapped: 0 ✓

---

*Roadmap created: 2026-02-05*
*Last updated: 2026-02-05 after project state analysis*
