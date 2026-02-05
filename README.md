# Opcode - CLI Tool for Git Repository Operations

Opcode is a powerful Command Line Interface (CLI) tool designed to automate Git repository operations with a user-friendly TUI (Terminal User Interface). Perfect for developers who need to manage their Git workflow efficiently without leaving the terminal.

## Features

- **Intuitive TUI Interface**: Navigate repository operations with keyboard controls and visual feedback
- **Headless Mode**: Run Opcode from scripts and other programs programmatically
- **Comprehensive Git Operations**: Branch management, commits, staging, merging, resolving conflicts
- **Auto-Commit on Conflict Resolution**: Automatically commits resolved merge conflicts
- **Environment Configuration**: Set repository-specific settings via environment variables
- **Multiple Repo Support**: Switch between multiple Git repositories
- **History Navigation**: Track and navigate through previous operations
- **Auto-Save**: Persistent session state across terminal sessions

## Installation

### From Source (Recommended)

```bash
git clone https://github.com/your-org/opcode.git
cd opcode
cargo build --release
cargo install --path .
```

### From crates.io (Coming Soon)

```bash
cargo install opcode
```

## Quick Start

```bash
# Initialize Opcode for current repository
opcode init

# Open Opcode interface
opcode

# Create a new branch
opcode branch new my-feature

# Commit staged changes
opcode commit

# Open Opcode (headless mode)
opcode
```

## Usage & Configuration

### CLI Arguments

| Argument    | Type     | Description                               |
|-------------|----------|-------------------------------------------|
| `opcode`    | command  | Main command - accepts subcommands        |
| `init`      | subcmd   | Initialize Opcode for current repo        |
| `open`      | subcmd   | Open Opcode TUI interface                |
| `branch`    | subcmd   | Manage branches                          |
| `commit`    | subcmd   | Commit staged changes                    |
| `pull`      | subcmd   | Fetch and merge remote changes           |
| `push`      | subcmd   | Push local commits to remote             |
| `status`    | subcmd   | Show current Git status                  |
| `log`       | subcmd   | View commit history                      |
| `help`      | subcmd   | Show help message                        |
| `--version` | flag     | Display version information              |
| `--verbose` | flag     | Enable verbose output                    |

### Environment Variables

All Opcode settings are controlled via environment variables with the `OPCODE_` prefix:

| Variable               | Description                                 | Default | Example      |
|------------------------|---------------------------------------------|---------|--------------|
| `OPCODE_INIT_AUTOSAVE` | Automatically save on every operation       | false   | `true`       |
| `OPCODE_EDITOR`        | Text editor to use for commit messages     | `vim`   | `nano`       |
| `OPCODE_MAX_LOGS`      | Maximum number of operation logs to keep   | 100     | `50`         |
| `OPCODE_BRANCH_COLORS` | Enable branch name coloring in TUI          | false   | `true`       |
| `OPCODE_THEME`         | Color theme (dark/light)                   | `dark`  | `light`      |
| `OPCODE_HEADLESS`      | Run in headless mode                       | false   | `true`       |
| `OPCODE_LOG_PATH`      | Path to save operation logs                | `./opcode.log` | `/var/log/opcode.log` |
| `OPCODE_REPO_PATH`     | Specific repository to manage              | Current | `/path/to/repo` |
| `OPCODE_CONFIG_DIR`    | Directory for Opcode configuration files   | `./.opcode` | `~/.opcode` |

### Basic Usage Examples

**1. Basic Workflow**

```bash
# Stage and commit changes using Opcode
opcode
# Use TUI to stage files and commit

# Create a new branch
opcode branch new feature/add-passwords
```

**2. Environment Configuration**

```bash
# Configure default editor
export OPCODE_EDITOR=nano

# Enable autosave on every operation
export OPCODE_INIT_AUTOSAVE=true

# Configure log location
export OPCODE_LOG_PATH=/var/log/opcode.log
```

**3. Headless Mode (Scripts)**

```bash
# Run in headless mode
opcode --headless

# Or set environment variable
export OPCODE_HEADLESS=true
opcode open
```

**4. Multiple Repositories**

```bash
# Navigate to first repo
cd /path/to/repo1
opcode init
opcode open

# Navigate to second repo
cd /path/to/repo2
opcode init
opcode open
```

**5. Viewing Status & History**

```bash
# Check Git status
opcode status

# View commit history
opcode log
```

## Examples

### Basic Usage

```bash
# Initialize Opcode
opcode init

# Open the TUI interface
opcode

# Navigate with arrow keys:
# - <Enter> to select an option
# - Space to toggle selection
# - Escape to cancel
# - Ctrl+C to exit
```

### Advanced Usage

```bash
# Enable branch coloring in TUI
export OPCODE_BRANCH_COLORS=true

# Set custom theme
export OPCODE_THEME=light

# Run with verbose output
opcode --verbose

# Check specific repository
export OPCODE_REPO_PATH=/path/to/repo
opcode open
```

### Environment Configuration

```bash
# Create .env file for project
cat > .env <<EOF
OPCODE_EDITOR=nano
OPCODE_INIT_AUTOSAVE=true
OPCODE_BRANCH_COLORS=true
OPCODE_THEME=dark
EOF

# Use the environment file
source .env
opcode open
```

### Headless Mode

```bash
# Run in script mode
opcode --headless << EOF
commit
EOF

# Use environment variable
export OPCODE_HEADLESS=true
opcode
```

## Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/your-org/opcode.git
cd opcode

# Run tests
cargo test

# Run with debug output
cargo run

# Build release version
cargo build --release
```

### Project Structure

```
opcode/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI argument parsing
│   ├── ui.rs            # TUI interface
│   ├── core.rs          # Core Git operations
│   └── config.rs        # Configuration management
├── examples/            # Usage examples
├── tests/               # Integration tests
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

## Contributing

Contributions are welcome! Please read our contributing guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- TUI powered by [ratatui](https://ratatui.rs/)
- Git operations based on [git2](https://github.com/libgit2/libgit2)
