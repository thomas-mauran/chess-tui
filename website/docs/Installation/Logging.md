# Logging

Chess-tui includes a configurable logging system that can help with debugging and understanding the application's behavior.

## Configuration

Logging can be configured in the `~/.config/chess-tui/config.toml` file. The log level can be set using the `log_level` option:

```toml
log_level = "INFO"  # Default is "OFF"
```

### Available Log Levels

- `OFF` - Logging disabled (default)
- `ERROR` - Only error messages
- `WARN` - Warning and error messages
- `INFO` - Informational messages, warnings, and errors
- `DEBUG` - Detailed debug information plus all above
- `TRACE` - Most detailed logging level

## Log Files

When logging is enabled, log files are stored in:
```
~/.config/chess-tui/logs/
```

Each log file is named with a timestamp:
```
chess-tui_YYYY-MM-DD_HH-MM-SS.log
```

For example: `chess-tui_2024-03-20_15-30-45.log`

## Usage

Logs can be helpful when:
- Debugging multiplayer connection issues
- Understanding game state changes
- Investigating unexpected behavior
- Developing new features

:::tip
For normal gameplay, you can leave logging set to `OFF`. Enable logging only when you need to troubleshoot issues or want to understand the application's behavior in detail.
::: 