---
phase: 01-core-infrastructure
plan: 02
type: execute
wave: 1
depends_on: [01-core-infrastructure-core-setup]
files_modified:
  - src/main.rs
  - src/config.rs
  - src/environment.rs
autonomous: true
must_haves:
  truths:
    - "CLI arguments accept all required parameters"
    - "Reviewer system is configurable via CLI"
    - "Configuration validation is enforced"
    - "Server and client components integrate successfully"
  artifacts:
    - path: "src/main.rs"
      provides: "CLI argument parsing and orchestration"
      exports: ["Args"]
    - path: "src/config.rs"
      provides: "Configuration validation and defaults"
      exports: ["ControlConfig::new"]
    - path: "src/environment.rs"
      provides: "Environment variable handling"
      exports: ["load_config_from_env"]
  key_links:
    - from: "src/main.rs"
      to: "src/config.rs"
      via: "config instantiation"
      pattern: "ControlConfig.*new.*args"
    - from: "src/main.rs"
      to: "src/server.rs"
      via: "server.spawn"
      pattern: "ServerManager::spawn.*args"
    - from: "src/main.rs"
      to: "src/reviewer.rs"
      via: "ReviewerClient::new"
      pattern: "ReviewerClient::new.*args"
---

<objective>
Configure the CLI interface, implement server and client components, and establish the review system with proper argument handling.

Purpose: Complete the initial configuration layer that allows users to specify task parameters, worker/reviewer models, and system behavior through the command line.

Output: CLI with all arguments implemented, server spawning functional, client connection working, and reviewer system configured with defaults.
</objective>

<execution_context>
@./.opencode/get-shit-done/workflows/execute-plan.md
@./.opencode/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md

This plan builds on Plan 01 by implementing the full CLI interface, server/client integration, and review system configuration. Server and client implementations already exist in src/server.rs and src/client.rs.
</context>

<tasks>

<task type="auto">
  <name>Complete CLI argument implementation</name>
  <files>src/main.rs</files>
  <action>
    Implement complete CLI argument parsing in main.rs:

    Add all required arguments from plan.md:
    - `--task`: Task description
    - `--working-dir`: Working directory (default: ".")
    - `--worker-model`: Worker model string (default: "ollama/llama3.1")
    - `--reviewer-url`: OpenAI-compatible API URL (default: "http://localhost:11434/v1")
    - `--reviewer-model`: Reviewer model (default: "ollama/llama3.1")
    - `--max-iterations`: Maximum iterations before abort (default: 10)
    - `--inactivity-timeout`: Inactivity timeout in seconds (default: 30)
    - `--headless`: Run without TUI
    - `--extra-args`: Additional opencode arguments (trailing_var_arg)

    Implement logging output for all arguments
    Implement basic validation (non-empty task, positive timeouts)

    Add help/usage message generation with clap attributes.
  </action>
  <verify>`cargo build --release` completes successfully</verify>
  <done>All CLI arguments parse correctly and are logged at startup</done>
</task>

<task type="auto">
  <name>Implement server and client integration</name>
  <files>src/main.rs</files>
  <action>
    In main.rs, complete the integration between server management and client connection:

    1. Call `ServerManager::spawn()` with working_dir, worker_model, and extra_args
    2. Get base_url from server
    3. Call `OpenCodeClient::connect(base_url)` with server URL
    4. Log connection success

    Ensure proper error handling (try/catch or ? operator)
    Add graceful shutdown for server when control loop exits

    The implementation should follow the structure from plan.md lines 86-103.
  </action>
  <verify>Server spawns successfully and client connects without errors</verify>
  <done>ServerManager and OpenCodeClient components integrate cleanly</done>
</task>

<task type="auto">
  <name>Configure review system with defaults</name>
  <files>src/main.rs</files>
  <action>
    In main.rs, configure the reviewer system:

    1. Create `ReviewerClient::new(reviewer_url, reviewer_model)`
    2. Create `Sampler::new(100)` with max sample size
    3. Create `State::new()` for state management
    4. Create `ControlConfig` with all args (task, max_iterations, inactivity_timeout)

    Add logging for reviewer, sampler, and state initialization

    Reference plan.md for correct initialization pattern (lines 106-118).
  </action>
  <verify>All components (reviewer, sampler, state, config) instantiate successfully</verify>
  <done>Review system configured with all CLI parameters</done>
</task>

</tasks>

<verification>
- [ ] All CLI arguments implemented and parsed
- [ ] Server spawns and base_url is available
- [ ] Client connects to server
- [ ] Reviewer, sampler, and state are initialized
- [ ] ControlConfig includes all parameters
- [ ] Error handling works for failures
</verification>

<success_criteria>
The CLI is fully functional, server and client components integrate properly, and the review system is configured with all user-specified parameters.
</success_criteria>

<output>
After completion, create `.planning/phases/01-core-infrastructure/02-core-infrastructure-configuration-SUMMARY.md`
</output>
