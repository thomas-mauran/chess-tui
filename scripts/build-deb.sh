#!/bin/bash
# Build script for creating Debian/Ubuntu packages
# Usage: ./scripts/build-deb.sh [method]
# Methods: dh-cargo (default) or cargo-deb

set -e

METHOD="${1:-dh-cargo}"

echo "Building chess-tui Debian package using method: $METHOD"

if [ "$METHOD" = "cargo-deb" ]; then
    echo "Using cargo-deb method..."
    
    # Check if cargo-deb is installed
    if ! command -v cargo-deb &> /dev/null; then
        echo "Error: cargo-deb is not installed"
        echo "Install it with: cargo install cargo-deb"
        exit 1
    fi
    
    # Build the package
    cargo deb
    
    echo "Package built successfully!"
    echo "Find the .deb file in: target/debian/"
    
elif [ "$METHOD" = "dh-cargo" ]; then
    echo "Using dh-cargo method..."
    
    # Check if required tools are installed
    if ! command -v dpkg-buildpackage &> /dev/null; then
        echo "Error: dpkg-buildpackage is not installed"
        echo "Install it with: sudo apt-get install devscripts debhelper dh-cargo"
        exit 1
    fi
    
    # Check if we're in a clean git state (recommended)
    if [ -d ".git" ]; then
        if ! git diff-index --quiet HEAD --; then
            echo "Warning: You have uncommitted changes"
            read -p "Continue anyway? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                exit 1
            fi
        fi
    fi
    
    # Build the package
    dpkg-buildpackage -us -uc
    
    echo "Package built successfully!"
    echo "Find the .deb file in the parent directory"
    
else
    echo "Error: Unknown method '$METHOD'"
    echo "Valid methods: dh-cargo, cargo-deb"
    exit 1
fi
