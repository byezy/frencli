#!/bin/bash
# Script to run all tests individually with timeout to identify hanging tests

# Don't exit on error - we want to continue running all tests
set +e

TIMEOUT_SECONDS=30
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$TEST_DIR/.." && pwd)"
cd "$PROJECT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
TOTAL=0
PASSED=0
FAILED=0
TIMED_OUT=0
SKIPPED=0

# Test files to check
TEST_FILES=(
    "audit_tests"
    "executor_tests"
    "help_tests"
    "integration_tests"
    "list_tests"
    "rename_tests"
    "subcommands_tests"
    "template_tests"
    "templates_tests"
    "make_tests"
    "ui_tests"
    "undo_tests"
    "validate_tests"
)

echo "=========================================="
echo "Running tests individually with ${TIMEOUT_SECONDS}s timeout"
echo "=========================================="
echo ""

# Function to run a single test
run_test() {
    local test_file=$1
    local test_name=$2
    TOTAL=$((TOTAL + 1))
    
    echo -n "[$TOTAL] Running $test_file::$test_name ... "
    
    # Run test with timeout and exact match (use --exact after -- to avoid partial matches)
    if timeout $TIMEOUT_SECONDS cargo test --test "$test_file" "$test_name" -- --exact --nocapture 2>&1 > /tmp/test_output_$$.txt; then
        echo -e "${GREEN}PASSED${NC}"
        PASSED=$((PASSED + 1))
        return 0
    else
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            # Timeout
            echo -e "${RED}TIMED OUT${NC} (exceeded ${TIMEOUT_SECONDS}s)"
            TIMED_OUT=$((TIMED_OUT + 1))
            echo "  Output:"
            head -20 /tmp/test_output_$$.txt | sed 's/^/    /'
            return 1
        else
            # Failed
            echo -e "${RED}FAILED${NC}"
            FAILED=$((FAILED + 1))
            echo "  Output:"
            head -30 /tmp/test_output_$$.txt | sed 's/^/    /'
            return 1
        fi
    fi
}

# Get list of tests from a test file by parsing source
get_tests_from_file() {
    local test_file=$1
    local test_file_path="tests/${test_file}.rs"
    
    if [ ! -f "$test_file_path" ]; then
        return
    fi
    
    # Extract test function names using grep
    # Match both #[test] and #[tokio::test] patterns, then get the function name
    grep -E "^#\[(tokio::)?test\]" "$test_file_path" -A 1 | \
        grep -E "^\s*(pub\s+)?(async\s+)?fn\s+\w+" | \
        sed -E 's/^\s*(pub\s+)?(async\s+)?fn\s+(\w+).*/\3/' | \
        grep -v '^$' || true
}

# Main execution
for test_file in "${TEST_FILES[@]}"; do
    echo ""
    echo "=== Processing $test_file ==="
    
    # Check if test file exists
    if [ ! -f "tests/${test_file}.rs" ]; then
        echo "  Skipping: tests/${test_file}.rs not found"
        SKIPPED=$((SKIPPED + 1))
        continue
    fi
    
    # Get list of tests
    tests=$(get_tests_from_file "$test_file")
    
    if [ -z "$tests" ]; then
        echo "  No tests found in $test_file"
        SKIPPED=$((SKIPPED + 1))
        continue
    fi
    
    # Run each test individually
    while IFS= read -r test_name; do
        if [ -n "$test_name" ]; then
            run_test "$test_file" "$test_name"
        fi
    done <<< "$tests"
done

# Cleanup
rm -f /tmp/test_output_$$.txt

# Summary
echo ""
echo "=========================================="
echo "Summary"
echo "=========================================="
echo "Total tests:  $TOTAL"
echo -e "${GREEN}Passed:      $PASSED${NC}"
echo -e "${RED}Failed:      $FAILED${NC}"
echo -e "${YELLOW}Timed out:   $TIMED_OUT${NC}"
echo "Skipped:     $SKIPPED"
echo ""

if [ $TIMED_OUT -gt 0 ] || [ $FAILED -gt 0 ]; then
    exit 1
else
    exit 0
fi

