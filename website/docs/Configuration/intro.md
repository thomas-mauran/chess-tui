---
id: configuration-intro
title: Configuration
sidebar_position: 1
---

# Configuration

Chess-tui can be configured through the configuration file located at `CONFIG_DIR/chess-tui/config.toml`. Additionally, custom color skins can be configured in `CONFIG_DIR/chess-tui/skins.json`. This section covers all available configuration options.

CONFIG_DIR is typically:
- Linux: $XDG_CONFIG_HOME or $HOME/.config
- macOS: $HOME/Library/Application Support
- Windows: {FOLDERID_RoamingAppData}

## Command Line Options

Some configuration options can also be set directly from the command line:

```bash
# Set chess engine path
chess-tui -e /path/to/engine

# Set bot thinking depth
chess-tui --depth 15

# Combine both options
chess-tui -e /path/to/engine --depth 15
```

Command line options take precedence over configuration file values.

## Configuration File

The configuration file is automatically created when you first run chess-tui. You can modify it manually to customize your experience:

```toml
# CONFIG_DIR/chess-tui/config.toml

# Display mode: "DEFAULT" or "ASCII"
display_mode = "DEFAULT"

# Chess engine path (optional)
engine_path = "/path/to/your/engine"

# Logging level: "OFF", "ERROR", "WARN", "INFO", "DEBUG", or "TRACE"
log_level = "OFF"

# Bot thinking depth for chess engine (1-255, default: 10)
bot_depth = 10
``` 

CONFIG_DIR is typically:
- Linux: $XDG_CONFIG_HOME or $HOME/.config
- macOS: $HOME/Library/Application Support
- Windows: {FOLDERID_RoamingAppData}