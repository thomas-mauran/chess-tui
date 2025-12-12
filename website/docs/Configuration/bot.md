---
id: bot
title: Bot Configuration
sidebar_position: 5
---

# Bot Configuration

Chess-tui allows you to configure the behavior of the computer opponent (bot) when playing against a chess engine.

## Configuration

Bot settings can be configured in the `CONFIG_DIR/chess-tui/config.toml` file:

```toml
# Bot thinking depth for chess engine (1-255, default: 10)
bot_depth = 10
```

CONFIG_DIR is typically:
- Linux: $XDG_CONFIG_HOME or $HOME/.config
- macOS: $HOME/Library/Application Support
- Windows: {FOLDERID_RoamingAppData}

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

## Related Configuration

The bot also requires a chess engine to be configured. See [Chess Engine Configuration](./engine.md) for details on setting up your UCI-compatible chess engine.
