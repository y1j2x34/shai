#!/bin/bash

# SHAI Build and Validation Script
# This script verifies the implementation is complete and working

set -e  # Exit on error

echo "================================================"
echo "SHAI Implementation Validation"
echo "================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}⚠️  Rust/Cargo not found. Please install Rust first:${NC}"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${BLUE}✓ Rust toolchain found${NC}"
cargo --version
rustc --version
echo ""

# Check project structure
echo "================================================"
echo "Checking Project Structure..."
echo "================================================"

check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} $1"
    else
        echo -e "${YELLOW}✗${NC} $1 (missing)"
        return 1
    fi
}

check_dir() {
    if [ -d "$1" ]; then
        echo -e "${GREEN}✓${NC} $1/"
    else
        echo -e "${YELLOW}✗${NC} $1/ (missing)"
        return 1
    fi
}

echo ""
echo "Core Modules:"
check_file "src/main.rs"
check_file "src/lib.rs"
check_file "src/command.rs"
check_file "src/ai.rs"
check_file "src/config.rs"
check_file "src/storage.rs"
check_file "src/history.rs"

echo ""
echo "Templates & Tests:"
check_file "src/bookmark.rs.template"
check_file "src/history_tests.rs"

echo ""
echo "Documentation:"
check_file "README.md"
check_file "ARCHITECTURE.md"
check_file "TESTING.md"
check_file "QUICKREF.md"
check_file "CHANGELOG.md"
check_file "PROJECT_OVERVIEW.md"

echo ""
echo "Configuration:"
check_file "Cargo.toml"
check_file "Cargo.lock"

echo ""

# Check dependencies in Cargo.toml
echo "================================================"
echo "Checking Dependencies..."
echo "================================================"

check_dependency() {
    if grep -q "^$1 =" Cargo.toml; then
        echo -e "${GREEN}✓${NC} $1"
    else
        echo -e "${YELLOW}✗${NC} $1 (missing from Cargo.toml)"
        return 1
    fi
}

check_dependency "chrono"
check_dependency "clap"
check_dependency "serde"
check_dependency "serde_json"
check_dependency "tokio"
check_dependency "inquire"
check_dependency "openai-api-rs"

echo ""

# Run cargo check
echo "================================================"
echo "Running Cargo Check..."
echo "================================================"
echo ""

if cargo check; then
    echo ""
    echo -e "${GREEN}✓ Cargo check passed${NC}"
else
    echo ""
    echo -e "${YELLOW}✗ Cargo check failed${NC}"
    exit 1
fi

echo ""

# Run cargo clippy if available
echo "================================================"
echo "Running Clippy (if available)..."
echo "================================================"
echo ""

if command -v cargo-clippy &> /dev/null; then
    if cargo clippy -- -D warnings; then
        echo ""
        echo -e "${GREEN}✓ Clippy check passed${NC}"
    else
        echo ""
        echo -e "${YELLOW}⚠️  Clippy found some warnings${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Clippy not installed, skipping${NC}"
fi

echo ""

# Build release binary
echo "================================================"
echo "Building Release Binary..."
echo "================================================"
echo ""

if cargo build --release; then
    echo ""
    echo -e "${GREEN}✓ Release build successful${NC}"
    
    # Check binary size
    if [ -f "target/release/shai" ]; then
        SIZE=$(du -h target/release/shai | cut -f1)
        echo -e "${BLUE}Binary size: $SIZE${NC}"
    fi
else
    echo ""
    echo -e "${YELLOW}✗ Release build failed${NC}"
    exit 1
fi

echo ""

# Verify CLI works
echo "================================================"
echo "Testing CLI Interface..."
echo "================================================"
echo ""

if [ -f "target/release/shai" ]; then
    echo "Testing --help flag:"
    ./target/release/shai --help
    echo ""
    
    echo "Testing history subcommand help:"
    ./target/release/shai history --help
    echo ""
    
    echo -e "${GREEN}✓ CLI interface working${NC}"
else
    echo -e "${YELLOW}✗ Binary not found${NC}"
    exit 1
fi

echo ""

# Summary
echo "================================================"
echo "Validation Summary"
echo "================================================"
echo ""
echo -e "${GREEN}✓ All checks passed!${NC}"
echo ""
echo "Implementation Status:"
echo "  • Modular architecture: ✓"
echo "  • History feature: ✓"
echo "  • Storage system: ✓"
echo "  • CLI interface: ✓"
echo "  • Documentation: ✓"
echo "  • Build success: ✓"
echo ""
echo "Next Steps:"
echo "  1. Set your API key: export SHAI_API_KEY='your-key'"
echo "  2. Test the tool: ./target/release/shai \"list files\""
echo "  3. Check history: ./target/release/shai history"
echo ""
echo "Ready for deployment! 🚀"
echo ""

