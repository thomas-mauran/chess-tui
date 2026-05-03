---
id: bot
title: Setup
sidebar_position: 5
description: "Step-by-step guide to setting up a UCI chess bot in Chess TUI."
---

# Bot Setup

Before you can play against a bot, you need to configure a UCI-compatible chess engine.

## Quick Setup (Recommended)

The easiest way is the automatic Stockfish installer:

```bash
./scripts/install-stockfish.sh
```

This script detects your OS, installs Stockfish via the appropriate package manager, and writes the engine path to your config automatically. No further configuration needed.

## Manual Configuration

### Configuration File

```toml
# CONFIG_DIR/chess-tui/config.toml
engine_path = "/path/to/your/engine"
```

### Command Line

```bash
chess-tui -e /path/to/your/engine
chess-tui --engine-path /path/to/your/engine
```

The path is automatically saved to your config for future sessions.

## Common Engine Paths

**Stockfish:**
- macOS (Homebrew): `/opt/homebrew/bin/stockfish`
- Linux: `/usr/bin/stockfish`

**GNU Chess** (requires `--uci` flag):
- macOS (Homebrew): `/opt/homebrew/bin/gnuchess --uci`
- Linux: `/usr/bin/gnuchess --uci`

:::tip
Some engines require extra flags to enable UCI mode. Pass them together with the path:
```bash
chess-tui -e "/usr/bin/gnuchess --uci"
```
:::

## Verifying Your Setup

1. Launch `chess-tui`
2. Navigate to **Play Bot**
3. If the engine path is valid you can select your color and start playing otherwise an error message will explain what's wrong
