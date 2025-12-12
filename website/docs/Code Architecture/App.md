---
id: App
title: App
sidebar_position: 3
---

## App

The `App` struct is the main application container that orchestrates the entire chess-tui application.

## Responsibilities

The `App` struct manages:
- **Application lifecycle and state** - running flag, pages, popups
- **Game configuration** - chess engine path, bot depth, display mode
- **Bot move computation** - background threads for async bot thinking
- **Multiplayer server setup** - opponent connections and game server
- **Menu navigation** - user input handling and page transitions
- **Configuration file management** - `CONFIG_DIR/chess-tui/config.toml`

CONFIG_DIR is typically:
- Linux: $XDG_CONFIG_HOME or $HOME/.config
- macOS: $HOME/Library/Application Support
- Windows: `%APPDATA%` (Roaming AppData folder)

## Key Fields

```rust
pub struct App {
    pub running: bool,                          // Application running state
    pub game: Game,                             // The chess game instance
    pub current_page: Pages,                   // Current page (Home, Solo, Multiplayer, Bot, Credit)
    pub current_popup: Option<Popups>,         // Active popup (Help, ColorSelection, etc.)
    pub selected_color: Option<Color>,          // Selected color when playing against bot
    pub hosting: Option<bool>,                  // Whether hosting a multiplayer game
    pub host_ip: Option<String>,                // Host IP address for multiplayer
    pub menu_cursor: u8,                       // Menu navigation cursor
    pub chess_engine_path: Option<String>,     // Path to UCI chess engine
    pub log_level: LevelFilter,                 // Logging level
    pub bot_depth: u8,                          // Bot thinking depth (1-255)
    pub bot_move_receiver: Option<Receiver<Move>>, // Channel receiver for bot moves
    pub error_message: Option<String>,          // Error message for Error popup
}
```

## Key Methods

### Bot Management

- **`start_bot_thinking()`** - Spawns a background thread to compute bot moves asynchronously
- **`check_bot_move()`** - Checks if bot move is ready and applies it to the game
- **`is_bot_thinking()`** - Returns whether bot is currently computing a move
- **`apply_bot_move(Move)`** - Applies a computed bot move to the game board

### Game Setup

- **`bot_setup()`** - Initializes bot with engine path and depth
- **`create_opponent()`** - Sets up multiplayer opponent connection
- **`setup_game_server(host_color: Color)`** - Starts game server for hosting
- **`hosting_selection()`** - Handles host/client selection in multiplayer

### Navigation

- **`go_to_home()`** - Returns to home page and restarts game
- **`menu_select()`** - Handles menu item selection
- **`menu_cursor_up/down/left/right(l: u8)`** - Navigates menu cursor

### Configuration

- **`update_config()`** - Writes current settings to config file
- **`get_host_ip()`** - Retrieves local IP address for multiplayer hosting

### Game Control

- **`restart()`** - Resets game while preserving bot/opponent setup
- **`reset()`** - Complete reset of all game state
- **`quit()`** - Exits the application

## Bot Async Architecture

Bot moves are computed asynchronously to keep the UI responsive:

1. **`start_bot_thinking()`** spawns a thread that:
   - Creates a new `Bot` instance in the thread
   - Gets the current FEN position
   - Computes the best move using the UCI engine
   - Sends the move through a channel (`bot_move_receiver`)

2. **`check_bot_move()`** is called each frame to:
   - Check if a move is ready in the channel
   - Apply the move if available
   - Clear the receiver

3. **`is_bot_thinking()`** prevents starting multiple bot threads simultaneously

This architecture ensures the UI remains responsive even when the chess engine takes time to compute moves.

