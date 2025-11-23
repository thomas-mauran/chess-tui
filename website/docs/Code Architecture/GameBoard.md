---
id: GameBoard
title: GameBoard
sidebar_position: 5
---

## GameBoard

The `GameBoard` struct manages the chess board state using the **shakmaty** chess library. It maintains position history, move history, and handles all chess rule validation.

## Responsibilities

The `GameBoard` manages:
- **Position history** - Complete history of board positions
- **Move history** - All moves made in the game
- **Draw detection** - 50-move rule, threefold repetition, stalemate, insufficient material
- **Move validation** - Using shakmaty's legal move system
- **Coordinate conversion** - Handling flipped board orientation

## Key Fields

```rust
pub struct GameBoard {
    pub move_history: Vec<Move>,              // History of all moves
    pub position_history: Vec<Chess>,         // History of board positions (last = current)
    pub consecutive_non_pawn_or_capture: i32, // Counter for 50-move rule
    pub taken_pieces: Vec<Piece>,             // Captured pieces (with color)
    pub is_flipped: bool,                     // Board orientation for display
}
```

## Key Methods

### Position Access

- **`position_ref() -> &Chess`** - Gets a read-only reference to the last position in the history (panics if empty)
- **`current_position() -> Option<&Chess>`** - Safe access to current position, returns None if history is empty

### Move Execution

- **`execute_move(from: Square, to: Square, promotion: Option<Role>) -> Option<Move>`**
  - Validates move using shakmaty's legal move system
  - Tracks captured pieces
  - Updates position and move history
  - Returns executed Move if successful, None if illegal

- **`execute_shakmaty_move(from: Square, to: Square) -> bool`**
  - Convenience method for moves without promotion
  - Returns true if move was executed

- **`execute_standard_move(from: Square, to: Square, promotion: Option<Role>) -> Option<Move>`**
  - Executes moves from standard (non-flipped) coordinates
  - Used for bot and opponent moves which come in standard notation

- **`execute_shakmaty_move_with_promotion(from: Square, to: Square, promotion: Option<Role>) -> bool`**
  - Executes move with explicit promotion piece

### Move Information

- **`move_to_san(index: usize) -> String`** - Converts move to Standard Algebraic Notation
  - Uses shakmaty's SAN conversion
  - Examples: "e4", "Nf3", "O-O", "Qxd5+"

- **`is_latest_move_promotion() -> bool`** - Checks if last move was a promotion

### Piece Information

- **`get_role_at_square(square: &Square) -> Option<Role>`** - Gets piece type at square
- **`get_piece_color_at_square(square: &Square) -> Option<Color>`** - Gets piece color at square
- **`is_square_occupied(square: &Square) -> bool`** - Checks if square has a piece

### Legal Moves

- **`get_authorized_positions(player_turn: Color, square: &Square) -> Vec<Square>`**
  - Returns all legal destination squares for a piece
  - Uses shakmaty's `legal_moves()` method
  - Filters moves by source square

### Game End Detection

- **`is_checkmate() -> bool`** - Checks if current position is checkmate
- **`is_draw() -> bool`** - Checks for draw conditions:
  - Stalemate
  - 50-move rule (`consecutive_non_pawn_or_capture == 50`)
  - Threefold repetition (`is_draw_by_repetition()`)
  - Insufficient material
- **`is_draw_by_repetition() -> bool`** - Checks if position repeated 3+ times
- **`is_getting_checked(player_turn: Color) -> bool`** - Checks if player is in check

### Taken Pieces

- **`white_taken_pieces() -> Vec<Role>`** - Returns captured white pieces
- **`black_taken_pieces() -> Vec<Role>`** - Returns captured black pieces

### Board Orientation

- **`flip_the_board()`** - Toggles board orientation (for perspective switching)
- **`is_flipped: bool`** - Tracks current orientation

### FEN Position

- **`fen_position() -> String`**
  - Generates FEN string for UCI engine communication
  - Uses shakmaty's FEN encoding
  - Automatically uses current board position

### Utility

- **`reset()`** - Resets board to starting position
- **`increment_consecutive_non_pawn_or_capture(role_from: Role, role_to: Option<Role>)`**
  - Updates 50-move rule counter
  - Resets on pawn moves or captures

## Shakmaty Integration

The `GameBoard` uses shakmaty's `Chess` type for position management:

- **`Vec<Chess>`** - Position history, each element is a complete board state
- **`Move`** - Move representation with promotion, castling, en passant support
- **Legal move validation** - All moves are validated using `chess.legal_moves()`
- **SAN conversion** - Uses `San::from_move()` for algebraic notation

This ensures all chess rules are correctly enforced by the shakmaty library.

