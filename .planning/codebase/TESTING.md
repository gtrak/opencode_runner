# Testing Patterns

**Analysis Date:** 2026-02-05

## Test Framework

**Runner:** Built-in Rust test runner (`cargo test`)
- No external test framework detected
- Standard `#[test]` attribute for unit tests
- Standard `#[tokio::test]` expected for async tests (not currently used)

**Assertion Library:**
- Standard Rust `assert!`, `assert_eq!`, `assert_ne!`
- No external assertion libraries

**Run Commands:**
```bash
cargo test                    # Run all tests
cargo test <module_name>      # Run specific module tests
cargo test --no-run           # Compile tests without running
cargo test -- --nocapture     # Show println! output
```

## Test File Organization

**Location:** Inline tests in source files
- Pattern: `#[cfg(test)] mod tests` at bottom of each module
- No separate `tests/` directory detected

**Files with tests:**
- `src/sampler.rs` - Lines 138-186
- `src/state.rs` - Lines 175-221
- `src/reviewer.rs` - Lines 267-293

**Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_description() {
        // Arrange
        let input = ...;
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler_basic() {
        let mut sampler = Sampler::new(5);
        sampler.add_line("Line 1");
        assert_eq!(sampler.line_count(), 1);
        assert!(sampler.sample().contains("Line 1"));
    }

    #[test]
    fn test_sampler_overflow() {
        // Tests circular buffer eviction
    }

    #[test]
    fn test_parse_decision() {
        // Tests JSON deserialization
    }
}
```

**Patterns:**
- Tests use same module namespace (no `super::super::` needed)
- Direct struct instantiation in tests
- Assert on public methods only

## Mocking

**Framework:** None detected
- No mock libraries in `Cargo.toml`
- Tests use real implementations or simple stubs

**Patterns:**
- Direct struct instantiation
- Manual state setup
- Example from `src/state.rs`:
```rust
#[test]
fn test_iteration_tracking() {
    let mut state = State::new();
    
    state.start_iteration();
    assert_eq!(state.current_iteration(), 1);
    
    state.record_decision(
        50,
        ReviewerDecision {
            action: ReviewerAction::Continue,
            reason: "Good progress".to_string(),
        },
        0,
    );
    
    assert_eq!(state.iterations().len(), 1);
}
```

**What to Mock (when needed):**
- External API calls (reviewer client)
- File system operations
- Time-based operations
- Random number generation

**Current approach for external APIs:**
- Tests in `src/reviewer.rs` test JSON parsing only, not HTTP calls
- No integration tests for API layer detected

## Fixtures and Factories

**Test Data:**
- Inline literals in tests
- No centralized fixture files
- Example:
```rust
let json = r#"{"action": "continue", "reason": "Making progress"}"#;
let decision: ReviewerDecision = serde_json::from_str(json).unwrap();
```

**Factory Pattern:**
- Not used - prefer direct instantiation with `::new()`

**Location:**
- Test data defined within test functions
- No `tests/fixtures/` or `tests/data/` directories

## Coverage

**Requirements:** Not enforced
- No coverage tools configured
- No CI coverage gates detected

**View Coverage:**
```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html
```

**Current Coverage Analysis:**

| Module | Lines | Tests | Coverage Notes |
|--------|-------|-------|----------------|
| `src/sampler.rs` | 187 | 3 | Tests buffer overflow, empty lines |
| `src/state.rs` | 222 | 3 | Tests iteration tracking, max iterations |
| `src/reviewer.rs` | 294 | 2 | Tests JSON deserialization only |
| `src/client.rs` | 155 | 0 | No tests - external API wrapper |
| `src/control_loop.rs` | 302 | 0 | No tests - orchestration logic |
| `src/server.rs` | 170 | 0 | No tests - process management |
| `src/tui/mod.rs` | 347 | 0 | No tests - UI rendering |
| `src/opencode_stub.rs` | 280 | 0 | No tests - stub implementation |
| `src/main.rs` | 248 | 0 | No tests - entry point |

## Test Types

**Unit Tests:**
- Present in: `sampler`, `state`, `reviewer`
- Scope: Single module, isolated functionality
- Approach: Direct method calls on instantiated structs

**Integration Tests:**
- None detected
- No `tests/` directory at crate root

**E2E Tests:**
- None detected
- Manual testing via CLI execution

## Common Patterns

**Async Testing:**
Not currently used, but when needed:
```rust
#[tokio::test]
async fn test_async_feature() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

**Error Testing:**
```rust
#[test]
fn test_parse_abort() {
    let json = r#"{"action": "abort", "reason": "Stuck in loop"}"#;
    let decision: ReviewerDecision = serde_json::from_str(json).unwrap();
    
    match decision.action {
        ReviewerAction::Abort => {}
        _ => panic!("Expected Abort"),
    }
}
```

**State Setup:**
```rust
#[test]
fn test_max_iterations() {
    let mut state = State::new();
    
    state.start_iteration();
    state.start_iteration();
    state.start_iteration();
    
    assert!(!state.is_max_iterations(5));
    assert!(state.is_max_iterations(3));
    assert!(state.is_max_iterations(2));
}
```

**JSON Testing:**
```rust
#[test]
fn test_parse_decision() {
    let json = r#"{"action": "continue", "reason": "Making progress"}"#;
    let decision: ReviewerDecision = serde_json::from_str(json).unwrap();
    
    match decision.action {
        ReviewerAction::Continue => {}
        _ => panic!("Expected Continue"),
    }
    assert_eq!(decision.reason, "Making progress");
}
```

## Testing Gaps

**High Priority:**
1. `src/control_loop.rs` - Core orchestration logic untested
2. `src/client.rs` - OpenCode client wrapper untested
3. `src/server.rs` - Process management untested
4. Async code paths - No async tests present

**Medium Priority:**
1. `src/tui/mod.rs` - UI rendering (may require special mocking)
2. Error handling paths - Most tests cover happy path only
3. Platform-specific code - Windows stub untested

**Low Priority:**
1. `src/main.rs` - Entry point logic
2. `src/opencode_stub.rs` - Stub implementation (returns errors)

## Recommended Testing Additions

**Integration Tests:**
```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_full_control_loop() {
    // Setup mock server
    // Run control loop with test task
    // Assert on results
}
```

**Property-Based Testing:**
Consider `proptest` for:
- Sampler buffer behavior
- State iteration tracking
- JSON serialization roundtrips

**Mocking for External APIs:**
Consider `mockall` or `wiremock` for:
- Reviewer API calls
- OpenCode server interactions

---

*Testing analysis: 2026-02-05*
