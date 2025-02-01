---
id: configuration-intro
title: Configuration
sidebar_position: 1
---

# Configuration

Chess-tui can be configured through the configuration file located at `~/.config/chess-tui/config.toml`. This section covers all available configuration options.

## Configuration File

The configuration file is automatically created when you first run chess-tui. You can modify it manually to customize your experience:

```toml
# ~/.config/chess-tui/config.toml

# Display mode: "DEFAULT" or "ASCII"
display_mode = "DEFAULT"

# Chess engine path (optional)
engine_path = "/path/to/your/engine"

# Logging level: "OFF", "ERROR", "WARN", "INFO", "DEBUG", or "TRACE"
log_level = "OFF"
``` 