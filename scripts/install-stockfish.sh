#!/bin/bash

# Install Stockfish chess engine
# This script detects the operating system and installs Stockfish accordingly
# and automatically configures it in chess-tui's config file

set -e

echo "Installing Stockfish chess engine..."

# Function to get config directory based on OS
get_config_dir() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        echo "$HOME/Library/Application Support"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux - check XDG_CONFIG_HOME first, fallback to ~/.config
        if [[ -n "$XDG_CONFIG_HOME" ]]; then
            echo "$XDG_CONFIG_HOME"
        else
            echo "$HOME/.config"
        fi
    else
        # Fallback (Windows would be %APPDATA% but bash script won't work there anyway)
        echo "$HOME/.config"
    fi
}

# Function to update config.toml with engine_path
update_config_file() {
    local config_file="$1"
    local engine_path="$2"
    
    # Create config directory if it doesn't exist
    local config_dir=$(dirname "$config_file")
    mkdir -p "$config_dir"
    
    if [ -f "$config_file" ] && [ -s "$config_file" ]; then
        # Config file exists and is not empty
        # Check if engine_path already exists in file
        if grep -q "^engine_path" "$config_file"; then
            # Update existing engine_path line
            # Use | as delimiter to avoid issues with forward slashes in paths
            if [[ "$OSTYPE" == "darwin"* ]]; then
                # macOS uses BSD sed
                sed -i '' "s|^engine_path = .*|engine_path = \"$engine_path\"|" "$config_file"
            else
                # Linux uses GNU sed
                sed -i "s|^engine_path = .*|engine_path = \"$engine_path\"|" "$config_file"
            fi
        else
            # Append engine_path to existing file
            # Add a blank line before appending for better formatting
            echo "" >> "$config_file"
            echo "engine_path = \"$engine_path\"" >> "$config_file"
        fi
    else
        # Create new config file with default values
        cat > "$config_file" << EOF
engine_path = "$engine_path"
display_mode = "DEFAULT"
log_level = "OFF"
bot_depth = 10
selected_skin_name = "Default"
sound_enabled = true
EOF
    fi
}

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    if command -v brew &> /dev/null; then
        echo "Detected macOS with Homebrew"
        echo "Installing Stockfish via Homebrew..."
        brew install stockfish
        STOCKFISH_PATH=$(brew --prefix)/bin/stockfish
        
        # Update config file
        CONFIG_DIR=$(get_config_dir)
        CONFIG_FILE="$CONFIG_DIR/chess-tui/config.toml"
        echo ""
        echo "Configuring chess-tui..."
        update_config_file "$CONFIG_FILE" "$STOCKFISH_PATH"
        
        echo ""
        echo "✓ Stockfish installed and configured successfully!"
        echo ""
        echo "The engine path has been set in: $CONFIG_FILE"
        echo "  engine_path = \"$STOCKFISH_PATH\""
    else
        echo "Error: Homebrew is required for macOS installation."
        echo "Install Homebrew from: https://brew.sh"
        exit 1
    fi
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if command -v apt-get &> /dev/null; then
        # Debian/Ubuntu
        echo "Detected Debian/Ubuntu Linux"
        echo "Installing Stockfish via apt..."
        sudo apt-get update
        sudo apt-get install -y stockfish
        STOCKFISH_PATH="/usr/bin/stockfish"
    elif command -v dnf &> /dev/null; then
        # Fedora
        echo "Detected Fedora Linux"
        echo "Installing Stockfish via dnf..."
        sudo dnf install -y stockfish
        STOCKFISH_PATH="/usr/bin/stockfish"
    elif command -v pacman &> /dev/null; then
        # Arch Linux
        echo "Detected Arch Linux"
        echo "Installing Stockfish via pacman..."
        sudo pacman -S --noconfirm stockfish
        STOCKFISH_PATH="/usr/bin/stockfish"
    else
        echo "Error: Unsupported Linux distribution."
        echo "Please install Stockfish manually from: https://stockfishchess.org/download/"
        exit 1
    fi
    
    # Update config file
    CONFIG_DIR=$(get_config_dir)
    CONFIG_FILE="$CONFIG_DIR/chess-tui/config.toml"
    echo ""
    echo "Configuring chess-tui..."
    update_config_file "$CONFIG_FILE" "$STOCKFISH_PATH"
    
    echo ""
    echo "✓ Stockfish installed and configured successfully!"
    echo ""
    echo "The engine path has been set in: $CONFIG_FILE"
    echo "  engine_path = \"$STOCKFISH_PATH\""
else
    echo "Error: Unsupported operating system: $OSTYPE"
    echo "Please install Stockfish manually from: https://stockfishchess.org/download/"
    exit 1
fi

