---
id: bot-settings
title: Settings
sidebar_position: 7
description: "Configure bot difficulty, thinking time, and engine parameters in Chess TUI."
---

# Bot Settings

## Configuration File

```toml
# CONFIG_DIR/chess-tui/config.toml

# Engine search depth (1-255, default: 10). Used when difficulty is Off.
bot_depth = 10

# Difficulty preset: 0=Easy, 1=Medium, 2=Hard, 3=Magnus. Omit for full strength.
bot_difficulty = 0
```

`CONFIG_DIR` is:
- Linux: `$XDG_CONFIG_HOME` or `$HOME/.config`
- macOS: `$HOME/Library/Application Support`
- Windows: `%APPDATA%`

## Command Line Overrides

Override depth or difficulty for a single session without editing the config:

```bash
chess-tui --depth 15
chess-tui -d 15

chess-tui --difficulty easy
chess-tui --difficulty medium
chess-tui --difficulty hard
chess-tui --difficulty magnus
```

## Difficulty Presets

Use **←** and **→** in the game setup form to cycle through presets:

| Preset | Approx. ELO | Best for |
|--------|-------------|----------|
| **Off** | Full strength | Experienced players |
| **Easy** | 400 | Beginners |
| **Medium** | 900 | Casual play |
| **Hard** | 1500 | Intermediate |
| **Magnus** | 2700 | Strong players |

:::tip
Stockfish supports `UCI_LimitStrength` / `UCI_Elo` natively, giving smooth and predictable scaling across all presets. For engines that don't support ELO limiting (e.g. GNU Chess), chess-tui weakens the bot by reducing search depth and thinking time instead.
:::

## Bot Depth

Controls how many moves ahead the engine looks. Only applies when difficulty is **Off**.

| Range | Speed | Strength |
|-------|-------|----------|
| 1–5 | Near-instant | Weak |
| 6–10 | 1–5 seconds | Balanced (recommended) |
| 11–15 | 5–30 seconds | Strong |
| 16+ | Minutes or more | Very strong |

:::warning
Depth 20+ can cause the engine to think for several minutes in complex positions.
:::

## Troubleshooting

### Engine path not found

1. Confirm the path points to an actual executable
2. Check permissions: `chmod +x /path/to/engine`
3. Test it manually: `echo "uci" | /path/to/engine` you should see `uciok`
4. For Stockfish, re-run `./scripts/install-stockfish.sh`

### Engine not responding during play

1. Confirm UCI support: `echo "uci" | /path/to/engine`
2. Add required flags if needed (e.g. GNU Chess needs `--uci`)
3. Check logs in `CONFIG_DIR/chess-tui/logs/` after enabling `log_level = "DEBUG"`
