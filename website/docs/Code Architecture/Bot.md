---
id: Bot
title: Bot
sidebar_position: 7
---

## Bot

The `Bot` struct interfaces with UCI-compatible chess engines using the **ruci** library. It manages the engine process and converts between FEN positions and UCI moves.

## Responsibilities

The `Bot` manages:
- **Engine process** - Spawning and managing the chess engine subprocess
- **UCI protocol communication** - Sending positions and receiving moves
- **Move computation** - Configurable thinking depth
- **FEN conversion** - Converting board positions to FEN strings

## Key Fields

```rust
pub struct Bot {
    process: Rc<RefCell<Child>>,                    // Engine subprocess
    engine: Rc<RefCell<Engine<BufReader<ChildStdout>, ChildStdin>>>, // UCI engine
    pub bot_will_move: bool,                        // Flag to trigger bot move
    pub is_bot_starting: bool,                      // Whether bot plays first
    pub depth: u8,                                  // Thinking depth (1-255)
}
```

## Key Methods

### Initialization

- **`new(engine_path: &str, is_bot_starting: bool, depth: u8) -> Bot`**
  - Spawns chess engine subprocess
  - Initializes UCI engine communication
  - Sets thinking depth
  - **Note**: Currently reuses the same process (marked as TODO/FIXME)

### Move Computation

- **`get_move(fen: &str) -> UciMove`**
  - Sends FEN position to engine
  - Requests best move with configured depth
  - Returns UCI move (e.g., "e2e4")
  - Blocks until engine responds

## UCI Protocol

The bot communicates with engines using the Universal Chess Interface (UCI) protocol:

1. **Position command**: Sends current board state in FEN format
2. **Go command**: Requests engine to calculate best move
3. **Best move response**: Engine returns move in UCI notation

The **ruci** library handles the protocol details, allowing the bot to focus on:
- Converting game positions to FEN
- Converting UCI moves to shakmaty Moves
- Managing engine lifecycle

## Async Architecture

Bot moves are computed asynchronously to keep the UI responsive:

1. **`App::start_bot_thinking()`** spawns a background thread
2. Thread creates a new `Bot` instance
3. Bot computes move in the background
4. Move is sent through a channel (`bot_move_receiver`)
5. **`App::check_bot_move()`** applies the move when ready

This prevents the UI from freezing during engine computation, which can take several seconds for deep analysis.

## Engine Requirements

The bot requires a UCI-compatible chess engine, such as:
- **Stockfish** - Popular open-source engine
- **Leela Chess Zero** - Neural network engine
- Any engine supporting UCI protocol

The engine path is configured via:
- Command line argument: `--engine-path`
- Configuration file: `~/.config/chess-tui/config.toml`

## Configuration

- **Depth**: Controls how many moves ahead the engine analyzes (default: 10)
  - Higher depth = stronger play but slower
  - Lower depth = faster but weaker play
- **Engine path**: Path to the chess engine executable

## Limitations

⚠️ **Current Implementation Note**: The bot reuses the same engine process across moves. The code includes a TODO/FIXME noting that chess engines are not meant to be used this way. A future refactor may create new engine instances per move or properly manage engine state.

