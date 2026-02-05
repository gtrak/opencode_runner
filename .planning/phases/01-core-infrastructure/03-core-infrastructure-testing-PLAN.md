---
phase: 01-core-infrastructure
plan: 03
type: execute
wave: 2
depends_on: [01-core-infrastructure-core-setup, 02-core-infrastructure-configuration]
files_modified:
  - src/sampler.rs
  - src/reviewer.rs
  - src/state.rs
  - tests/sampler_test.rs
  - tests/reviewer_test.rs
  - README.md
  - .github/workflows/ci.yml
autonomous: true
must_haves:
  truths:
    - "Unit tests exist for sampler and reviewer"
    - "Integration tests verify core workflows"
    - "README documentation is complete"
    - "CI pipeline is configured"
    - "Code quality checks pass"
  artifacts:
    - path: "tests/sampler_test.rs"
      provides: "Unit tests for sampler"
      exports: ["tests for Sampler"]
    - path: "tests/reviewer_test.rs"
      provides: "Unit tests for reviewer"
      exports: ["tests for ReviewerClient"]
    - path: "README.md"
      provides: "Project documentation"
      contains: ["Installation", "Usage", "Examples"]
    - path: ".github/workflows/ci.yml"
      provides: "CI/CD pipeline configuration"
      exports: ["tests", "builds"]
  key_links:
    - from: "tests/"
      to: "src/sampler.rs"
      via: "unit testing"
      pattern: "Sampler.*test"
    - from: "tests/"
      to: "src/reviewer.rs"
      via: "unit testing"
      pattern: "ReviewerClient.*test"
---

<objective>
Add comprehensive testing infrastructure, improve documentation, and configure CI/CD pipeline for project quality assurance.

Purpose: Ensure the codebase is well-tested, properly documented, and follows quality standards through automated CI checks.

Output: Test suite with unit/integration tests, complete README, CI pipeline configured, and code quality checks in place.
</objective>

<execution_context>
@./.opencode/get-shit-done/workflows/execute-plan.md
@./opencode/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md

This plan focuses on quality assurance. Sampler, reviewer, and state modules need comprehensive unit tests. The project needs documentation and CI setup.
</context>

<tasks>

<task type="auto">
  <name>Add unit tests for sampler module</name>
  <files>tests/sampler_test.rs</files>
  <action>
    Create tests/sampler_test.rs with tests for Sampler:

    Test event filtering rules from plan.md:
    1. Test PartAdded event is captured (text parts)
    2. Test PartUpdated event is captured (delta updates)
    3. Test ToolCall event is captured with summary format
    4. Test ToolResult event is filtered out
    5. Test Thinking/Reasoning events are filtered out
    6. Test System messages are filtered out
    7. Test Error events are captured
    8. Test sampling returns correct number of lines
    9. Test buffer clears correctly

    Use mock event structures or sample data. Verify all event types behave according to plan.md filtering rules (lines 202-232).

    Format tests with #[test] attribute, run with `cargo test`.
  </action>
  <verify>`cargo test sampler` passes all tests</verify>
  <done>Sampler module has comprehensive unit test coverage</done>
</task>

<task type="auto">
  <name>Add unit tests for reviewer module</name>
  <files>tests/reviewer_test.rs</files>
  <action>
    Create tests/reviewer_test.rs with tests for ReviewerClient:

    Test retry logic from plan.md:
    1. Test review() method handles successful responses
    2. Test review_with_retry() with successful response on first attempt
    3. Test review_with_retry() with retries on failures
    4. Test default decision after max retries
    5. Test exponential backoff timing

    Test prompt generation:
    6. Test reviewer context formatting
    7. Test JSON response parsing
    8. Test action enum serialization (Continue/Abort)

    Use mock HTTP client responses. Verify plan.md retry logic (lines 282-304) and prompt template (lines 308-342).

    Run tests with `cargo test reviewer`.
  </action>
  <verify>`cargo test reviewer` passes all tests</verify>
  <done>Reviewer module has comprehensive unit test coverage</done>
</task>

<task type="auto">
  <name>Create CI pipeline and documentation</name>
  <files>README.md, .github/workflows/ci.yml</files>
  <action>
    Create README.md with:
    - Project description and purpose
    - Installation instructions (cargo install or cargo build)
    - Usage examples with basic command
    - Configuration options (CLI arguments)
    - Features overview (TUI, headless, review system)

    Create .github/workflows/ci.yml with:
    - Rust CI/CD pipeline
    - Build and test workflow
    - Code quality checks (clippy, fmt)
    - Release workflow for binaries

    Include usage examples from plan.md (lines 684-710).

    Ensure CI pipeline runs `cargo test --all` and `cargo clippy`.
  </action>
  <verify>CI workflow runs successfully and passes all checks</verify>
  <done>README complete, CI pipeline configured and tested</done>
</task>

</tasks>

<verification>
- [ ] sampler_test.rs exists and passes tests
- [ ] reviewer_test.rs exists and passes tests
- [ ] README.md has installation, usage, examples
- [ ] CI workflow configured in .github/workflows/ci.yml
- [ ] CI runs successfully on repository
</verification>

<success_criteria>
The project has comprehensive test coverage for critical modules, complete user documentation, and automated CI pipeline for quality assurance.
</success_criteria>

<output>
After completion, create `.planning/phases/01-core-infrastructure/03-core-infrastructure-testing-SUMMARY.md`
</output>
