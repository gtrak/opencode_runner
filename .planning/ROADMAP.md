# Roadmap: OpenCode Runner

**Created:** 2026-02-05
**Milestone:** v1.0 Refinement & Verification
**Goal:** Audit implementation against specification, ensure quality, and prepare for production use.

---

## Phase 1: Specification Audit

**Goal:** Verify implementation matches plan.md specification

**Requirements:** All 69 v1 requirements (verification focus)

**Success Criteria:**
1. Every component from plan.md exists with correct interface
2. All CLI arguments from spec are implemented
3. Error handling matches strategy in plan.md
4. Data flow matches architecture diagram
5. Default behaviors match specification

**Deliverables:**
- Audit report documenting deviations
- List of gaps or missing features
- Corrective tasks identified

---

## Phase 2: Testing Infrastructure

**Goal:** Establish comprehensive test coverage

**Requirements:** Testing completeness

**Success Criteria:**
1. Unit tests for Sampler event filtering
2. Unit tests for Reviewer retry logic
3. Unit tests for State management
4. Integration tests with mock OpenCode server
5. Integration tests with mock reviewer API
6. Test fixtures for control loop scenarios
7. CI-friendly test suite (headless mode)

**Deliverables:**
- Test suite in tests/ directory
- Mock implementations for external dependencies
- Test documentation
- CI configuration

---

## Phase 3: Error Handling & Edge Cases

**Goal:** Harden implementation against edge cases

**Requirements:** Robustness

**Success Criteria:**
1. All error paths return meaningful messages
2. Network failures handled gracefully
3. Server startup failures handled
4. SSE stream disconnection recovery
5. Invalid reviewer responses handled
6. Resource cleanup verified (no leaks)
7. Timeout scenarios tested

**Deliverables:**
- Error handling audit report
- Edge case test scenarios
- Improved error messages
- Recovery mechanisms verified

---

## Phase 4: Code Quality & Refactoring

**Goal:** Improve code quality and maintainability

**Requirements:** Maintainability

**Success Criteria:**
1. Code follows Rust idioms consistently
2. Documentation completeness (docstrings, examples)
3. Clippy warnings resolved
4. Dead code eliminated
5. Module boundaries clear
6. Public API documented
7. Internal APIs have adequate context

**Deliverables:**
- Refactored codebase
- Documentation improvements
- Clean clippy run
- Architecture decision records updated

---

## Phase 5: Documentation & Examples

**Goal:** Complete user-facing documentation

**Requirements:** Usability

**Success Criteria:**
1. README.md with usage examples
2. Configuration documentation
3. Troubleshooting guide
4. Example scenarios (basic, headless, complex task)
5. API documentation (cargo doc)
6. Changelog started

**Deliverables:**
- Complete README
- Documentation site or mdBook
- Example scripts/configurations
- User guide

---

## Phase 6: Release Preparation

**Goal:** Prepare for v1.0 release

**Requirements:** Production readiness

**Success Criteria:**
1. Version bumped to 1.0.0
2. Cargo.toml metadata complete
3. License file present
4. Binary releases configured (GitHub Actions)
5. Installation instructions tested
6. Cross-compilation verified
7. Final integration test passes

**Deliverables:**
- Git tag v1.0.0
- Published crate (if applicable)
- Release binaries
- Installation verification

---

## Traceability

| Phase | Goal | Requirements | Success Criteria |
|-------|------|--------------|------------------|
| 1 | Audit spec compliance | All 69 v1 | 5 criteria |
| 2 | Testing infrastructure | Testing completeness | 7 criteria |
| 3 | Error handling | Robustness | 7 criteria |
| 4 | Code quality | Maintainability | 7 criteria |
| 5 | Documentation | Usability | 6 criteria |
| 6 | Release | Production readiness | 7 criteria |

---

## Current Position

**Current Phase:** Not started
**Next Phase:** Phase 1 - Specification Audit
**Status:** Ready to begin refinement

---

*Roadmap created: 2026-02-05*
