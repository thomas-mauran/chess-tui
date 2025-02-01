---
id: engine
title: Chess Engine
sidebar_position: 4
---

# Chess Engine Configuration

To play against a computer opponent, you need to configure a UCI-compatible chess engine.

## Configuration

Set the path to your chess engine in the configuration file:

```toml
engine_path = "/path/to/your/engine"
```

## Supported Engines

Any UCI-compatible chess engine should work. Some popular options include:
- Stockfish
- Leela Chess Zero
- Komodo

:::note
The engine path must point to a valid UCI-compatible chess engine executable. If not configured correctly, the bot play option will be disabled.
::: 