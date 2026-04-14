#!/bin/bash
# Build script wrapper for kitty-launcher
# This script provides a unified build interface for both local and CI/CD builds
# Usage: ./scripts/build.sh [--release] [--test]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Defaults
RELEASE_BUILD=false
RUN_TESTS=false
STRIP_BINARY=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            RELEASE_BUILD=true
            shift
            ;;
        --test)
            RUN_TESTS=true
            shift
            ;;
        --strip)
            STRIP_BINARY=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --release    Build release binary (optimized)"
            echo "  --test       Run tests after build"
            echo "  --strip      Strip binary symbols (requires --release)"
            echo "  -h, --help   Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${YELLOW}Building kitty-launcher...${NC}"
echo "Project directory: $PROJECT_DIR"

# Change to project directory
cd "$PROJECT_DIR"

# Run cargo build
if [ "$RELEASE_BUILD" = true ]; then
    echo -e "${GREEN}Building release binary...${NC}"
    cargo build --release
    
    # Strip binary if requested
    if [ "$STRIP_BINARY" = true ]; then
        echo -e "${GREEN}Stripping binary...${NC}"
        strip target/release/kitty-launcher
    fi
    
    BINARY_PATH="target/release/kitty-launcher"
else
    echo -e "${GREEN}Building debug binary...${NC}"
    cargo build
    BINARY_PATH="target/debug/kitty-launcher"
fi

# Get binary size
BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
echo -e "${GREEN}Binary created: $BINARY_PATH ($BINARY_SIZE)${NC}"

# Run tests if requested
if [ "$RUN_TESTS" = true ]; then
    echo -e "${GREEN}Running tests...${NC}"
    if [ "$RELEASE_BUILD" = true ]; then
        cargo test --release
    else
        cargo test
    fi
fi

echo -e "${GREEN}Build completed successfully!${NC}"
exit 0
