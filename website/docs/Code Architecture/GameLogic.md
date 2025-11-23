---
id: GameLogic
title: GameLogic
sidebar_position: 4
---

## GameLogic

The `GameLogic` struct encapsulates all game logic, separating it from UI concerns. This separation was introduced in a recent refactor to improve code maintainability and reduce borrow checker issues.

## Responsibilities

The `GameLogic` struct manages:
- **Board state management** - via `GameBoard`
- **Move execution and validation** - using shakmaty library
- **Bot and opponent handling** - coordinating external players
- **Game state tracking** - checkmate, draw, promotion states

## Key Fields

```rust
pub struct GameLogic {
    pub game_board: GameBoard,           // Chess board state
    pub bot: Option<Bot>,                // Bot player (if playing against bot)
    pub opponent: Option<Opponent>,      // Opponent player (if multiplayer)
    pub player_turn: Color,              // Current player's turn
    pub game_state: GameState,           // Current game state
}
```

## Key Methods

### Move Execution

- **`execute_move(from: Square, to: Square)`** - Executes a player move
  - Validates the move using shakmaty
  - Updates move history and position history
  - Tracks captured pieces
  - Increments draw counter for 50-move rule

- **`execute_bot_move()`** - Executes a bot move
  - Gets FEN position from game board
  - Retrieves move from bot via UCI protocol
  - Converts UCI move to shakmaty Move (with error handling)
  - Applies move to board (with error handling)
  - Handles errors gracefully without panicking

- **`execute_opponent_move() -> bool`** - Executes opponent move from network
  - Reads move string from TCP stream
  - Uses `parse_opponent_move_string()` helper to parse chess notation (e.g., "e2e4")
  - Handles promotion piece selection
  - Returns true if move was successfully executed, false on error

- **`parse_opponent_move_string(move_str: &str) -> Option<(Square, Square, Option<Role>)>`** (private helper)
  - Parses move string in chess notation (e.g., "e2e4" or "e7e8q")
  - Returns (from_square, to_square, promotion_piece) or None if invalid

### Promotion

- **`promote_piece(promotion_cursor: u8)`** - Handles pawn promotion
  - Removes last move from history
  - Re-executes move with selected promotion piece
  - Updates game state back to Playing

- **`handle_multiplayer_promotion()`** - Sends promotion choice to opponent

### Game State

- **`switch_player_turn()`** - Alternates between White and Black
- **`update_game_state()`** - Updates game state based on board conditions
  - Checks for checkmate
  - Checks for draw conditions
  - Checks for promotion requirement

### Helper Methods (Internal)

- **`update_game_state_after_move()`** - Updates game state after a move (checkmate, draw, promotion)
- **`handle_after_move_bot_logic()`** - Handles bot-specific logic after a move
- **`handle_after_move_opponent_logic()`** - Handles opponent-specific logic after a move
- **`handle_after_move_board_flip()`** - Handles board flipping logic after a move (only in single-player mode)

## Integration with Game

The `GameLogic` is owned by the `Game` struct, which also contains the `UI` struct:

```rust
pub struct Game {
    pub logic: GameLogic,
    pub ui: UI,
}
```

This separation allows:
- UI to remain responsive while game logic executes
- Clear separation of concerns
- Easier testing of game logic independently
- Reduced borrow checker conflicts

## Game State Flow

1. **Playing** - Normal game state, players making moves
2. **Promotion** - Pawn reached promotion square, waiting for piece selection
3. **Checkmate** - Game ended, one player is in checkmate
4. **Draw** - Game ended in a draw (stalemate, 50-move rule, repetition, insufficient material)

The game state is automatically updated after each move via `update_game_state()`.

