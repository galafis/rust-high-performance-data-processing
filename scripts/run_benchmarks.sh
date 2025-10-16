#!/bin/bash
# Benchmark Runner Script
# This script runs all benchmarks and generates reports

set -e

echo "=================================="
echo "Running Performance Benchmarks"
echo "=================================="

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_info() {
    echo -e "${YELLOW}[i]${NC} $1"
}

# Check if criterion is available
print_info "Checking benchmark dependencies..."
if ! cargo bench --no-run 2>&1 | grep -q "Finished"; then
    echo "Building benchmark dependencies..."
fi

# Run benchmarks
print_info "Running benchmarks (this may take a few minutes)..."
cargo bench

print_status "Benchmarks completed!"
print_info "Results saved to: target/criterion/"

# Check if criterion generated HTML reports
if [ -d "target/criterion" ]; then
    print_info "HTML reports are available at:"
    find target/criterion -name "index.html" | head -5
fi

echo "=================================="
echo "To view detailed results, open the HTML files in target/criterion/"
echo "=================================="
