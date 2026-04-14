#!/bin/bash
# Debian package build script wrapper for kitty-launcher
# This script provides a unified interface for building debian packages
# Usage: ./scripts/build-deb.sh [OPTIONS]

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
CLEAN_BUILD=false
SIGNED=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN_BUILD=true
            shift
            ;;
        --signed)
            SIGNED=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --clean      Clean build artifacts before building"
            echo "  --signed     Sign the debian package (requires GPG key)"
            echo "  -h, --help   Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                    # Build unsigned package"
            echo "  $0 --clean            # Clean and build"
            echo "  $0 --signed --clean   # Clean and build with signature"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${YELLOW}Building Debian package for kitty-launcher...${NC}"
echo "Project directory: $PROJECT_DIR"

# Change to project directory
cd "$PROJECT_DIR"

# Check for required tools
echo -e "${GREEN}Checking dependencies...${NC}"
for cmd in dpkg-buildpackage cargo rustc; do
    if ! command -v $cmd &> /dev/null; then
        echo -e "${RED}Error: $cmd not found. Please install build dependencies.${NC}"
        echo "On Debian/Ubuntu: sudo apt-get install build-essential debhelper dpkg-dev cargo rustc"
        exit 1
    fi
done

# Check for debhelper (specific file instead of command)
if [ ! -d "/usr/share/debhelper" ]; then
    echo -e "${RED}Error: debhelper not found. Please install build dependencies.${NC}"
    echo "On Debian/Ubuntu: sudo apt-get install debhelper"
    exit 1
fi

# Clean build if requested
if [ "$CLEAN_BUILD" = true ]; then
    echo -e "${GREEN}Cleaning build artifacts...${NC}"
    cargo clean
    rm -f kitty-launcher_*.deb
    rm -f kitty-launcher_*.build
    rm -f kitty-launcher_*.changes
fi

# Build source first to verify everything compiles
echo -e "${GREEN}Building Rust binary...${NC}"
cargo build --release
strip target/release/kitty-launcher

# Build Debian package
echo -e "${GREEN}Building Debian package...${NC}"
if [ "$SIGNED" = true ]; then
    echo -e "${YELLOW}Building signed package (requires GPG key)...${NC}"
    dpkg-buildpackage -uc
else
    echo -e "${YELLOW}Building unsigned package...${NC}"
    dpkg-buildpackage -us -uc -d
fi

# Find and verify the built packages
echo -e "${GREEN}Locating build artifacts...${NC}"
DEB_FILE=$(ls -1 ../kitty-launcher_*.deb 2>/dev/null | tail -1)

if [ -z "$DEB_FILE" ]; then
    echo -e "${RED}Error: Debian package not found after build.${NC}"
    exit 1
fi

echo -e "${GREEN}Debian package created: $(basename $DEB_FILE)${NC}"
ls -lh "$DEB_FILE"

# Copy to project directory
cp "$DEB_FILE" .
echo -e "${GREEN}Package copied to project directory.${NC}"

echo ""
echo -e "${GREEN}Debian package build completed successfully!${NC}"
echo -e "${YELLOW}Package file:${NC} $(basename $DEB_FILE)"
echo -e "${YELLOW}Install with:${NC} sudo dpkg -i $(basename $DEB_FILE)"
exit 0
