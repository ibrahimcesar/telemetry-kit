# Makefile Documentation

## Overview

The Makefile provides **70+ commands** organized into 17 categories for comprehensive project automation. All commands include colorful output and helpful emoji for better visibility.

## Quick Start

```bash
# Show all available commands
make help

# Setup development environment
make setup

# Run tests
make test

# Start development with auto-reload
make dev
```

## Categories

### 📚 General
- `make help` - Show all commands with descriptions
- `make info` - Display project information
- `make ascii-art` - Show project banner

### 🔨 Development
- `make setup` - One-time development environment setup
- `make dev` - Auto-reload development mode
- `make build` - Build debug version
- `make build-release` - Build optimized release
- `make clean` - Remove build artifacts

### 🧪 Testing
- `make test` - Run all tests
- `make test-all` - Comprehensive test suite
- `make test-unit` - Unit tests only
- `make test-integration` - Integration tests
- `make test-watch` - Auto-run tests on file change
- `make coverage` - Generate coverage report

### 🎨 Code Quality
- `make fmt` - Format code
- `make fmt-check` - Check formatting
- `make lint` - Run clippy lints
- `make lint-fix` - Auto-fix warnings
- `make check` - Quick validation

### 📖 Documentation
- `make doc` - Build docs
- `make doc-open` - Build and open in browser
- `make doc-check` - Validate documentation

### 🔒 Security
- `make audit` - Security vulnerability scan
- `make deny-check` - Dependency policy check
- `make outdated` - Check for old dependencies
- `make update` - Update dependencies

### 🎯 Examples
- `make example-basic` - Run basic example
- `make example-local` - Local-only tracking
- `make example-sync` - Sync example
- `make example-e2e` - End-to-end test

### 🚀 Server
- `make server-up` - Start server (Docker)
- `make server-down` - Stop server
- `make server-logs` - View logs
- `make server-restart` - Restart server
- `make server-clean` - Clean all data

### 🔄 Synchronization
- `make check-sync` - Verify schema alignment
- `make sync-status` - Show sync status

### 📦 Git & Release
- `make git-status` - Show git status
- `make commit-phase1` - Commit Phase 1 changes
- `make push` - Push to remote
- `make tag` - Create version tag

### 🚀 Publishing
- `make publish-check` - Dry-run publish
- `make publish` - Publish to crates.io

### 🤖 CI/CD Simulation
- `make ci` - Simulate CI locally
- `make ci-full` - Full CI with security

### ⚡ Quick Commands
- `make quick-test` - Fast: format + lint + test
- `make quick-fix` - Fast: format + auto-fix
- `make pre-commit` - Run before commit
- `make pre-push` - Run before push

### 🛠️ Utilities
- `make install-tools` - Install dev tools
- `make deps` - Show dependency tree
- `make lines` - Count lines of code
- `make todos` - Find TODO comments
- `make stats` - Project statistics

### ☕ Special
- `make coffee` - Take a break reminder
- `make stats` - Detailed statistics
- `make ascii-art` - Show banner

## Common Workflows

### First Time Setup
```bash
make setup              # Install tools
make build              # Verify build works
make test               # Run tests
make doc-open           # View documentation
```

### Daily Development
```bash
make dev                # Start auto-reload mode
# ... make changes ...
make quick-test         # Fast checks
make commit-phase1      # Commit (if Phase 1)
```

### Before Committing
```bash
make pre-commit         # Format + lint + test
git add .
git commit -m "..."
```

### Before Pushing
```bash
make pre-push           # Full CI simulation
make push               # Push to remote
```

### Running Examples
```bash
make server-up          # Start server first
make example-e2e        # Run E2E test
make server-logs        # Check logs
make server-down        # Stop when done
```

### Code Quality Checks
```bash
make fmt                # Format code
make lint               # Check lints
make audit              # Security scan
make deny-check         # Policy check
make doc-check          # Doc validation
```

### Full Test Suite
```bash
make test-all           # Comprehensive tests
# Or individually:
make test-unit
make test-integration
make test-doc
```

### Coverage Report
```bash
make coverage           # Generate HTML report
open coverage/index.html
```

### Publishing Workflow
```bash
make publish-check      # Dry run
make ci-full            # All checks pass
make tag                # Create version tag
make publish            # Publish to crates.io
```

## Color Reference

The Makefile uses ANSI colors for better readability:
- 🔵 **Cyan** - Command output, headers
- 🟢 **Green** - Success messages
- 🟡 **Yellow** - Warnings, info
- 🔴 **Red** - Errors (from tools)
- **Bold** - Section headers, important info

## Requirements

### Required
- `make` - Build automation (pre-installed on macOS/Linux)
- `rust` + `cargo` - Rust toolchain

### Optional (installed via `make install-tools`)
- `cargo-watch` - Auto-reload development
- `cargo-audit` - Security scanning
- `cargo-deny` - Dependency policy
- `cargo-tarpaulin` - Code coverage
- `cargo-outdated` - Dependency updates
- `cargo-bloat` - Binary size analysis

### Optional (for server)
- `docker` + `docker-compose` - Server deployment

## Tips

1. **Tab Completion**: Use tab to autocomplete make targets
2. **Multiple Targets**: Run multiple: `make fmt lint test`
3. **Parallel Execution**: Some targets run in parallel automatically
4. **CI Simulation**: Always run `make ci` before pushing
5. **Quick Feedback**: Use `make dev` for instant feedback while coding

## Troubleshooting

### "make: command not found"
Install build tools:
```bash
# macOS
xcode-select --install

# Linux
sudo apt-get install build-essential
```

### "cargo: command not found"
Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Colors not showing
Your terminal may not support ANSI colors. The Makefile will still work, just without colors.

### "cargo-watch: command not found"
Install development tools:
```bash
make install-tools
```

## Customization

You can customize the Makefile by:
1. Adding new targets in the appropriate section
2. Following the existing format: `target: ## 🎯 Description`
3. Using the color variables for consistent output
4. Adding to the help categories with `##@ Category Name`

## Integration with IDEs

### VS Code
Add to `.vscode/tasks.json`:
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Make: Test",
      "type": "shell",
      "command": "make test",
      "group": "test"
    }
  ]
}
```

### IntelliJ/CLion
Right-click on Makefile → "Run Make Target"

## Performance

- Most commands complete in <10 seconds
- `make dev` provides instant feedback (<1s)
- `make ci-full` is comprehensive (~2-5 minutes)
- `make coverage` may take several minutes

## See Also

- [PRODUCTION_PLAN.md](PRODUCTION_PLAN.md) - Development roadmap
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [scripts/](scripts/) - Shell scripts used by Makefile

---

**Need Help?** Run `make help` for a complete list of commands!
