# Codebase Structure

**Analysis Date:** 2026-02-05

## Directory Layout

```
[project-root]/
├── src/                    # Application source code
│   ├── main.rs            # CLI entry point and orchestration
│   ├── client.rs          # OpenCode server client wrapper
│   ├── control_loop.rs    # Main control loop implementation
│   ├── reviewer.rs        # LLM reviewer client
│   ├── sampler.rs         # Worker output sampling/buffering
│   ├── server.rs          # OpenCode server process management
│   ├── state.rs           # Iteration state tracking
│   ├── opencode_stub.rs   # Windows compatibility stubs
│   └── tui/               # Terminal UI module (optional feature)
│       └── mod.rs         # TUI implementation and event handling
├── Cargo.toml             # Rust package manifest
├── Cargo.lock             # Dependency lockfile
├── plan.md                # Project implementation plan
├── .gitignore             # Git ignore rules (/target)
├── target/                # Build artifacts (generated, not committed)
│   └── debug/            # Debug build outputs
└── .planning/             # Planning documentation (this directory)
    └── codebase/         # Codebase analysis documents
```

## Directory Purposes

**src/ (Source Root):**
- Purpose: All application source code
- Contains: 8 Rust modules (~2,200 total lines)
- Key files: See "Key File Locations" below

**src/tui/ (TUI Submodule):**
- Purpose: Terminal UI implementation isolated in directory
- Contains: Single file `mod.rs` (could expand to multiple files)
- Conditional: Only compiled with `tui` feature enabled
- Dependencies: `ratatui`, `crossterm`

**target/ (Build Artifacts):**
- Purpose: Cargo build outputs and cached dependencies
- Contains: Compiled binaries, object files, dependency builds
- Generated: Yes (by `cargo build`)
- Committed: No (in `.gitignore`)

**.planning/codebase/ (Documentation):**
- Purpose: Architecture and analysis documentation
- Contains: Markdown files describing codebase
- Generated: Yes (by analysis tools)
- Committed: Optional (project-specific)

## Key File Locations

**Entry Points:**
- `src/main.rs` (248 lines): Application entry point, CLI parsing, component orchestration

**Core Components:**
- `src/control_loop.rs` (302 lines): Main orchestration logic, iteration management
- `src/client.rs` (155 lines): OpenCode SDK wrapper, session management
- `src/reviewer.rs` (294 lines): LLM reviewer with retry logic
- `src/sampler.rs` (187 lines): Output sampling with ring buffer

**Supporting Modules:**
- `src/server.rs` (170 lines): Process management for OpenCode server
- `src/state.rs` (222 lines): Iteration tracking and history
- `src/tui/mod.rs` (347 lines): Terminal UI implementation
- `src/opencode_stub.rs` (280 lines): Windows compatibility layer

**Configuration:**
- `Cargo.toml`: Package manifest, dependencies, features
- `Cargo.lock`: Locked dependency versions

**Documentation:**
- `plan.md`: Implementation plan and design notes
- `.planning/codebase/*.md`: Architecture and analysis documents

## Naming Conventions

**Files:**
- Source files: `snake_case.rs` (e.g., `control_loop.rs`, `opencode_stub.rs`)
- Module directories: `snake_case/` (e.g., `tui/`)
- Entry point: `main.rs`

**Modules:**
- Module name matches filename (e.g., `mod control_loop;` for `control_loop.rs`)
- Re-exports at crate root: `use crate::client::OpenCodeClient;`
- Platform-specific: `opencode_stub` compiled only on Windows

**Structs:**
- PascalCase, descriptive: `ControlLoop`, `ReviewerClient`, `ServerManager`
- State structs: `State`, `UiState`
- Configuration: `ControlConfig`, `Args`

**Functions/Methods:**
- snake_case: `run()`, `spawn()`, `process_event()`
- Constructor pattern: `new()` for primary construction
- Builder pattern: `builder()` → chain → `build()`

**Enums:**
- PascalCase variants: `RunResult { Completed, Aborted, MaxIterations }`
- Action enums: `ReviewerAction { Continue, Abort }`

**Constants:**
- Hardcoded in methods or as associated constants
- No module-level constants defined

**Tests:**
- In-module tests using `#[cfg(test)]` module
- Test functions: `test_*` prefix
- Example: `test_sampler_basic`, `test_parse_decision`

## Where to Add New Code

**New Control Logic:**
- Primary code: `src/control_loop.rs` - add methods to `ControlLoop` impl
- State tracking: `src/state.rs` - add fields to `State` or `Iteration`
- Configuration: `src/main.rs` - add fields to `Args` and `ControlConfig`

**New Client Features:**
- OpenCode client: `src/client.rs` - add methods to `OpenCodeClient`
- Event handling: `src/sampler.rs` - add event matching in `process_event()`
- Platform stubs: `src/opencode_stub.rs` - mirror additions for Windows builds

**New Reviewer Features:**
- Review logic: `src/reviewer.rs` - modify `build_prompt()` or add methods
- Decision types: `src/reviewer.rs` - extend `ReviewerAction` enum

**New UI Features:**
- TUI components: `src/tui/mod.rs` - add widgets or panels
- Event types: `src/control_loop.rs` - extend `UiEvent` enum
- State display: `src/tui/mod.rs` - modify `draw_ui()` or panel functions

**New Server Management:**
- Process control: `src/server.rs` - add methods to `ServerManager`
- Health checks: `src/server.rs` - extend `spawn()` or add verification

**New Utilities:**
- Shared helpers: Add to relevant module or create new module in `src/`
- Import in `main.rs`: `mod new_module;` and re-export if needed

## Special Directories

**src/tui/:**
- Purpose: Isolates TUI dependencies behind feature flag
- Structure: Currently single-file, but structured as directory for expansion
- Generated: No
- Committed: Yes

**target/:**
- Purpose: Cargo build directory
- Generated: Yes (by cargo)
- Committed: No (in `.gitignore`)
- Note: Contains both debug and release subdirectories

**.planning/:**
- Purpose: Project planning and analysis documentation
- Contains: This documentation and other planning files
- Generated: Partially (by tools), partially manual
- Committed: Recommended for team visibility

## File Size Analysis

Largest files (may indicate complexity):
1. `src/tui/mod.rs`: 347 lines (UI rendering code)
2. `src/control_loop.rs`: 302 lines (core orchestration)
3. `src/reviewer.rs`: 294 lines (LLM integration with retries)
4. `src/opencode_stub.rs`: 280 lines (platform abstraction)
5. `src/main.rs`: 248 lines (CLI and orchestration)
6. `src/state.rs`: 222 lines (state management with tests)
7. `src/sampler.rs`: 187 lines (event sampling with tests)
8. `src/server.rs`: 170 lines (process management)
9. `src/client.rs`: 155 lines (SDK wrapper)

**Observation:** All modules are reasonably sized (under 350 lines). TUI module is largest due to widget definitions. Good separation of concerns.

---

*Structure analysis: 2026-02-05*
