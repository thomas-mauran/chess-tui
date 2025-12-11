#!/bin/bash

# Script to generate all VHS demo videos
# This script will process all .tape files in the examples directory

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Ensure the vhs command is available
if ! command -v vhs >/dev/null 2>&1; then
    echo -e "${RED}Error: vhs command not found.${NC}"
    echo "Please install VHS: https://github.com/charmbracelet/vhs"
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo -e "${GREEN}Generating all VHS demo videos...${NC}"
echo ""

# Find all .tape files in the current directory
tape_files=($(find . -maxdepth 1 -name "*.tape" -type f | sort))

if [ ${#tape_files[@]} -eq 0 ]; then
    echo -e "${YELLOW}No .tape files found in the examples directory.${NC}"
    exit 0
fi

# Process each tape file
for tape_file in "${tape_files[@]}"; do
    filename=$(basename "$tape_file" .tape)
    echo -e "${YELLOW}Processing: ${filename}.tape${NC}"
    
    if vhs "$tape_file"; then
        echo -e "${GREEN}✓ Generated: ${filename}.gif${NC}"
    else
        echo -e "${RED}✗ Failed to generate: ${filename}.gif${NC}"
        exit 1
    fi
    echo ""
done

echo -e "${GREEN}All videos generated successfully!${NC}"
