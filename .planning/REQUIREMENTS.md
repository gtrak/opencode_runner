# Project Requirements

## Phase 01: Core Infrastructure Requirements

### User Stories

**US-01: Project initialization**
As a developer, I want to set up the project infrastructure with proper tooling.
**Acceptance Criteria:**
- Cargo.toml configured with all dependencies
- build.rs for cross-platform support
- Basic module structure in place
- TUI feature enabled
- No compilation errors

**US-02: Command-line interface**
As a user, I want to specify the task, working directory, models, and configuration via CLI.
**Acceptance Criteria:**
- All CLI arguments implemented (task, working_dir, worker_model, reviewer_url, etc.)
- Arguments validated and parsed correctly
- Help message displays available options
- --headless mode available
- --extra-args for additional opencode options

**US-03: Server management**
As a user, I want the system to spawn and manage the OpenCode server process.
**Acceptance Criteria:**
- Server spawns on random available port
- Server health verified before use
- Server graceful shutdown on completion
- Error handling for spawn failures

**US-04: Review system configuration**
As a user, I want to configure the external reviewer API for progress assessment.
**Acceptance Criteria:**
- OpenAI-compatible API URL configurable
- Reviewer model selectable
- Retry logic with exponential backoff
- Default decision (continue) on API failures

**US-05: Testing and documentation**
As a developer, I want comprehensive testing and documentation for the project.
**Acceptance Criteria:**
- Unit tests for core modules (sampler, reviewer, state)
- Integration tests for full workflow
- README with usage examples
- CI pipeline configured
- No linting or compilation warnings

### Functional Requirements

**FR-01:** Project must compile and run without errors.
**FR-02:** CLI must accept all required arguments with defaults.
**FR-03:** Server must spawn and become healthy within configurable timeout.
**FR-04:** Reviewer API must retry failed requests with exponential backoff.
**FR-05:** Tests must cover critical paths and edge cases.
**FR-06:** Documentation must include installation, usage, and examples.

### Non-Functional Requirements

**NFR-01:** Build system must support cross-platform compilation.
**NFR-02:** Code must follow Rust best practices and idioms.
**NFR-03:** Documentation must be clear and concise.
**NFR-04:** Test coverage must be at least 80% for critical components.

### Quality Gates

**Must-haves:**
- ✅ Cargo.toml configured with all dependencies
- ✅ All CLI arguments implemented
- ✅ Server management working
- ✅ Reviewer API integration
- ✅ Test suite with coverage
- ✅ README documentation
- ✅ CI pipeline configured
