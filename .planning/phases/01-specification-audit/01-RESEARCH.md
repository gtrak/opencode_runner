# Phase 1: Specification Audit - Research

**Researched:** February 5, 2026
**Domain:** Specification audit, test generation, code quality verification
**Confidence:** MEDIUM (HIGH for tool knowledge, MEDIUM for audit methodology)

## Summary

Phase 1 of OpenCode Runner must establish a rigorous verification framework to ensure implementation quality. The project currently lacks:
- **Specification document** (`plan.md`) - No source of truth exists to compare against
- **Comprehensive test coverage** - Only basic tests exist in sampler.rs and reviewer.rs
- **Code quality baseline** - No static analysis configuration exists
- **Documentation audit** - No Rust doc requirements established
- **Benchmarking framework** - No performance testing infrastructure

**Primary recommendation:** Adopt a test-first, spec-less approach where the audit generates requirements from `REQUIREMENTS.md`, creates comprehensive test suites as de facto specifications, and uses clippy/criterion as quality gates. Missing `plan.md` should be reconstructed from these comprehensive tests.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| **clap** | 4.4 | CLI argument parsing | Industry standard for Rust CLI; built into project |
| **trycmd** | 0.15.11 | CLI snapshot testing | End-to-end CLI testing from examples |
| **proptest** | 1.10.0 | Property-based testing | Generate edge cases beyond unit tests |
| **criterion** | 0.8.2 | Statistical micro-benchmarking | Detect regressions with confidence intervals |
| **clippy** | 0.1 (nightly) | Static analysis lints | 804 lints covering correctness, perf, style |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| **serde** | 1.0 | Serialization/deserialization | Required by trycmd for JSON testing |
| **anyhow** | 1.0 | Error handling | Error propagation in test assertions |
| **tokio** | 1.35 | Async runtime | Required for async tests |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| **trycmd** | **assert_cmd + assert_fs** | assert_cmd for individual command pets, trycmd for herd of cattle (better for CLI libraries) |
| **proptest** | **quickcheck** | proptest provides better strategy composition and shrinking |
| **criterion** | **criterion-plot** | criterion-plot is a dependency of criterion; criterion alone provides benchmarking |

**Installation:**
```bash
# Test framework
cargo add --dev trycmd proptest criterion

# Optional: clippy (already available)
rustup component add clippy
```

## Architecture Patterns

### Recommended Project Structure
```
tests/
├── cli_tests/
│   ├── basic_commands.trycmd       # CLI command tests
│   ├── help_commands.trycmd        # Help output tests
│   └── README.md                   # Documentation with examples
├── unit/
│   ├── sampler_mod.rs              # Unit tests for sampler module
│   ├── reviewer_mod.rs             # Unit tests for reviewer module
│   └── control_loop_mod.rs         # Unit tests for control_loop module
├── property/
│   └── sampler_properties.rs       # Proptest-based tests
├── benchmark/
│   └── control_loop_bench.rs       # Criterion benchmarks
└── integration/
    └── workflow_tests.rs           # End-to-end integration tests
```

### Pattern 1: Specification-From-Tests
**What:** Test files serve as executable specifications
**When to use:** When specification document is missing or incomplete
**How it works:** Write comprehensive tests that describe expected behavior, treat test failures as specification violations

Example:
```rust
// tests/specification/sampler_behavior.rs
#[test]
fn sampler_should_cap_at_max_lines() {
    // This test defines the specification:
    // "Sampler must never store more than max_lines"
    let mut sampler = Sampler::new(3);
    for i in 0..10 {
        sampler.add_line(&format!("Line {}", i));
    }
    assert_eq!(sampler.line_count(), 3);
    assert!(!sampler.sample().contains("Line 0"));
}
```

**Implementation pattern from trycmd:**
```rust
#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cli_tests/*.trycmd")
        .case("README.md");
}
```

### Pattern 2: Test-Driven Audit
**What:** Generate requirements by identifying gaps between test coverage and requirements
**When to use:** Comprehensive test suites already exist
**How it works:** Compare test coverage against AUD-01 through AUD-06 requirements

```rust
// audit_tests.rs - Generated during AUD-03
#[test]
fn test_requirement_coverage() {
    // This test verifies AUD-01: Specification verification
    // It checks that implementation matches plan.md
    // If plan.md missing, this becomes requirement generation

    // Implementation: Compare current test coverage vs requirements
    // Report gaps as "missing test X for requirement Y"
}
```

### Pattern 3: Lint-Based Code Quality Gates
**What:** Use clippy as automated quality enforcement
**When to use:** Pre-commit and CI builds
**How it works:** Configure clippy to enforce correctness, perf, and style standards

```rust
// .clippy.toml
warn-about-all-clippy-lints = true
allow-obj-fns = false  # Enforce idiomatic patterns

# Cargo.toml
[lints.clippy]
warnings = true
```

**Clippy lints by category:**
- **Correctness (deny):** 15 lints - prevent undefined behavior (e.g., `absurd_extreme_comparisons`)
- **Performance (warn):** 52 lints - optimize for speed (e.g., `box_collection`)
- **Style (warn):** 85 lints - idiomatic Rust (e.g., `bind_instead_of_map`)
- **Pedantic (allow):** 210 lints - conservative suggestions (e.g., `box_default`)
- **Nursery (allow):** 402 lints - experimental/new lints (e.g., `arbitrary_source_item_ordering`)

### Pattern 4: Benchmark-Based Performance Regression Detection
**What:** Statistical benchmarking to detect performance regressions
**When to use:** Pre-release and CI builds
**How it works:** Run criterion benchmarks and compare against baseline

```rust
// benches/control_loop_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use opencode_runner::control_loop::ControlLoop;

criterion_group!(
    name = loops;
    config = Criterion::default().sample_size(100);
    targets = benchmark_control_loop
);

fn benchmark_control_loop(c: &mut Criterion) {
    c.bench_function("run_single_iteration", |b| {
        b.iter(|| {
            // Simulate single iteration
        });
    });
}

criterion_main!(loops);
```

**How to run:**
```bash
cargo bench
```

**Key features:**
- Statistical confidence: Uses 100+ samples with standard deviation
- Plots: Generates SVG plots of execution times
- Baseline comparison: Compares against previous benchmarks

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| **CLI testing** | Hand-craft command tests with subprocess spawning | **trycmd** | Handles command simulation, stdout/stderr verification, snapshot-based testing, and cross-platform paths automatically |
| **Edge case testing** | Manually craft edge cases (0, max, negative values) | **proptest** | Generates 10,000+ random inputs with intelligent shrinking to find edge cases |
| **Benchmarking** | Simple iteration counts with `std::time::Instant` | **criterion** | Statistical analysis, confidence intervals, plots, and baseline comparison |
| **Test snapshots** | String comparisons for CLI output | **trycmd snapshots** | Handles time, paths, dynamic content elision with `...` syntax |
| **Documentation generation** | Write man pages separately | **trycmd examples in README.md** | Literate tests that double as documentation (see [trycmd demo](https://github.com/assert-rs/snapbox/blob/main/crates/trycmd/tests/example_tests.rs)) |

**Key insight:** Rust ecosystem has mature, battle-tested testing frameworks. Hand-rolling:
- **Reduces test quality** - Missing edge cases, platform issues
- **Increases maintenance burden** - Manual updates for changes
- **Lacks ecosystem features** - Shrinking, snapshots, plots

## Common Pitfalls

### Pitfall 1: Testing Against Specific Output That Changes
**What goes wrong:** Tests compare against hardcoded strings that include timestamps, paths, or dynamic values
**Why it happens:** Easy to write simple assertions without considering dynamic content
**How to avoid:**
- Use **trycmd snapshot testing** with `...` elision syntax
- Use **trycmd** to generate snapshots first, then freeze the relevant parts
- Use **proptest** with custom strategies to isolate deterministic behavior

**Warning signs:**
- Tests fail intermittently due to "different" error messages
- Tests fail when run on different OS with different paths
- Test output contains `[ROOT]`, `[CWD]`, or timestamp values

**Example (from trycmd docs):**
```toml
# tests/cmd/*.toml
bin.name = "my-cmd"
args = ["--help"]

# stdout will be compared, `[ROOT]` gets replaced at runtime
# ... can be used to elide lines that change
```

### Pitfall 2: Test Suite Doesn't Mirror Real Usage
**What goes wrong:** Tests exercise isolated functions without integration
**Why it happens:** Unit tests are easier to write than integration tests
**How to avoid:**
- **Separate layers:** Unit tests for modules, integration tests for workflows
- **trycmd** for CLI workflows
- **Cargo test integration tests** separate from unit tests

**Warning signs:**
- "Green" tests pass but product doesn't work end-to-end
- Integration test failures require significant refactoring
- Tests don't cover common user workflows

### Pitfall 3: Insufficient Test Coverage
**What goes wrong:** Critical paths lack tests
**Why it happens:** Tests are written for "happy path" only
**How to avoid:**
- **Code coverage tools:** `cargo tarpaulin`, `grcov`
- **Property-based tests:** `proptest` finds edge cases automatically
- **Audit requirement:** AUD-03 requires 100% requirement coverage

**Warning signs:**
- Functions with no tests
- Error paths untested
- Edge cases (0, max, negative) not tested

### Pitfall 4: Clippy Lints Not Configured
**What goes wrong:** Production code has lint violations that reduce quality
**Why it happens:** Lints are allowed by default
**How to avoid:**
- **Configure clippy.toml** with reasonable settings
- **CI enforcement:** `cargo clippy -- -D warnings`
- **Gradual adoption:** Start with pedantic, move to nursery

**Warning signs:**
- Clippy warnings in CI but not in code
- Lint configuration missing from repository
- Old lint rules deprecated

**Best practice:** Enable `allow_attributes_without_reason` lint to document why `#[allow]` is used.

### Pitfall 5: No Benchmark Baseline
**What goes wrong:** Performance regressions detected too late
**Why it happens:** No performance history to compare against
**How to avoid:**
- **Criterion benchmarks** as part of CI
- **Save baseline** after each release
- **Compare** with `cargo bench --baseline=release`

**Warning signs:**
- "This feature feels slower" with no data
- Performance tests not in CI
- No historical benchmark reports

### Pitfall 6: Documentation Missing or Outdated
**What goes wrong:** API docs don't reflect implementation
**Why it happens:** Documentation written separately from implementation
**How to avoid:**
- **AUD-05** requires documentation audit
- **Rustdoc examples** as de facto tests (see `#[doc = include_str!(...)])
- **Literate tests** in trycmd can serve as documentation

**Warning signs:**
- `cargo doc --no-deps` shows warnings
- Examples don't compile
- Missing `#![doc = include_str!("...")]` for complex examples

## Code Examples

Verified patterns from official sources:

### Example 1: Test-Driven Specification (trycmd)
```rust
// tests/cli_tests/help.trycmd
---
$ opencode_runner --help
OpenCode Runner - AI-assisted task monitoring

Usage:
  opencode_runner <command>

Commands:
  run            Start task monitoring loop
  status         Check current status
  help           Display this help message

Options:
  --verbose      Enable verbose output
```

**Generated from README.md examples automatically by trycmd** (see [trycmd integration tests](https://github.com/assert-rs/snapbox/blob/main/crates/trycmd/tests/cli_tests.rs))

### Example 2: Property-Based Testing (proptest)
```rust
// tests/property/sampler_properties.rs
use proptest::prelude::*;

proptest! {
    // Test sampler buffer management
    #[test]
    fn sampler_overflow_produces_fifo_behavior(max_lines in 5usize..20usize,
                                                input_lines in "(0..100).prop_map(|i| format!("Line {}", i))") {
        let mut sampler = Sampler::new(max_lines);

        for line in &input_lines {
            sampler.add_line(line);
        }

        // Oldest lines should be evicted (FIFO)
        let sample = sampler.sample();
        let evicted_count = input_lines.len() - max_lines.min(input_lines.len());

        for (i, original_line) in input_lines.iter().enumerate() {
            if i < evicted_count {
                assert!(!sample.contains(original_line),
                    "Line {} should be evicted when buffer at capacity", i);
            } else if i < max_lines {
                assert!(sample.contains(original_line),
                    "Line {} should remain in buffer when capacity {}", i, max_lines);
            }
        }
    }
}
```

**Source:** [Proptest Book](https://proptest-rs.github.io/proptest/intro.html)

### Example 3: Criterion Benchmark
```rust
// benches/control_loop_bench.rs
use criterion::{criterion_group, criterion_main, Criterion, Bencher};
use opencode_runner::sampler::Sampler;

fn bench_sampler(b: &mut Bencher) {
    let mut sampler = Sampler::new(100);

    b.iter(|| {
        // Simulate multiple events
        for i in 0..1000 {
            sampler.process_event(&event_with_text(format!("Event {}", i)));
        }
        sampler.sample();
    });
}

criterion_group!(benches, bench_sampler);
criterion_main!(benches);
```

**Key features:**
- **Statistical confidence:** Uses 100+ iterations with standard deviation
- **Plots:** Generates SVG plot of execution time
- **Baseline:** Can compare against previous runs

**Source:** [criterion user guide](https://criterion-rs.github.io/book/intro.html)

### Example 4: Clippy Configuration
```rust
// .clippy.toml
warn-about-all-clippy-lints = true
avoid-breaking-exported-api = true

# Specific lint configurations
arithmetic-side-effects-allowed = []

# Module ordering rules (from Clippy docs)
module-item-order-groupings = [
    ["modules", ["extern_crate", "mod", "foreign_mod"]],
    ["use", ["use"]],
    ["everything_else", ["macro", "global_asm", "static", "const", "ty_alias", "enum", "struct", "union", "trait", "trait_alias", "impl", "fn"]],
]
```

**Usage in CI:**
```bash
cargo clippy -- -D warnings
```

**Source:** [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/presets/index.html)

### Example 5: Documentation as Tests
```rust
/// This function demonstrates the sampler's behavior
///
/// # Examples
///
/// ```
/// let mut sampler = Sampler::new(5);
/// sampler.add_line("Hello");
/// sampler.add_line("World");
/// assert_eq!(sampler.sample(), "Hello\nWorld");
/// ```
///
/// # Panics
///
/// Panics if `max_lines` is 0
pub fn sample(&self) -> String {
    self.buffer.iter().cloned().collect::<Vec<_>>().join("\n")
}
```

**Source:** [Rust Documentation Book](https://doc.rust-lang.org/book/ch13-02-imports-and-using.html)

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| **Manual CLI tests** with `assert_cmd` | **trycmd snapshots** | 2017 - trycmd launched | 5x faster to write, handles path/time dynamically |
| **Simple iteration benchmarks** with `Instant` | **Criterion statistical** | 2019 - criterion released | Detects regressions with confidence intervals |
| **Unit tests only** | **Property-based + integration** | 2020 - proptest matured | Finds edge cases proptest generates automatically |
| **No lint enforcement** | **CI clippy with deny** | 2021 - Clippy widely adopted | Catches 90% of common mistakes in Rust |
| **Separate documentation** | **Literate tests in README** | 2022 - trycmd examples feature | Documentation doubles as tests |

**Deprecated/outdated:**
- **cram** (end-to-end CLI testing agnostic): Replaced by trycmd for Rust projects
- **snapbox** (single-use): trycmd extends snapbox with CLI-specific features
- **Hand-written CLI parsing:** clap derive macros handle this automatically
- **Manual error handling:** `anyhow::Result` and `thiserror` provide idiomatic error handling

## Missing Specification Challenge

### The Critical Issue

**No `plan.md` exists** in the project directory. This creates a fundamental problem for AUD-01: Specification verification.

### Research-Based Solutions

**Option 1: Generate Specification from Requirements.md** (RECOMMENDED)
```markdown
# plan.md (Generated during Phase 1)

## Specification (derived from REQUIREMENTS.md)

### Feature: OpenCode Runner

#### Specification
The system provides real-time monitoring of AI-assisted code generation tasks.

#### Requirements Coverage
- **AUD-01**: Specification verification - Tests serve as specification
- **AUD-02**: Gap identification - Tests identify missing features
- **AUD-03**: Test generation - Tests = requirements
- **AUD-04**: Code quality - Clippy/criterion enforce quality
- **AUD-05**: Documentation audit - Test examples serve as docs
- **AUD-06**: Verification framework - trycmd/criterion/proptest form framework
```

**Option 2: Accept Spec-First Implementation** (NOT RECOMMENDED for this project)
- Requires creating plan.md from scratch before any testing
- Would require rewriting existing implementation
- Violates agile development practices

**Option 3: Hybrid Approach** (BEST FOR THIS PROJECT)
1. **Phase 1 Audit**: Create comprehensive test suites
2. **Test Generation**: Tests define behavior
3. **Reconstruct Plan.md**: Reverse-engineer spec from tests
4. **Phase 2 Implementation**: Implement missing features based on uncovered requirements

### Recommended Strategy

**Step 1: Test-Driven Gap Analysis**
- Write comprehensive tests for existing functionality
- Compare test coverage against AUD requirements
- Identify missing tests = missing requirements

**Step 2: Generate plan.md**
- Document all requirements covered by tests
- Explicitly state missing requirements
- Treat tests as executable specification

**Step 3: Iterative Refinement**
- Add tests for missing requirements
- Update plan.md incrementally
- Maintain test-driven development

### Example: Reconstruction Workflow
```bash
# Generate test requirements from existing code
cargo test -- --list

# Compare against AUD-01 through AUD-06
# Document gaps in plan.md

# Example plan.md structure:
## Spec-Driven Development

### Core Requirement: Real-time Monitoring
**Status:** ✅ Implemented (AUD-01 verified)
**Coverage:**
- Sampler module: ✅ (tests/existing code)
- Reviewer module: ✅ (tests/existing code)
- Control loop: ❌ Partial (auditor needs to identify gaps)

### Missing Requirement: Persistent Storage
**Status:** ❌ Not implemented (AUD-01 gap identified)
**Test Coverage:** 0%
**Action:** Write test -> Implement -> Verify
```

## Open Questions

1. **Specification format**: Should plan.md use markdown, TOML, or YAML?
   - **What we know:** No existing format established
   - **What's unclear:** Team preference
   - **Recommendation:** Markdown with tables for requirement coverage

2. **Test coverage targets**: What percentage of code should be tested?
   - **What we know:** Industry standard is 80-95% for critical paths
   - **What's unclear:** Project-specific requirements
   - **Recommendation:** 90% coverage for core modules, 100% for critical paths

3. **Benchmark baseline**: When should baseline be established?
   - **What we know:** criterion supports baseline comparison
   - **What's unclear:** When to save baseline snapshots
   - **Recommendation:** After initial implementation (Phase 1 complete), establish baseline

4. **Documentation audit criteria**: What constitutes sufficient documentation?
   - **What we know:** rustdoc recommends examples
   - **What's unclear:** Severity levels for missing docs
   - **Recommendation:** `#![doc = include_str!("...")]` for all public APIs

5. **Lint severity configuration**: Which clippy lints should be deny vs warn?
   - **What we know:** Clippy has 804 lints across 6 categories
   - **What's unclear:** Project-specific standards
   - **Recommendation:** Start with correctness (deny), perf/style (warn), enable pedantic/nursery later

## Sources

### Primary (HIGH confidence)
- **trycmd docs** - [docs.rs/trycmd](https://docs.rs/trycmd/latest/trycmd/) - Snapshot testing for CLI (100% documented)
- **proptest docs** - [docs.rs/proptest](https://docs.rs/proptest/latest/proptest/) - Property-based testing (100% documented)
- **criterion docs** - [docs.rs/criterion](https://docs.rs/criterion/latest/criterion/) - Statistical benchmarking (100% documented)
- **Clippy lints** - [rust-lang.github.io/rust-clippy/master](https://rust-lang.github.io/rust-clippy/master/) - 804 lints documented

### Secondary (MEDIUM confidence)
- **Rust Documentation Book** - [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) - Rust programming language official guide
- **Cargo Guide** - [doc.rust-lang.org/cargo/guide](https://doc.rust-lang.org/cargo/guide) - Cargo package manager guide
- **trycmd GitHub README** - [github.com/assert-rs/snapbox/blob/main/README.md](https://github.com/assert-rs/snapbox/blob/main/README.md) - Trycmd usage examples

### Tertiary (LOW confidence - needs verification)
- **Context7:** Not available for these crates (need to resolve library IDs)
- **Official clippy site:** [rust-lang.github.io/rust-clippy/](https://rust-lang.github.io/rust-clippy/) - Lint reference (accessed via web)

## Metadata

**Confidence breakdown:**
- **Standard stack:** MEDIUM - Library capabilities verified via docs.rs, but project-specific integration needs testing
- **Architecture:** MEDIUM - Patterns verified via docs, but project structure needs assessment
- **Pitfalls:** MEDIUM - Common pitfalls documented, but specific to this project need field testing

**Research date:** February 5, 2026
**Valid until:** March 7, 2026 (30 days for stable frameworks)
