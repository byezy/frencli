#!/bin/bash
# Helper script to run frencli binary tests
# This script is executed from Rust tests to work around Command::new() issues
# Usage: run_binary_test.sh <binary_path> <subcommand> [args...]

set -e

# Get the binary path (first arg) and convert to absolute path
BINARY="$1"
shift

# Convert to absolute path before changing directories
if [ ! -f "$BINARY" ]; then
    echo "ERROR: Binary not found: $BINARY" >&2
    exit 1
fi

# Get absolute path of binary
BINARY_ABS="$(cd "$(dirname "$BINARY")" && pwd)/$(basename "$BINARY")"

# Verify binary exists and is executable
if [ ! -f "$BINARY_ABS" ]; then
    echo "ERROR: Binary not found: $BINARY_ABS" >&2
    exit 1
fi

if [ ! -x "$BINARY_ABS" ]; then
    chmod +x "$BINARY_ABS"
fi

# Get test data directory (relative to script location)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DATA_DIR="$SCRIPT_DIR/../test_data"

# Change to test data directory
if [ ! -d "$TEST_DATA_DIR" ]; then
    echo "ERROR: Test data directory not found: $TEST_DATA_DIR" >&2
    exit 1
fi

cd "$TEST_DATA_DIR" || {
    echo "ERROR: Cannot change to test data directory: $TEST_DATA_DIR" >&2
    exit 1
}

# Execute the binary with all remaining args (use absolute path)
# Redirect stdin from /dev/null to prevent hanging on interactive prompts
exec "$BINARY_ABS" "$@" < /dev/null
