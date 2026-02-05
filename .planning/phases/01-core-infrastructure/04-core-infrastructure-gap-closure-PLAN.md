---
phase: 01-core-infrastructure
plan: 04
type: execute
wave: 1
depends_on: [01-core-infrastructure-core-setup, 02-core-infrastructure-configuration]
files_modified:
  - tests/sampler_test.rs
  - tests/reviewer_test.rs
  - tests/integration_test.rs
autonomous: true
gap_closure: true
must_haves:
  truths:
    - "Unit tests exist for sampler and reviewer"
    - "Integration tests verify core workflows"
    - "All test files compile and pass"
  artifacts:
    - path: "tests/sampler_test.rs"
      provides: "Unit tests for sampler"
      exports: ["tests for Sampler"]
      min_lines: 50
    - path: "tests/reviewer_test.rs"
      provides: "Unit tests for reviewer"
      exports: ["tests for ReviewerClient"]
      min_lines: 50
    - path: "tests/integration_test.rs"
      provides: "Integration tests for core workflows"
      exports: ["tests for full workflow"]
      min_lines: 50
  key_links:
    - from: "tests/sampler_test.rs"
      to: "src/sampler.rs"
      via: "unit testing"
      pattern: "Sampler.*test"
    - from: "tests/reviewer_test.rs"
      to: "src/reviewer.rs"
      via: "unit testing"
      pattern: "ReviewerClient.*test"
    - from: "tests/integration_test.rs"
      to: "src/"
      via: "integration testing"
      pattern: "integration.*test"
---

<objective>
Fix all testing infrastructure gaps by creating missing test files and fixing syntax errors.

Purpose: Ensure comprehensive test coverage for core modules and integration workflows to verify the existing implementation works correctly.

Output: Complete test suite with unit tests for sampler and reviewer, plus integration tests for full workflow.
</objective>

<execution_context>
@./.opencode/get-shit-done/workflows/execute-plan.md
@./opencode/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md

This plan addresses the testing gaps identified in verification. The sampler module already has extensive built-in tests but needs a separate test file. The reviewer_test.rs has a syntax error that needs fixing. Integration tests are completely missing.
</context>

<tasks>

<task type="auto">
  <name>Create sampler_test.rs with comprehensive unit tests</name>
  <files>tests/sampler_test.rs</files>
  <action>
    Create tests/sampler_test.rs with unit tests for Sampler module:

    The sampler.rs already has extensive built-in tests (lines 138-689). Extract the key test scenarios into a separate test file:

    1. Test basic sampler functionality (new, add_line, sample, line_count, clear)
    2. Test buffer overflow behavior (oldest lines evicted)
    3. Test empty line filtering (whitespace trimmed)
    4. Test event processing:
       - PartAdded event captures text content
       - PartUpdated event captures delta updates  
       - ToolCall event captures summary format
       - ToolResult event is filtered out
       - Error events are captured
       - Thinking/Progress events are filtered out
    5. Test whitespace trimming behavior
    6. Test max_lines preservation with overflow

    Reference the existing tests in src/sampler.rs (lines 138-689) but create standalone tests that work with the public API.

    Use the same mock Event patterns but ensure they work as separate unit tests. The tests should verify the same filtering rules from the implementation (lines 28-77).

    Run tests with `cargo test sampler` to verify they pass.
  </action>
  <verify>`cargo test sampler` passes all tests</verify>
  <done>sampler_test.rs created with comprehensive unit test coverage</done>
</task>

<task type="auto">
  <name>Fix syntax error in reviewer_test.rs</name>
  <files>tests/reviewer_test.rs</files>
  <action>
    Fix the syntax error in tests/reviewer_test.rs on line 23:

    Current issue: Extra closing brace `}` on line 23 inside the `create_test_context` function.

    The function ends with:
    ```rust
        };
    }  // <- This extra brace needs to be removed
    ```

    Should be:
    ```rust
        };
    ```

    After fixing the syntax error, verify the tests compile and run:
    - `cargo test reviewer` should pass
    - All existing tests should work correctly
    - The test file already has comprehensive coverage for ReviewerClient, prompt building, decision parsing, etc.

    The tests cover:
    - Context creation and validation
    - Prompt generation with/without previous summaries
    - Decision parsing (continue/abort/invalid)
    - Decision summary formatting
    - Client creation and configuration
    - JSON parsing with edge cases
    - Unicode and special character handling

    Ensure no functionality is lost when fixing the syntax error.
  </action>
  <verify>`cargo test reviewer` passes all tests</verify>
  <done>reviewer_test.rs syntax error fixed, all tests pass</done>
</task>

<task type="auto">
  <name>Create integration tests for core workflows</name>
  <files>tests/integration_test.rs</files>
  <action>
    Create tests/integration_test.rs with integration tests for the complete workflow:

    Test the full end-to-end workflow that users will experience:

    1. **Configuration Integration Test:**
       - Test ControlConfig::from_args() with all CLI arguments
       - Test ControlConfig::from_env() with environment variables
       - Test validation (non-empty task, positive timeouts)

    2. **Server-Client Integration Test:**
       - Test ServerManager::spawn() creates a server
       - Test OpenCodeClient::connect() can connect to spawned server
       - Test graceful shutdown works

    3. **Control Loop Integration Test:**
       - Test State management with iterations
       - Test Sampler integration with State
       - Test ReviewerClient integration with State
       - Test complete control loop execution (mock events)

    4. **Error Handling Integration Test:**
       - Test invalid CLI arguments are rejected
       - Test server connection failures are handled
       - Test reviewer API failures are handled gracefully

    Use integration test patterns:
- Use `#[tokio::test]` for async tests
- Create temporary directories for testing
- Mock server responses where needed
- Test both success and failure paths
- Verify cleanup and resource management

    Integration tests should verify that all components work together as designed in the core infrastructure.

    Run with `cargo test --test integration`.
  </action>
  <verify>`cargo test --test integration` passes all integration tests</verify>
  <done>Integration tests created for complete workflow verification</done>
</task>

</tasks>

<verification>
- [ ] sampler_test.rs exists and passes all tests
- [ ] reviewer_test.rs syntax error fixed and passes tests  
- [ ] integration_test.rs exists and passes all tests
- [ ] All tests compile without errors
- [ ] Test coverage includes unit and integration tests
</verification>

<success_criteria>
All testing infrastructure gaps are closed: sampler has unit tests, reviewer has working unit tests, and integration tests verify the complete workflow. The test suite provides confidence that the core infrastructure works as intended.
</success_criteria>

<output>
After completion, create `.planning/phases/01-core-infrastructure/04-core-infrastructure-gap-closure-SUMMARY.md`
</output>