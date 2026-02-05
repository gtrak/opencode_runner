# Testing Patterns

**Analysis Date:** 2026-02-05

## Test Framework

**Runner:**
- **Rust Built-in Test Framework** - Uses Rust's native testing infrastructure (no external test framework)
- **Cargo.toml:** No test framework dependencies configured (Rust tests work out of the box)
- **Test Discovery:** Automatically discovered by Cargo in source files ending with `_test` or inside `#[cfg(test)] mod tests { }` blocks

**Assertion Library:**
- Built-in `assert_eq!`, `assert!`, etc. macros from `#[test]` attribute

**Run Commands:**
```bash
cargo test              # Run all tests
cargo test --workspace  # Run tests in workspace
cargo test --features   # Run tests with specific features
```

## Test File Organization

**Location:**
- **Co-located with source files** - Tests are embedded within source files in `#[cfg(test)]` blocks
- No separate `tests/` directory or `tests/*.rs` files found

**Naming:**
- Tests follow `test_<function_name>` convention within the module's `tests` block
- Example: `test_sampler_basic()`, `test_parse_decision()`, `test_iteration_tracking()`

**Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_<name>() {
        // Test implementation
    }
}
```

**Test Module Pattern:**
Tests are contained in a separate `mod tests { }` block at the end of each source file, isolated from production code using `#[cfg(test)]` attribute.

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_<function>() {
        // Test setup
        // Test execution
        // Assertions
    }

    #[test]
    fn test_<another_function>() {
        // Another test
    }
}
```

**Patterns:**
- **Setup:** Tests don't have explicit setup methods; they directly call constructors (e.g., `let state = State::new()`)
- **Teardown:** No teardown required; tests run in isolation
- **Assertions:** Uses built-in macros (`assert_eq!`, `assert!`, etc.)
- **Test isolation:** Each test creates fresh instances of test subjects

**Example from `src/state.rs` tests (lines 175-221):**
```rust
#[test]
fn test_state_new() {
    let state = State::new();
    assert_eq!(state.current_iteration(), 0);
    assert!(state.iterations().is_empty());
}

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
    assert_eq!(state.total_lines_sampled(), 50);
}
```

## Mocking

**Framework:** None (no external mocking framework configured)

**Patterns:**
- **No mocking observed** - Tests are unit tests that directly test functionality
- **Direct instantiation:** Tests create real instances of objects (e.g., `Sampler::new(5)`)
- **Dependency injection:** Components passed directly to constructors (e.g., `Sampler::new(100)`)

**What to Mock:**
- Not applicable - tests don't mock external dependencies or dependencies that aren't available

**What NOT to Mock:**
- No mocking at all - tests verify behavior of tested code directly
- External API calls are tested via integration or manual simulation (reviewer tests parse JSON strings)

## Fixtures and Factories

**Test Data:**
```rust
// Direct construction within tests
let mut sampler = Sampler::new(5);
sampler.add_line("Line 1");
sampler.add_line("Line 2");

// Struct literals
let decision = ReviewerDecision {
    action: ReviewerAction::Continue,
    reason: "Good progress".to_string(),
};
```

**Location:**
- No separate test fixtures or factories
- Test data constructed inline within tests

**Reuse:**
- No shared test utilities or fixtures observed
- Each test independently creates its own data

## Coverage

**Requirements:** Not enforced/enabled

**View Coverage:**
```bash
cargo test             # Run tests
cargo test -- --nocapture # Show output
cargo test -- --no-run    # Compile tests without running
```

**Coverage Reports:**
- No test coverage configuration found in `Cargo.toml`
- No test coverage tooling (tarpaulin, cargo-llvm-cov, etc.) configured
- No coverage threshold enforcement

**Coverage Analysis:**
- Only 3 modules have tests: `state.rs` (tests), `sampler.rs` (tests), `reviewer.rs` (tests)
- No tests found for: `client.rs`, `control_loop.rs`, `main.rs`, `server.rs`, `tui/mod.rs`

## Test Types

**Unit Tests:**
- **Scope:** Test individual functions and structs in isolation
- **Approach:** Direct instantiation with expected inputs and assertions
- **Examples:**
  - `test_sampler_basic()` - Tests Sampler::new() and basic operations
  - `test_parse_decision()` - Tests JSON parsing of reviewer decisions
  - `test_state_new()` - Tests State::new() initialization

**Integration Tests:**
- **Scope:** No integration tests observed
- **Approach:** N/A - No integration tests found

**E2E Tests:**
- **Framework:** Not used
- **Approach:** N/A - No end-to-end tests found

## Common Patterns

**Async Testing:**
Not applicable - No async tests observed in current codebase

**Error Testing:**
```rust
// Error scenarios tested by observing error propagation
#[test]
fn test_sampler_overflow() {
    let mut sampler = Sampler::new(3);
    sampler.add_line("Line 1");
    sampler.add_line("Line 2");
    sampler.add_line("Line 3");
    sampler.add_line("Line 4");
    sampler.add_line("Line 5");

    assert_eq!(sampler.line_count(), 3);
    let sample = sampler.sample();
    assert!(!sample.contains("Line 1")); // Should be evicted
}
```

**Mocking External APIs:**
Not applicable - No external API mocking in current tests
- `test_parse_decision()` simulates JSON parsing by manually creating JSON strings

**Fixture Sharing:**
Not applicable - No fixtures or shared test utilities

**Test Isolation:**
Each test runs independently with no shared state between tests within the same module.

---

*Testing analysis: 2026-02-05*
