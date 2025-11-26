# Makefile for telemetry-kit
# Comprehensive build, test, and development automation

# Colors and formatting
RESET := \033[0m
BOLD := \033[1m
RED := \033[31m
GREEN := \033[32m
YELLOW := \033[33m
BLUE := \033[34m
MAGENTA := \033[35m
CYAN := \033[36m

# Project info
PROJECT_NAME := telemetry-kit
VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d '"' -f 2)

# Detect OS
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    OS_TYPE := linux
endif
ifeq ($(UNAME_S),Darwin)
    OS_TYPE := macos
endif

# Default target
.DEFAULT_GOAL := help

##@ General

.PHONY: help
help: ## 📚 Show this help message
	@echo ""
	@echo "  $(BOLD)$(CYAN)🔭 $(PROJECT_NAME) v$(VERSION)$(RESET)"
	@echo "  $(YELLOW)Privacy-first telemetry toolkit for Rust$(RESET)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "$(BOLD)Usage:$(RESET)\n  make $(CYAN)<target>$(RESET)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(CYAN)%-20s$(RESET) %s\n", $$1, $$2 } /^##@/ { printf "\n$(BOLD)%s$(RESET)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
	@echo ""

.PHONY: info
info: ## 📋 Show project information
	@echo "$(BOLD)$(CYAN)Project Information$(RESET)"
	@echo "  Name:    $(PROJECT_NAME)"
	@echo "  Version: $(VERSION)"
	@echo "  OS:      $(OS_TYPE)"
	@echo "  Rust:    $(shell rustc --version 2>/dev/null || echo 'not installed')"
	@echo "  Cargo:   $(shell cargo --version 2>/dev/null || echo 'not installed')"
	@echo ""

##@ Development

.PHONY: setup
setup: ## 🚀 Setup development environment
	@echo "$(CYAN)🚀 Setting up development environment...$(RESET)"
	@chmod +x scripts/*.sh
	@./scripts/dev-setup.sh

.PHONY: dev
dev: ## 🔄 Start development with auto-reload (requires cargo-watch)
	@echo "$(CYAN)🔄 Starting development mode with auto-reload...$(RESET)"
	@cargo watch -x 'test --lib' -x 'clippy'

.PHONY: build
build: ## 🔨 Build the project
	@echo "$(CYAN)🔨 Building project...$(RESET)"
	@cargo build
	@echo "$(GREEN)✅ Build complete$(RESET)"

.PHONY: build-release
build-release: ## 🚀 Build optimized release binary
	@echo "$(CYAN)🚀 Building release binary...$(RESET)"
	@cargo build --release
	@echo "$(GREEN)✅ Release build complete$(RESET)"
	@ls -lh target/release/$(PROJECT_NAME) 2>/dev/null || true

.PHONY: build-all
build-all: ## 🏗️  Build all targets (lib, bins, examples)
	@echo "$(CYAN)🏗️  Building all targets...$(RESET)"
	@cargo build --all-targets
	@echo "$(GREEN)✅ All targets built$(RESET)"

.PHONY: clean
clean: ## 🧹 Clean build artifacts
	@echo "$(CYAN)🧹 Cleaning build artifacts...$(RESET)"
	@cargo clean
	@rm -rf coverage/
	@echo "$(GREEN)✅ Clean complete$(RESET)"

##@ Testing

.PHONY: test
test: ## 🧪 Run all tests
	@echo "$(CYAN)🧪 Running tests...$(RESET)"
	@cargo test
	@echo "$(GREEN)✅ Tests passed$(RESET)"

.PHONY: test-all
test-all: ## 🔬 Run comprehensive test suite (format, lint, test, doc)
	@echo "$(CYAN)🔬 Running comprehensive test suite...$(RESET)"
	@./scripts/test-all.sh

.PHONY: test-unit
test-unit: ## 🧪 Run unit tests only
	@echo "$(CYAN)🧪 Running unit tests...$(RESET)"
	@cargo test --lib
	@echo "$(GREEN)✅ Unit tests passed$(RESET)"

.PHONY: test-integration
test-integration: ## 🔗 Run integration tests
	@echo "$(CYAN)🔗 Running integration tests...$(RESET)"
	@cargo test --test '*'
	@echo "$(GREEN)✅ Integration tests passed$(RESET)"

.PHONY: test-doc
test-doc: ## 📚 Run documentation tests
	@echo "$(CYAN)📚 Running documentation tests...$(RESET)"
	@cargo test --doc
	@echo "$(GREEN)✅ Doc tests passed$(RESET)"

.PHONY: test-watch
test-watch: ## 👀 Run tests in watch mode (requires cargo-watch)
	@echo "$(CYAN)👀 Watching for changes and running tests...$(RESET)"
	@cargo watch -x test

.PHONY: coverage
coverage: ## 📊 Generate code coverage report
	@echo "$(CYAN)📊 Generating code coverage...$(RESET)"
	@cargo tarpaulin --out Html --output-dir coverage
	@echo "$(GREEN)✅ Coverage report generated: coverage/index.html$(RESET)"

##@ Code Quality

.PHONY: fmt
fmt: ## 🎨 Format code
	@echo "$(CYAN)🎨 Formatting code...$(RESET)"
	@cargo fmt
	@echo "$(GREEN)✅ Code formatted$(RESET)"

.PHONY: fmt-check
fmt-check: ## 🔍 Check code formatting
	@echo "$(CYAN)🔍 Checking code formatting...$(RESET)"
	@cargo fmt -- --check
	@echo "$(GREEN)✅ Formatting check passed$(RESET)"

.PHONY: lint
lint: ## 🔍 Run clippy lints
	@echo "$(CYAN)🔍 Running clippy lints...$(RESET)"
	@cargo clippy --all-features --all-targets -- -D warnings
	@echo "$(GREEN)✅ Lints passed$(RESET)"

.PHONY: lint-fix
lint-fix: ## 🔧 Auto-fix clippy warnings
	@echo "$(CYAN)🔧 Auto-fixing clippy warnings...$(RESET)"
	@cargo clippy --fix --allow-dirty --allow-staged
	@echo "$(GREEN)✅ Fixes applied$(RESET)"

.PHONY: check
check: ## ✅ Run cargo check
	@echo "$(CYAN)✅ Running cargo check...$(RESET)"
	@cargo check --all-features --all-targets
	@echo "$(GREEN)✅ Check passed$(RESET)"

##@ Documentation

.PHONY: doc
doc: ## 📖 Build documentation
	@echo "$(CYAN)📖 Building documentation...$(RESET)"
	@cargo doc --no-deps --document-private-items
	@echo "$(GREEN)✅ Documentation built$(RESET)"

.PHONY: doc-open
doc-open: ## 🌐 Build and open documentation in browser
	@echo "$(CYAN)🌐 Building and opening documentation...$(RESET)"
	@cargo doc --no-deps --document-private-items --open

.PHONY: doc-check
doc-check: ## 🔍 Check documentation for warnings
	@echo "$(CYAN)🔍 Checking documentation...$(RESET)"
	@RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
	@echo "$(GREEN)✅ Documentation check passed$(RESET)"

##@ Security

.PHONY: audit
audit: ## 🔒 Run security audit
	@echo "$(CYAN)🔒 Running security audit...$(RESET)"
	@cargo audit
	@echo "$(GREEN)✅ Security audit complete$(RESET)"

.PHONY: deny-check
deny-check: ## 🚫 Check dependency policy
	@echo "$(CYAN)🚫 Checking dependency policy...$(RESET)"
	@cargo deny check
	@echo "$(GREEN)✅ Dependency policy check passed$(RESET)"

.PHONY: outdated
outdated: ## 📦 Check for outdated dependencies
	@echo "$(CYAN)📦 Checking for outdated dependencies...$(RESET)"
	@cargo outdated
	@echo "$(GREEN)✅ Outdated check complete$(RESET)"

.PHONY: update
update: ## 🔄 Update dependencies
	@echo "$(CYAN)🔄 Updating dependencies...$(RESET)"
	@cargo update
	@echo "$(GREEN)✅ Dependencies updated$(RESET)"

##@ Examples

.PHONY: example-basic
example-basic: ## 🎯 Run basic example
	@echo "$(CYAN)🎯 Running basic example...$(RESET)"
	@cargo run --example basic

.PHONY: example-local
example-local: ## 💾 Run local-only example
	@echo "$(CYAN)💾 Running local-only example...$(RESET)"
	@cargo run --example local_only

.PHONY: example-sync
example-sync: ## 🔄 Run sync example
	@echo "$(CYAN)🔄 Running sync example...$(RESET)"
	@cargo run --example sync_example

.PHONY: example-e2e
example-e2e: ## 🌐 Run end-to-end sync test
	@echo "$(CYAN)🌐 Running end-to-end sync test...$(RESET)"
	@echo "$(YELLOW)⚠️  Make sure server is running: cd server && docker compose up -d$(RESET)"
	@cargo run --example e2e_sync_test

.PHONY: examples
examples: ## 📦 Build all examples
	@echo "$(CYAN)📦 Building all examples...$(RESET)"
	@cargo build --examples
	@echo "$(GREEN)✅ All examples built$(RESET)"

##@ Server

.PHONY: server-up
server-up: ## 🚀 Start the server (Docker Compose)
	@echo "$(CYAN)🚀 Starting server...$(RESET)"
	@cd server && docker compose up -d
	@echo "$(GREEN)✅ Server started$(RESET)"
	@echo "$(YELLOW)🌐 Server running at http://localhost:8080$(RESET)"

.PHONY: server-down
server-down: ## ⬇️  Stop the server
	@echo "$(CYAN)⬇️  Stopping server...$(RESET)"
	@cd server && docker compose down
	@echo "$(GREEN)✅ Server stopped$(RESET)"

.PHONY: server-logs
server-logs: ## 📋 Show server logs
	@echo "$(CYAN)📋 Server logs (Ctrl+C to exit):$(RESET)"
	@cd server && docker compose logs -f

.PHONY: server-restart
server-restart: ## 🔄 Restart the server
	@echo "$(CYAN)🔄 Restarting server...$(RESET)"
	@cd server && docker compose restart
	@echo "$(GREEN)✅ Server restarted$(RESET)"

.PHONY: server-build
server-build: ## 🔨 Build server Docker image
	@echo "$(CYAN)🔨 Building server Docker image...$(RESET)"
	@cd server && docker compose build
	@echo "$(GREEN)✅ Server image built$(RESET)"

.PHONY: server-clean
server-clean: ## 🧹 Clean server data (stop + remove volumes)
	@echo "$(CYAN)🧹 Cleaning server data...$(RESET)"
	@cd server && docker compose down -v
	@echo "$(GREEN)✅ Server data cleaned$(RESET)"

##@ Synchronization

.PHONY: check-sync
check-sync: ## 🔄 Check schema sync between repos
	@echo "$(CYAN)🔄 Checking schema synchronization...$(RESET)"
	@./scripts/check-sync.sh

.PHONY: sync-status
sync-status: ## 📊 Show sync status
	@echo "$(CYAN)📊 Sync status:$(RESET)"
	@echo ""
	@echo "$(BOLD)Public Repo:$(RESET)  $(shell pwd)"
	@echo "  Version: $(VERSION)"
	@echo "  Schema:  $(shell grep -o 'SCHEMA_VERSION.*=.*"[0-9.]*"' src/event.rs | grep -o '[0-9.]*' || echo 'unknown')"
	@echo ""
	@if [ -d "$$HOME/Dev/telemetry-kit.dev" ]; then \
		echo "$(BOLD)Private Repo:$(RESET) $$HOME/Dev/telemetry-kit.dev"; \
		echo "  Status:  Found ✅"; \
	else \
		echo "$(BOLD)Private Repo:$(RESET) Not found ⚠️"; \
	fi
	@echo ""

##@ Git & Release

.PHONY: git-status
git-status: ## 📊 Show git status with emoji
	@echo "$(CYAN)📊 Git status:$(RESET)"
	@git status

.PHONY: commit-phase1
commit-phase1: ## 💾 Commit Phase 1 changes
	@echo "$(CYAN)💾 Committing Phase 1 changes...$(RESET)"
	@git add .
	@git commit -m "feat: Phase 1 complete - foundation & infrastructure\n\nBREAKING CHANGE: Version bump from 0.0.1 to 0.2.0-alpha.1\n\nThis is the first functional alpha release with working SDK and\nsync protocol. Not suitable for production use yet.\n\nInfrastructure:\n- CI/CD pipeline fixed and enhanced (6 jobs)\n- Security scanning and vulnerability disclosure\n- Development tooling and helper scripts\n- Comprehensive documentation and roadmap\n\nSee PRODUCTION_PLAN.md for roadmap to v1.0.0."
	@echo "$(GREEN)✅ Changes committed$(RESET)"
	@echo "$(YELLOW)💡 Run 'make push' to push to remote$(RESET)"

.PHONY: push
push: ## ⬆️  Push to remote
	@echo "$(CYAN)⬆️  Pushing to remote...$(RESET)"
	@git push origin main
	@echo "$(GREEN)✅ Pushed to remote$(RESET)"

.PHONY: tag
tag: ## 🏷️  Create and push version tag
	@echo "$(CYAN)🏷️  Creating tag v$(VERSION)...$(RESET)"
	@git tag -a v$(VERSION) -m "Release v$(VERSION)"
	@git push origin v$(VERSION)
	@echo "$(GREEN)✅ Tag v$(VERSION) created and pushed$(RESET)"

##@ Publishing

# Get macros version
MACROS_VERSION := $(shell grep '^version' telemetry-kit-macros/Cargo.toml | head -1 | cut -d '"' -f 2)

.PHONY: publish-check
publish-check: ## ✅ Check if ready to publish
	@echo "$(CYAN)✅ Checking if ready to publish...$(RESET)"
	@cargo publish --dry-run
	@echo "$(GREEN)✅ Publish check passed$(RESET)"

.PHONY: publish-macros-check
publish-macros-check: ## ✅ Check if macros crate is ready to publish
	@echo "$(CYAN)✅ Checking macros crate...$(RESET)"
	@cargo publish -p telemetry-kit-macros --dry-run
	@echo "$(GREEN)✅ Macros publish check passed$(RESET)"

.PHONY: publish-macros
publish-macros: ## 📦 Publish telemetry-kit-macros to crates.io
	@echo "$(CYAN)📦 Publishing telemetry-kit-macros v$(MACROS_VERSION) to crates.io...$(RESET)"
	@echo "$(YELLOW)⚠️  This will publish telemetry-kit-macros v$(MACROS_VERSION)$(RESET)"
	@read -p "Continue? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		cargo publish -p telemetry-kit-macros; \
		echo "$(GREEN)✅ telemetry-kit-macros published$(RESET)"; \
	else \
		echo "$(YELLOW)❌ Publish cancelled$(RESET)"; \
	fi

.PHONY: publish
publish: ## 📦 Publish telemetry-kit to crates.io
	@echo "$(CYAN)📦 Publishing to crates.io...$(RESET)"
	@echo "$(YELLOW)⚠️  This will publish version $(VERSION) to crates.io$(RESET)"
	@read -p "Continue? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		cargo publish; \
		echo "$(GREEN)✅ Published to crates.io$(RESET)"; \
	else \
		echo "$(YELLOW)❌ Publish cancelled$(RESET)"; \
	fi

.PHONY: publish-all
publish-all: ## 🚀 Publish all crates in correct order (macros first, then main)
	@echo "$(BOLD)$(CYAN)🚀 Publishing all crates to crates.io$(RESET)"
	@echo ""
	@echo "$(BOLD)Step 1: Sync versions$(RESET)"
	@echo "  Main crate:   v$(VERSION)"
	@echo "  Macros crate: v$(MACROS_VERSION)"
	@echo ""
	@if [ "$(VERSION)" != "$(MACROS_VERSION)" ]; then \
		echo "$(RED)❌ Version mismatch! Please ensure both crates have the same version.$(RESET)"; \
		echo "$(YELLOW)   Update telemetry-kit-macros/Cargo.toml version to $(VERSION)$(RESET)"; \
		exit 1; \
	fi
	@echo "$(GREEN)✅ Versions match$(RESET)"
	@echo ""
	@echo "$(BOLD)Step 2: Pre-publish checks$(RESET)"
	@cargo publish -p telemetry-kit-macros --dry-run
	@echo "$(GREEN)✅ Macros crate ready$(RESET)"
	@echo ""
	@echo "$(BOLD)Step 3: Publish telemetry-kit-macros$(RESET)"
	@echo "$(YELLOW)⚠️  About to publish telemetry-kit-macros v$(VERSION) to crates.io$(RESET)"
	@read -p "Publish macros crate? [y/N] " -n 1 -r; \
	echo; \
	if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
		echo "$(YELLOW)❌ Publish cancelled$(RESET)"; \
		exit 1; \
	fi
	@cargo publish -p telemetry-kit-macros
	@echo "$(GREEN)✅ telemetry-kit-macros v$(VERSION) published$(RESET)"
	@echo ""
	@echo "$(BOLD)Step 4: Wait for crates.io to index (30 seconds)...$(RESET)"
	@echo "$(YELLOW)⏳ Waiting for crates.io to index the macros crate...$(RESET)"
	@sleep 30
	@echo "$(GREEN)✅ Wait complete$(RESET)"
	@echo ""
	@echo "$(BOLD)Step 5: Publish telemetry-kit$(RESET)"
	@echo "$(YELLOW)⚠️  About to publish telemetry-kit v$(VERSION) to crates.io$(RESET)"
	@read -p "Publish main crate? [y/N] " -n 1 -r; \
	echo; \
	if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
		echo "$(YELLOW)❌ Publish cancelled$(RESET)"; \
		exit 1; \
	fi
	@cargo publish
	@echo ""
	@echo "$(GREEN)$(BOLD)🎉 All crates published successfully!$(RESET)"
	@echo ""
	@echo "  📦 telemetry-kit-macros v$(VERSION)"
	@echo "  📦 telemetry-kit v$(VERSION)"
	@echo ""
	@echo "$(CYAN)View on crates.io:$(RESET)"
	@echo "  https://crates.io/crates/telemetry-kit-macros"
	@echo "  https://crates.io/crates/telemetry-kit"
	@echo ""

.PHONY: publish-status
publish-status: ## 📊 Show publish status and version info
	@echo "$(BOLD)$(CYAN)📊 Publish Status$(RESET)"
	@echo ""
	@echo "$(BOLD)Local versions:$(RESET)"
	@echo "  telemetry-kit:        v$(VERSION)"
	@echo "  telemetry-kit-macros: v$(MACROS_VERSION)"
	@echo ""
	@if [ "$(VERSION)" = "$(MACROS_VERSION)" ]; then \
		echo "$(GREEN)✅ Versions are in sync$(RESET)"; \
	else \
		echo "$(RED)❌ Version mismatch - sync before publishing$(RESET)"; \
	fi
	@echo ""
	@echo "$(BOLD)Crates.io status:$(RESET)"
	@echo "  Check: https://crates.io/crates/telemetry-kit"
	@echo "  Check: https://crates.io/crates/telemetry-kit-macros"
	@echo ""

.PHONY: version-sync
version-sync: ## 🔄 Sync macros version to match main crate version
	@echo "$(CYAN)🔄 Syncing versions...$(RESET)"
	@echo "  Main crate version: $(VERSION)"
	@echo "  Updating telemetry-kit-macros to v$(VERSION)..."
	@sed -i '' 's/^version = ".*"/version = "$(VERSION)"/' telemetry-kit-macros/Cargo.toml
	@echo "$(GREEN)✅ Macros version updated to $(VERSION)$(RESET)"
	@echo ""
	@echo "$(YELLOW)📝 Also update the dependency version in main Cargo.toml:$(RESET)"
	@grep "telemetry-kit-macros" Cargo.toml
	@echo ""

##@ Benchmarking

.PHONY: bench
bench: ## 📈 Run benchmarks
	@echo "$(CYAN)📈 Running benchmarks...$(RESET)"
	@cargo bench
	@echo "$(GREEN)✅ Benchmarks complete$(RESET)"

##@ CI/CD Simulation

.PHONY: ci
ci: fmt-check lint test doc-check ## 🤖 Simulate CI pipeline locally
	@echo ""
	@echo "$(GREEN)✅ CI simulation passed$(RESET)"

.PHONY: ci-full
ci-full: fmt-check lint test doc-check audit deny-check ## 🚀 Full CI pipeline with security checks
	@echo ""
	@echo "$(GREEN)✅ Full CI simulation passed$(RESET)"

##@ Utilities

.PHONY: install-tools
install-tools: ## 🛠️  Install development tools
	@echo "$(CYAN)🛠️  Installing development tools...$(RESET)"
	@cargo install cargo-audit cargo-deny cargo-tarpaulin cargo-watch cargo-outdated
	@echo "$(GREEN)✅ Tools installed$(RESET)"

.PHONY: deps
deps: ## 📦 Show dependency tree
	@echo "$(CYAN)📦 Dependency tree:$(RESET)"
	@cargo tree

.PHONY: bloat
bloat: ## 📊 Analyze binary size (requires cargo-bloat)
	@echo "$(CYAN)📊 Analyzing binary size...$(RESET)"
	@cargo bloat --release

.PHONY: lines
lines: ## 📏 Count lines of code
	@echo "$(CYAN)📏 Lines of code:$(RESET)"
	@echo ""
	@echo "$(BOLD)Source code:$(RESET)"
	@find src -name '*.rs' | xargs wc -l | tail -1
	@echo ""
	@echo "$(BOLD)Server code:$(RESET)"
	@find server/src -name '*.rs' 2>/dev/null | xargs wc -l 2>/dev/null | tail -1 || echo "  N/A"
	@echo ""
	@echo "$(BOLD)Tests:$(RESET)"
	@find tests -name '*.rs' 2>/dev/null | xargs wc -l 2>/dev/null | tail -1 || echo "  N/A"
	@echo ""
	@echo "$(BOLD)Total:$(RESET)"
	@find . -name '*.rs' -not -path "./target/*" -not -path "./server/target/*" | xargs wc -l | tail -1

.PHONY: todos
todos: ## 📝 Find TODO comments in code
	@echo "$(CYAN)📝 TODO comments:$(RESET)"
	@grep -rn "TODO\|FIXME\|XXX\|HACK" src/ --color=always || echo "  None found ✅"

##@ Workspace

.PHONY: workspace-build
workspace-build: ## 🏗️  Build entire workspace
	@echo "$(CYAN)🏗️  Building workspace...$(RESET)"
	@cargo build --workspace
	@echo "$(GREEN)✅ Workspace built$(RESET)"

.PHONY: workspace-test
workspace-test: ## 🧪 Test entire workspace
	@echo "$(CYAN)🧪 Testing workspace...$(RESET)"
	@cargo test --workspace
	@echo "$(GREEN)✅ Workspace tests passed$(RESET)"

.PHONY: workspace-clean
workspace-clean: ## 🧹 Clean entire workspace
	@echo "$(CYAN)🧹 Cleaning workspace...$(RESET)"
	@cargo clean
	@cd server && cargo clean
	@echo "$(GREEN)✅ Workspace cleaned$(RESET)"

##@ Quick Commands

.PHONY: quick-test
quick-test: fmt lint test ## ⚡ Quick test (format + lint + test)
	@echo ""
	@echo "$(GREEN)✅ Quick test passed$(RESET)"

.PHONY: quick-fix
quick-fix: fmt lint-fix ## 🔧 Quick fix (format + auto-fix lints)
	@echo ""
	@echo "$(GREEN)✅ Code fixed$(RESET)"

.PHONY: pre-commit
pre-commit: fmt-check lint test ## ✅ Pre-commit checks
	@echo ""
	@echo "$(GREEN)✅ Pre-commit checks passed$(RESET)"

.PHONY: pre-push
pre-push: ci ## 🚀 Pre-push checks (full CI simulation)
	@echo ""
	@echo "$(GREEN)✅ Pre-push checks passed$(RESET)"

##@ Special

.PHONY: ascii-art
ascii-art: ## 🎨 Show project ASCII art
	@echo ""
	@echo "$(CYAN)"
	@echo "  ╔════════════════════════════════════════════╗"
	@echo "  ║                                            ║"
	@echo "  ║         🔭  telemetry-kit  🔭              ║"
	@echo "  ║                                            ║"
	@echo "  ║   Privacy-first telemetry for Rust apps   ║"
	@echo "  ║                                            ║"
	@echo "  ║              Version $(VERSION)              ║"
	@echo "  ║                                            ║"
	@echo "  ╚════════════════════════════════════════════╝"
	@echo "$(RESET)"
	@echo ""

.PHONY: coffee
coffee: ## ☕ Take a break reminder
	@echo ""
	@echo "$(YELLOW)☕ Time for a coffee break!$(RESET)"
	@echo "$(CYAN)You've been working hard. Take 5 minutes.$(RESET)"
	@echo ""
	@sleep 1
	@echo "$(GREEN)✅ Back to coding in 5... 4... 3... 2... 1...$(RESET)"
	@echo ""

.PHONY: stats
stats: ## 📊 Show project statistics
	@echo "$(BOLD)$(CYAN)📊 Project Statistics$(RESET)"
	@echo ""
	@echo "$(BOLD)Version:$(RESET)      $(VERSION)"
	@echo "$(BOLD)Rust Files:$(RESET)   $(shell find src -name '*.rs' | wc -l | xargs)"
	@echo "$(BOLD)Lines of Code:$(RESET) $(shell find src -name '*.rs' | xargs wc -l | tail -1 | awk '{print $$1}')"
	@echo "$(BOLD)Tests:$(RESET)        $(shell grep -r "^#\[test\]" src tests 2>/dev/null | wc -l | xargs)"
	@echo "$(BOLD)Examples:$(RESET)     $(shell ls examples/*.rs 2>/dev/null | wc -l | xargs)"
	@echo "$(BOLD)Dependencies:$(RESET) $(shell cargo tree --depth 1 | grep -v "telemetry" | wc -l | xargs)"
	@echo "$(BOLD)Git Commits:$(RESET)  $(shell git rev-list --count HEAD 2>/dev/null || echo 'N/A')"
	@echo ""

# Phony targets declaration
.PHONY: all
all: build test ## 🎯 Build and test (default: use 'make help')
	@echo "$(GREEN)✅ Build and test complete$(RESET)"
