---
id: Opponent
title: Opponent
sidebar_position: 8
---

## Opponent

The `Opponent` struct handles online multiplayer via TCP streams. It manages network communication, move synchronization, and color assignment between players.

## Responsibilities

The `Opponent` manages:
- **TCP stream communication** - Network connection to other player
- **Move encoding/decoding** - Converting moves to/from network format
- **Color assignment** - Determining which color each player plays
- **Game start coordination** - Synchronizing game initialization

## Key Fields

```rust
pub struct Opponent {
    pub stream: Option<TcpStream>,           // TCP connection to opponent
    pub opponent_will_move: bool,             // Flag indicating opponent's turn
    pub color: Color,                         // This player's color
    pub game_started: bool,                   // Whether game has started
}
```

## Key Methods

### Initialization

- **`new(addr: String, color: Option<Color>) -> Opponent`**
  - Attempts connection to server (up to 5 retries)
  - Gets color from server if not provided
  - Sets `opponent_will_move` based on color (White moves first)
  - Panics if connection fails after all attempts

- **`start_stream(addr: &str)`** - Establishes TCP connection to address

### Move Communication

- **`send_move_to_server(move: &Move, promotion: Option<Role>)`**
  - Encodes move in standard chess notation (e.g., "e2e4")
  - Appends promotion piece if applicable (q, r, b, n)
  - Sends move string over TCP stream

- **`read_stream() -> String`**
  - Reads move string from TCP stream (non-blocking)
  - Returns empty string if no data available
  - Handles "ended" message (game termination)
  - Returns move in format "e2e4" or "e7e8q" (with promotion)

### Game Control

- **`send_end_game_to_server()`** - Sends "ended" message to signal game end

### Utility

- **`copy() -> Opponent`** - Creates copy without stream (for cloning)
- **`clone()`** - Custom clone implementation that attempts to clone TCP stream

## Move Encoding

Moves are encoded as 4-5 character strings:
- **Format**: `{from}{to}[promotion]`
- **Examples**:
  - `"e2e4"` - Pawn from e2 to e4
  - `"e7e8q"` - Pawn promotion to queen
  - `"g1f3"` - Knight from g1 to f3

Promotion pieces:
- `q` - Queen
- `r` - Rook
- `b` - Bishop
- `n` - Knight

## Color Assignment

When connecting to a server:
1. **Host** chooses their color first
2. **Client** receives opposite color from server
3. Server sends:
   - `"w"` for White
   - `"b"` for Black

The `opponent_will_move` flag is set based on color:
- **White** → `opponent_will_move = true` (moves first)
- **Black** → `opponent_will_move = false` (waits for opponent)

## Game Start Coordination

The server sends `"s"` (start) message when:
- Both players are connected
- Colors are assigned
- Game is ready to begin

The `wait_for_game_start()` function blocks until this message is received.

## Network Protocol

### Connection Flow

1. **Host** starts game server via `App::setup_game_server()`
2. **Client** connects via `Opponent::new()`
3. Server assigns colors and sends start signal
4. Players exchange moves via TCP streams

### Move Exchange

- Moves are sent immediately after execution
- Non-blocking reads prevent UI freezing
- Empty reads indicate no move available yet
- "ended" message terminates the game

## Error Handling

- **Connection failures**: Retries up to 5 times before panicking
- **Network errors**: Returns empty string, allowing retry
- **Game end**: Panics on "ended" message (expected behavior)
- **Stream cloning**: Handles cases where stream cannot be cloned

## Integration with Game

The opponent is managed by `GameLogic`:

```rust
pub struct GameLogic {
    pub opponent: Option<Opponent>,
    // ...
}
```

When `opponent_will_move` is true:
1. `App` main loop checks for opponent moves
2. Calls `GameLogic::execute_opponent_move()`
3. Reads move from stream
4. Applies move to board
5. Switches player turn

This keeps the UI responsive while waiting for network moves.

