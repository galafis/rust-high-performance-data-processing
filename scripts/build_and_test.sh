#!/bin/bash
# Build and Test Script
# This script builds the project, runs all tests, and generates a report

set -e  # Exit on error

echo "=================================="
echo "Rust High-Performance Data Processing"
echo "Build and Test Script"
echo "=================================="

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

print_info() {
    echo -e "${YELLOW}[i]${NC} $1"
}

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo is not installed. Please install Rust toolchain."
    exit 1
fi

print_status "Cargo found: $(cargo --version)"

# Clean previous builds
print_info "Cleaning previous builds..."
cargo clean
print_status "Clean completed"

# Check formatting
print_info "Checking code formatting..."
if cargo fmt -- --check; then
    print_status "Code formatting is correct"
else
    print_error "Code formatting issues found. Run 'cargo fmt' to fix."
    exit 1
fi

# Run clippy
print_info "Running clippy (linter)..."
if cargo clippy -- -D warnings; then
    print_status "Clippy passed with no warnings"
else
    print_error "Clippy found issues"
    exit 1
fi

# Build in debug mode
print_info "Building in debug mode..."
if cargo build; then
    print_status "Debug build successful"
else
    print_error "Debug build failed"
    exit 1
fi

# Run tests
print_info "Running unit tests..."
if cargo test --lib; then
    print_status "Unit tests passed"
else
    print_error "Unit tests failed"
    exit 1
fi

# Run integration tests
print_info "Running integration tests..."
if cargo test --test integration_tests; then
    print_status "Integration tests passed"
else
    print_error "Integration tests failed"
    exit 1
fi

# Run doc tests
print_info "Running documentation tests..."
if cargo test --doc; then
    print_status "Documentation tests passed"
else
    print_error "Documentation tests failed"
    exit 1
fi

# Build in release mode
print_info "Building in release mode..."
if cargo build --release; then
    print_status "Release build successful"
else
    print_error "Release build failed"
    exit 1
fi

# Run the application
print_info "Running the application..."
./target/release/rust-high-performance-data-processing

print_status "All checks passed!"
echo "=================================="
echo "Build completed successfully!"
echo "=================================="
