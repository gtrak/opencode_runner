---
phase: 01-core-infrastructure
plan: 05
type: execute
wave: 1
depends_on: [01-core-infrastructure-core-setup, 02-core-infrastructure-configuration]
files_modified:
  - README.md
  - .github/workflows/ci.yml
autonomous: true
gap_closure: true
must_haves:
  truths:
    - "README documentation is complete"
    - "CI pipeline is configured"
    - "Project has proper installation and usage documentation"
  artifacts:
    - path: "README.md"
      provides: "Project documentation"
      contains: ["Installation", "Usage", "Examples", "Configuration"]
      min_lines: 100
    - path: ".github/workflows/ci.yml"
      provides: "CI/CD pipeline configuration"
      exports: ["build", "test", "quality checks"]
      min_lines: 50
  key_links:
    - from: "README.md"
      to: "Cargo.toml"
      via: "installation instructions"
      pattern: "cargo install|cargo build"
    - from: ".github/workflows/ci.yml"
      to: "src/"
      via: "automated testing"
      pattern: "cargo test|cargo clippy"
---

<objective>
Create complete project documentation and configure CI/CD pipeline for automated quality assurance.

Purpose: Provide users with clear documentation for installation and usage, and ensure code quality through automated CI checks on every commit.

Output: Comprehensive README.md and functional GitHub Actions workflow for build, test, and quality checks.
</objective>

<execution_context>
@./.opencode/get-shit-done/workflows/execute-plan.md
@./opencode/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/ROADMAP.md
@.planning/STATE.md

This plan addresses the documentation and CI gaps identified in verification. The project needs a README for users and a CI pipeline to ensure code quality. The core functionality is working but lacks proper documentation and automated testing.
</context>

<tasks>

<task type="auto">
  <name>Create comprehensive README.md documentation</name>
  <files>README.md</files>
  <action>
    Create README.md with complete project documentation:

    Include the following sections:

    1. **Project Title & Description:**
       - Clear project name and one-sentence description
       - Brief explanation of what the tool does

    2. **Installation:**
       - Prerequisites (Rust toolchain)
       - Install from crates.io: `cargo install opencode_runner`
       - Install from source: `cargo build --release`
       - Build requirements and dependencies

    3. **Quick Start:**
       - Basic usage example with common arguments
       - Simple command that works out of the box
       - Expected output format

    4. **Usage & Configuration:**
       - Complete CLI argument reference
       - All 9 CLI arguments with descriptions:
         - `--task`: Task description
         - `--working-dir`: Working directory
         - `--worker-model`: Worker model string
         - `--reviewer-url`: Reviewer API URL
         - `--reviewer-model`: Reviewer model
         - `--max-iterations`: Maximum iterations
         - `--inactivity-timeout`: Timeout in seconds
         - `--headless`: Run without TUI
         - `--extra-args`: Additional arguments

    5. **Environment Variables:**
       - List all OPCODE_ prefixed environment variables
       - Examples of configuration via environment

    6. **Features:**
       - TUI interface (enabled by default)
       - Headless mode
       - Review system integration
       - Configuration validation

    7. **Examples:**
       - Basic usage example
       - Advanced configuration example
       - Environment variable usage example
       - Headless mode example

    8. **Development:**
       - Build from source instructions
       - Run tests: `cargo test`
       - Run with features: `cargo run --features tui`

    9. **License & Contributing:**
       - License information
       - Contributing guidelines

    Use markdown formatting with code blocks, bullet points, and proper sections. Ensure all examples are syntactically correct and match the actual CLI implementation from src/main.rs.

    Reference the existing configuration and CLI structure from the completed core setup and configuration plans.
  </action>
  <verify>README.md exists and contains all required sections with valid examples</verify>
  <done>README.md provides complete documentation for users and developers</done>
</task>

<task type="auto">
  <name>Create GitHub Actions CI pipeline</name>
  <files>.github/workflows/ci.yml</files>
  <action>
    Create .github/workflows/ci.yml with comprehensive CI/CD pipeline:

    Include the following jobs:

    1. **Test Job:**
       - Runs on Ubuntu, Windows, and macOS
       - Tests against stable, beta, and nightly Rust
       - Steps:
         - Checkout code
         - Install Rust toolchain
         - Cache cargo dependencies
         - Run `cargo build --verbose`
         - Run `cargo test --verbose`
         - Test with TUI feature: `cargo test --features tui`
         - Test without TUI feature: `cargo test --no-default-features`

    2. **Quality Check Job:**
       - Runs `cargo clippy -- -D warnings`
       - Runs `cargo fmt -- --check`
       - Checks for documentation with `cargo doc --no-deps`
       - Runs audit with `cargo audit` if available

    3. **Build Release Job:**
       - Runs `cargo build --release`
       - Verifies release build works
       - Checks binary size if relevant

    4. **Integration Test Job:**
       - Runs the integration tests specifically
       - `cargo test --test integration`
       - Tests with sample configurations

    Workflow configuration:
    - Trigger on push to main branch
    - Trigger on pull requests
    - Use caching for cargo dependencies
    - Set proper permissions
    - Use recent stable actions versions
    - Fail fast on errors
    - Upload artifacts if needed

    Ensure the workflow works with the actual project structure:
- Default features include TUI
- All tests compile and pass
- Cross-platform compatibility
- Integration tests are included

    Test the workflow by checking syntax and ensuring it references existing files correctly.
  </action>
  <verify>CI workflow runs successfully and passes all checks</verify>
  <done>CI pipeline configured with build, test, and quality checks</done>
</task>

</tasks>

<verification>
- [ ] README.md exists with all required sections
- [ ] README.md contains valid installation and usage examples
- [ ] .github/workflows directory exists
- [ ] ci.yml workflow file exists and is valid
- [ ] CI pipeline builds, tests, and runs quality checks
- [ ] Documentation examples match actual CLI implementation
</verification>

<success_criteria>
The project has complete documentation for users and automated CI pipeline for quality assurance. Users can understand how to install and use the tool, and code quality is maintained through automated checks on every commit.
</success_criteria>

<output>
After completion, create `.planning/phases/01-core-infrastructure/05-core-infrastructure-gap-closure-SUMMARY.md`
</output>