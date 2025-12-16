---
id: bot
title: Bot Configuration
sidebar_position: 5
---

# Bot Configuration

Chess-tui allows you to configure the behavior of the computer opponent (bot) when playing against a chess engine.

## Chess Engine Setup

Before you can play against a bot, you need to configure a UCI-compatible chess engine.

### Quick Setup (Recommended) ⭐

The easiest way to set up a chess engine is using the automatic installation script for Stockfish:

```bash
./scripts/install-stockfish.sh
```

This script will:
- Detect your operating system (macOS, Linux)
- Install Stockfish using the appropriate package manager
- Automatically configure the engine path in your chess-tui config file
- Work on macOS (Homebrew), Debian/Ubuntu (apt), Fedora (dnf), and Arch Linux (pacman)

After running the script, you can immediately start playing against the bot - no manual configuration needed!

### Manual Engine Configuration

If you prefer to install the engine manually or use a different engine, you can configure it yourself.

#### Configuration File

Set the path to your chess engine in the configuration file:

```toml
engine_path = "/path/to/your/engine"
```

#### Command Line Configuration

You can also set the engine path via command line:

```bash
# Simple path
chess-tui -e /path/to/your/engine

# Path with command-line arguments (e.g., GNU Chess)
chess-tui -e "/opt/homebrew/bin/gnuchess --uci"
```

Or with the long form:

```bash
chess-tui --engine-path /path/to/your/engine
chess-tui --engine-path "/opt/homebrew/bin/gnuchess --uci"
```

The path will be automatically saved to your configuration file for future use.

### Supported Engines

Any UCI-compatible chess engine should work. Some popular options include:
- **Stockfish** - Popular open-source engine (recommended)
- **GNU Chess** - Classic chess engine (requires `--uci` flag)
- **Leela Chess Zero** - Neural network engine
- **Komodo** - Commercial engine

:::note
The engine path must point to a valid UCI-compatible chess engine executable. If not configured correctly, the bot play option will be disabled.
:::

### Engines Requiring Command-Line Arguments

Some chess engines require command-line arguments to enable UCI mode. For example, GNU Chess requires the `--uci` flag. You can specify these arguments directly in the engine path:

**Command Line:**
```bash
chess-tui -e "/opt/homebrew/bin/gnuchess --uci"
```

**Configuration File:**
```toml
engine_path = "/opt/homebrew/bin/gnuchess --uci"
```

:::tip
When using quotes in the command line, use double quotes to ensure the entire path and arguments are treated as a single value. In the configuration file, quotes are not needed.
:::

### Common Engine Paths

**Stockfish:**
- **macOS (Homebrew)**: `/opt/homebrew/bin/stockfish` or `/usr/local/bin/stockfish`
- **Linux (apt)**: `/usr/bin/stockfish`
- **Linux (dnf/pacman)**: `/usr/bin/stockfish`

**GNU Chess (requires `--uci` flag):**
- **macOS (Homebrew)**: `/opt/homebrew/bin/gnuchess --uci` or `/usr/local/bin/gnuchess --uci`
- **Linux (apt)**: `/usr/bin/gnuchess --uci`
- **Linux (dnf/pacman)**: `/usr/bin/gnuchess --uci`

## Bot Configuration

Bot settings can be configured in the `CONFIG_DIR/chess-tui/config.toml` file:

```toml
# Bot thinking depth for chess engine (1-255, default: 10)
bot_depth = 10
```

CONFIG_DIR is typically:
- Linux: $XDG_CONFIG_HOME or $HOME/.config
- macOS: $HOME/Library/Application Support
- Windows: `%APPDATA%` (Roaming AppData folder)

### Command Line Override

You can also set the bot depth directly from the command line, which will override the configuration file value for that session:

```bash
chess-tui --depth 15
# or
chess-tui -d 15
```

This is useful for:
- Testing different depth values without editing the config file
- Setting a custom depth for a specific game session
- Overriding the default depth temporarily

## Bot Depth

The `bot_depth` parameter controls how deeply the chess engine analyzes positions:

- **Lower values (1-5)**: Faster moves, weaker play
- **Medium values (6-15)**: Balanced speed and strength (recommended for most users)
- **Higher values (16+)**: Stronger play, but slower moves

### Default Value

The default bot depth is **10**, which provides a good balance between playing strength and response time.

### Performance Impact

- **Depth 1-5**: Moves are calculated almost instantly
- **Depth 6-10**: Moves typically take 1-5 seconds
- **Depth 11-15**: Moves may take 5-30 seconds
- **Depth 16+**: Moves can take several minutes or more

:::tip
For casual play, a depth of 8-12 is usually sufficient. For serious analysis or when you have time to wait, you can increase the depth to 15-20.
:::

:::warning
Very high depth values (20+) can cause the engine to think for a very long time, especially in complex positions. Use these values only when you have plenty of time and want maximum playing strength.
:::

## Verifying Your Setup

To verify your engine is configured correctly:

1. Launch `chess-tui`
2. Navigate to the **"Play Bot"** menu option
3. If your engine path is valid, you'll be able to select your color and start playing
4. If there's an issue, you'll see an error message with instructions

## Troubleshooting

### Engine Path Not Found

If you see an error that the engine path is invalid:

1. **Check the path**: Make sure the path points to an actual executable file
2. **Verify permissions**: Ensure the file is executable (`chmod +x /path/to/engine`)
3. **Test manually**: Try running the engine from the command line to verify it works
4. **Use the install script**: If using Stockfish, try the automatic installation script

### Engine Not Responding

If the engine path is valid but the engine doesn't respond during gameplay:

1. **Check UCI compatibility**: Ensure your engine supports the UCI protocol
2. **Add required flags**: Some engines (like GNU Chess) require command-line arguments to enable UCI mode:
   ```bash
   chess-tui -e "/opt/homebrew/bin/gnuchess --uci"
   ```
3. **Test UCI mode**: Test the engine manually in UCI mode:
   ```bash
   echo "uci" | /path/to/engine --uci
   ```
   You should see `uciok` in the response
4. **Check logs**: Enable logging to see detailed error messages:
   ```bash
   chess-tui -e /path/to/engine
   # Then check logs in CONFIG_DIR/chess-tui/logs/
   ```
