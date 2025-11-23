#!/usr/bin/env bash
# Developer setup script for telemetry-kit

set -euo pipefail

echo "🚀 Setting up telemetry-kit development environment..."
echo ""

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "   Install from: https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed: $(rustc --version)"
echo ""

# Install development tools
echo "📦 Installing development tools..."
echo ""

# cargo-audit for security scanning
if ! command -v cargo-audit &> /dev/null; then
    echo "Installing cargo-audit..."
    cargo install cargo-audit
else
    echo "✅ cargo-audit already installed"
fi

# cargo-deny for dependency policy
if ! command -v cargo-deny &> /dev/null; then
    echo "Installing cargo-deny..."
    cargo install cargo-deny
else
    echo "✅ cargo-deny already installed"
fi

# cargo-tarpaulin for code coverage
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
else
    echo "✅ cargo-tarpaulin already installed"
fi

# cargo-watch for auto-rebuilding
if ! command -v cargo-watch &> /dev/null; then
    echo "Installing cargo-watch..."
    cargo install cargo-watch
else
    echo "✅ cargo-watch already installed"
fi

echo ""
echo "📚 Fetching dependencies..."
cargo fetch

echo ""
echo "🧪 Running initial tests..."
cargo test --lib

echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "📋 Next steps:"
echo "  1. Read PRODUCTION_PLAN.md for the roadmap"
echo "  2. Read SYNC_STRATEGY.md if working with both repos"
echo "  3. Run 'cargo test' to verify everything works"
echo "  4. Run 'scripts/test-all.sh' for comprehensive testing"
echo "  5. Run 'cargo run --example basic' to see SDK in action"
echo ""
echo "🛠️  Useful commands:"
echo "  cargo watch -x test          # Auto-run tests on file change"
echo "  cargo watch -x clippy        # Auto-lint on file change"
echo "  cargo doc --open             # Open documentation in browser"
echo "  ./scripts/test-all.sh        # Run all test suites"
echo "  ./scripts/check-sync.sh      # Check schema sync (if using private repo)"
echo ""
