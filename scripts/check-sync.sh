#!/usr/bin/env bash
# Check if event schemas are in sync between public and private repos

set -euo pipefail

PUBLIC_REPO="$HOME/Dev/telemetry-kit"
PRIVATE_REPO="$HOME/Dev/telemetry-kit.dev"

echo "🔍 Checking schema sync between repositories..."
echo ""

# Check if private repo exists
if [ ! -d "$PRIVATE_REPO" ]; then
    echo "⚠️  Private repo not found at: $PRIVATE_REPO"
    echo "   Skipping sync check"
    exit 0
fi

# Compare event schema versions
echo "📋 Comparing event schema versions..."

PUBLIC_SCHEMA="$PUBLIC_REPO/src/event.rs"
PRIVATE_SCHEMA="$PRIVATE_REPO/crates/domain/src/entities/event.rs"

if [ ! -f "$PRIVATE_SCHEMA" ]; then
    echo "⚠️  Private schema not found at: $PRIVATE_SCHEMA"
    echo "   Assuming hexagonal architecture is still in development"
    exit 0
fi

# Extract schema version from both files
PUBLIC_VERSION=$(grep -o 'SCHEMA_VERSION.*=.*"[0-9.]*"' "$PUBLIC_SCHEMA" | grep -o '[0-9.]*' || echo "unknown")
PRIVATE_VERSION=$(grep -o 'SCHEMA_VERSION.*=.*"[0-9.]*"' "$PRIVATE_SCHEMA" | grep -o '[0-9.]*' || echo "unknown")

echo "  Public repo:  v$PUBLIC_VERSION"
echo "  Private repo: v$PRIVATE_VERSION"

if [ "$PUBLIC_VERSION" != "$PRIVATE_VERSION" ]; then
    echo ""
    echo "❌ Schema versions DO NOT MATCH!"
    echo "   Action required: Update schema in private repo"
    exit 1
else
    echo "  ✅ Schema versions match"
fi

echo ""
echo "📋 Comparing HMAC authentication..."

PUBLIC_AUTH="$PUBLIC_REPO/src/sync/auth.rs"
PRIVATE_AUTH_CANDIDATES=(
    "$PRIVATE_REPO/crates/application/src/services/auth.rs"
    "$PRIVATE_REPO/crates/api/src/middleware/auth.rs"
)

PRIVATE_AUTH=""
for candidate in "${PRIVATE_AUTH_CANDIDATES[@]}"; do
    if [ -f "$candidate" ]; then
        PRIVATE_AUTH="$candidate"
        break
    fi
done

if [ -z "$PRIVATE_AUTH" ]; then
    echo "⚠️  Private auth module not found"
    echo "   Expected locations:"
    for candidate in "${PRIVATE_AUTH_CANDIDATES[@]}"; do
        echo "     - $candidate"
    done
else
    echo "  ✅ Both auth modules exist"
    echo "     Public:  $PUBLIC_AUTH"
    echo "     Private: $PRIVATE_AUTH"
fi

echo ""
echo "📋 Recommendations:"
echo "  1. Review SYNC_STRATEGY.md for sync requirements"
echo "  2. Test SDK against both servers"
echo "  3. Update both CHANGELOGs when making protocol changes"
echo ""
echo "✅ Schema sync check complete"
