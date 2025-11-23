#!/usr/bin/env bash
# Run all test suites for telemetry-kit

set -euo pipefail

echo "🧪 Running all telemetry-kit tests..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
    fi
}

# Track overall status
FAILURES=0

# 1. Format check
echo "📝 Checking code formatting..."
if cargo fmt -- --check; then
    print_status 0 "Formatting check passed"
else
    print_status 1 "Formatting check failed"
    ((FAILURES++))
fi
echo ""

# 2. Clippy lints
echo "🔍 Running clippy lints..."
if cargo clippy --all-features --all-targets -- -D warnings; then
    print_status 0 "Clippy passed"
else
    print_status 1 "Clippy failed"
    ((FAILURES++))
fi
echo ""

# 3. Unit tests
echo "🧪 Running unit tests..."
if cargo test --lib; then
    print_status 0 "Unit tests passed"
else
    print_status 1 "Unit tests failed"
    ((FAILURES++))
fi
echo ""

# 4. Integration tests
echo "🔗 Running integration tests..."
if cargo test --test '*'; then
    print_status 0 "Integration tests passed"
else
    print_status 1 "Integration tests failed"
    ((FAILURES++))
fi
echo ""

# 5. Doc tests
echo "📚 Running documentation tests..."
if cargo test --doc; then
    print_status 0 "Doc tests passed"
else
    print_status 1 "Doc tests failed"
    ((FAILURES++))
fi
echo ""

# 6. Examples
echo "📦 Building examples..."
if cargo build --examples; then
    print_status 0 "Examples built successfully"
else
    print_status 1 "Examples build failed"
    ((FAILURES++))
fi
echo ""

# 7. Documentation build
echo "📖 Building documentation..."
if cargo doc --no-deps --document-private-items; then
    print_status 0 "Documentation built successfully"
else
    print_status 1 "Documentation build failed"
    ((FAILURES++))
fi
echo ""

# 8. Security audit (optional, may have warnings)
echo "🔒 Running security audit..."
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        print_status 0 "Security audit passed"
    else
        echo -e "${YELLOW}⚠️  Security audit found issues (not counted as failure)${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  cargo-audit not installed (run: cargo install cargo-audit)${NC}"
fi
echo ""

# Summary
echo "================================================"
if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ $FAILURES test suite(s) failed${NC}"
    exit 1
fi
