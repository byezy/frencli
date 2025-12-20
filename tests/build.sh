#!/usr/bin/env bash
# Test setup script - ensures the binary is built before running tests
# This script can be run manually or called from test setup
# Usage: ./tests/build.sh [--force]

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Get crate root (one level up from tests/)
CRATE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Binary paths to check
DEBUG_BINARY="$CRATE_ROOT/target/debug/fren"
TEST_BINARY="$CRATE_ROOT/target/test/fren"

# Check if binary exists
if [ -f "$DEBUG_BINARY" ] || [ -f "$TEST_BINARY" ]; then
    if [ "$1" != "--force" ]; then
        echo "Binary already exists. Use --force to rebuild."
        exit 0
    fi
fi

echo "Building binary for tests..."
cd "$CRATE_ROOT"

# Build the binary
cargo build

# Verify binary was created
if [ ! -f "$DEBUG_BINARY" ]; then
    echo "ERROR: Binary not found after build: $DEBUG_BINARY" >&2
    exit 1
fi

echo "Binary built successfully: $DEBUG_BINARY"
exit 0

