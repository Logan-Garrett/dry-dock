#!/bin/bash

# Dry Dock Build and Install Script
# Automates building and installing the macOS app bundle

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/dry-dock" && pwd)"
APP_NAME="Dry Dock"
BUNDLE_PATH="$PROJECT_DIR/target/release/bundle/osx/$APP_NAME.app"
INSTALL_PATH="/Applications/$APP_NAME.app"

# Default options
CLEAN_BUILD=false
BUILD_ONLY=false
SKIP_QUARANTINE=false

# Print colored message
print_message() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Print usage
print_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Build and install Dry Dock application"
    echo ""
    echo "OPTIONS:"
    echo "  --clean              Remove previous build artifacts before building"
    echo "  --build-only         Build the bundle but don't install"
    echo "  --skip-quarantine    Skip removing quarantine attributes"
    echo "  --help              Show this help message"
    echo ""
    echo "EXAMPLES:"
    echo "  $0                          # Build and install"
    echo "  $0 --clean                  # Clean build and install"
    echo "  $0 --build-only             # Build without installing"
    echo "  $0 --clean --build-only     # Clean build without installing"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN_BUILD=true
            shift
            ;;
        --build-only)
            BUILD_ONLY=true
            shift
            ;;
        --skip-quarantine)
            SKIP_QUARANTINE=true
            shift
            ;;
        --help)
            print_usage
            exit 0
            ;;
        *)
            print_message "$RED" "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

# Header
print_message "$BLUE" "╔════════════════════════════════════════╗"
print_message "$BLUE" "║   Dry Dock Build & Install Script     ║"
print_message "$BLUE" "╚════════════════════════════════════════╝"
echo ""

# Check if cargo-bundle is installed
if ! command -v cargo-bundle &> /dev/null; then
    print_message "$RED" "✗ cargo-bundle is not installed"
    print_message "$YELLOW" "Installing cargo-bundle..."
    cargo install cargo-bundle
fi

# Navigate to project directory
print_message "$BLUE" "→ Project directory: $PROJECT_DIR"
cd "$PROJECT_DIR"

# Clean build if requested
if [ "$CLEAN_BUILD" = true ]; then
    print_message "$YELLOW" "→ Cleaning previous build artifacts..."
    cargo clean
fi

# Build the bundle
print_message "$GREEN" "→ Building release bundle..."
cargo bundle --release

# Check if build succeeded
if [ ! -d "$BUNDLE_PATH" ]; then
    print_message "$RED" "✗ Build failed - bundle not found at: $BUNDLE_PATH"
    exit 1
fi

print_message "$GREEN" "✓ Build successful!"
print_message "$BLUE" "  Bundle location: $BUNDLE_PATH"

# Exit if build-only mode
if [ "$BUILD_ONLY" = true ]; then
    print_message "$YELLOW" "→ Build-only mode - skipping installation"
    print_message "$GREEN" "✓ Done!"
    exit 0
fi

# Remove existing installation
if [ -d "$INSTALL_PATH" ]; then
    print_message "$YELLOW" "→ Removing existing installation..."
    rm -rf "$INSTALL_PATH"
fi

# Install the app
print_message "$GREEN" "→ Installing to Applications folder..."
cp -r "$BUNDLE_PATH" "$INSTALL_PATH"

# Remove quarantine attributes
if [ "$SKIP_QUARANTINE" = false ]; then
    print_message "$GREEN" "→ Removing quarantine attributes..."
    xattr -cr "$INSTALL_PATH" 2>/dev/null || true
fi

# Verify installation
if [ -d "$INSTALL_PATH" ]; then
    print_message "$GREEN" "✓ Installation successful!"
    print_message "$BLUE" "  Installed at: $INSTALL_PATH"
    echo ""
    print_message "$GREEN" "You can now launch '$APP_NAME' from:"
    print_message "$BLUE" "  • Spotlight (Cmd + Space)"
    print_message "$BLUE" "  • Applications folder"
    print_message "$BLUE" "  • Launchpad"
    echo ""
    
    # Ask if user wants to launch the app
    read -p "$(echo -e ${GREEN}Launch $APP_NAME now? [y/N]: ${NC})" -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_message "$GREEN" "→ Launching $APP_NAME..."
        open "$INSTALL_PATH"
    fi
else
    print_message "$RED" "✗ Installation failed!"
    exit 1
fi

print_message "$GREEN" "✓ Done!"
