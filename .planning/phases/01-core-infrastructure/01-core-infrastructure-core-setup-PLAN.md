---
phase: 01-core-infrastructure
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - Cargo.toml
  - build.rs
  - src/main.rs
  - src/client.rs
  - src/server.rs
autonomous: true
must_haves:
  truths:
    - "Project compiles and runs without errors"
    - "TUI feature is enabled by default"
    - "All core modules exist and compile"
    - "CLI argument parsing is functional"
  artifacts:
    - path: "Cargo.toml"
      provides: "Project dependencies and configuration"
      contains: "[package] dependencies including tokio, clap, ratatui"
    - path: "src/main.rs"
      provides: "Main entry point and CLI parsing"
      exports: ["Args"]
    - path: "src/client.rs"
      provides: "OpenCode client wrapper"
      exports: ["OpenCodeClient::connect"]
    - path: "src/server.rs"
      provides: "Server lifecycle management"
      exports: ["ServerManager::spawn"]
  key_links:
    - from: "src/main.rs"
      to: "Cargo.toml"
      via: "dependencies"
      pattern: "tokio.*clap.*ratatui"
    - from: "src/main.rs"
      to: "src/client.rs"
      via: "module import"
      pattern: "mod client.*use client::OpenCodeClient"
---

<objective>
Initialize the OpenCode Runner project with basic infrastructure, core modules, and tooling setup. Establish the foundation for the control loop system.

Purpose: Create a working Rust project structure that compiles with all necessary dependencies and provides a skeleton for the control loop components.

Output: Compilable Rust project with core modules (server, client, sampler, reviewer, state), TUI feature enabled, and basic entry point ready for control loop integration.
</objective>

<execution_context>
@./.opencode/get-shit-done/workflows/execute-plan.md
@./.opencode/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md

This plan establishes the initial project structure. All modules (server, client, sampler, reviewer, state) are already implemented in the codebase but need to be integrated into the build system and main.rs.
</context>

<tasks>

<task type="auto">
  <name>Configure project dependencies and structure</name>
  <files>Cargo.toml</files>
  <action>
    Review and update Cargo.toml to ensure:
    - All required dependencies are included (tokio, clap, serde, etc.)
    - TUI feature is enabled by default: `default = ["tui"]`
    - Build script (build.rs) is created for cross-platform support
    - Features are properly documented in Cargo.toml

    Ensure no duplicate dependencies and all features are compatible. Verify Cargo.lock is up to date if project existed before.
  </action>
  <verify>Run `cargo build` and verify no dependency errors</verify>
  <done>Project compiles successfully with all required dependencies</done>
</task>

<task type="auto">
  <name>Implement basic main.rs entry point</name>
  <files>src/main.rs</files>
  <action>
    Create main.rs with:
    - Command-line argument parsing using clap
    - Basic module structure (mod client, server, sampler, reviewer, state)
    - Placeholder for control loop integration
    - TUI mode fallback logic (check feature flag)
    - Basic logging initialization

    Do NOT implement full control loop logic yet - just the scaffolding. Focus on:
    - Parsing CLI args
    - Setting up logging
    - Importing all modules
    - Placeholder for server.spawn() and control loop.run()

    Keep code clean and follow existing style from plan.md.
  </action>
  <verify>`cargo build --release` completes successfully</verify>
  <done>main.rs exists, compiles, and exports all core modules</done>
</task>

</tasks>

<verification>
- [ ] Cargo.toml has all dependencies from plan (tokio, clap, serde, ratatui, etc.)
- [ ] Project compiles with `cargo build`
- [ ] TUI feature is enabled by default
- [ ] All module files exist and compile
- [ ] main.rs has proper CLI argument structure
- [ ] build.rs handles cross-platform needs
</verification>

<success_criteria>
The project builds successfully, compiles all modules without warnings, and provides a solid foundation for Phase 02 control loop implementation.
</success_criteria>

<output>
After completion, create `.planning/phases/01-core-infrastructure/01-core-infrastructure-core-setup-SUMMARY.md`
</output>
