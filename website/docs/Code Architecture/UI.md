---
id: UI
title: UI
sidebar_position: 6
---

## UI

The `UI` struct handles all rendering and user interaction for the chess game. It manages cursor position, piece selection, and renders the board, move history, and captured pieces.

## Responsibilities

The `UI` struct manages:
- **Board rendering** - Chess board with pieces, highlights, and colors
- **Move history display** - Algebraic notation move list
- **Captured pieces display** - Shows taken pieces for each side
- **Cursor and selection** - Keyboard and mouse interaction
- **Display modes** - DEFAULT (Unicode) and ASCII rendering

## Key Fields

```rust
pub struct UI {
    pub cursor_coordinates: Coord,           // Current cursor position
    pub selected_square: Option<Square>,      // Currently selected piece square
    pub selected_piece_cursor: i8,           // Cursor for authorized moves
    pub promotion_cursor: i8,                // Cursor for promotion selection
    pub old_cursor_position: Coord,           // Previous cursor (for unselect)
    pub top_x: u16,                          // Board top-left X coordinate
    pub top_y: u16,                          // Board top-left Y coordinate
    pub width: u16,                          // Cell width
    pub height: u16,                         // Cell height
    pub mouse_used: bool,                     // Whether last action was mouse
    pub display_mode: DisplayMode,           // DEFAULT or ASCII
    pub prompt: Prompt,                      // Text input prompt
}
```

## Key Methods

### Cursor Management

- **`cursor_up/down/left/right(authorized_positions: Vec<Coord>)`**
  - Moves cursor in specified direction
  - If piece is selected, moves through authorized positions only
  - Otherwise, moves freely on the board

- **`move_selected_piece_cursor(first_time_moving: bool, direction: i8, authorized_positions: Vec<Coord>)`**
  - Moves cursor through legal moves when piece is selected
  - Cycles through available destination squares

- **`cursor_left_promotion()` / `cursor_right_promotion()`**
  - Navigates promotion piece selection (Queen, Rook, Bishop, Knight)

### Selection

- **`is_cell_selected() -> bool`** - Checks if a piece is currently selected
- **`unselect_cell()`** - Deselects piece and restores cursor position

### Rendering Methods

#### Board Rendering

- **`board_render(area: Rect, frame: &mut Frame, logic: &GameLogic)`**
  - Main board rendering method
  - Handles board flipping for perspective
  - Renders pieces using Unicode or ASCII
  - Highlights:
    - **Blue** - Current cursor position
    - **Green** - Selected piece or last move squares
    - **Grey** - Available move destinations
    - **Magenta** - King in check (with blinking)

#### Label Rendering

- **`render_rank_labels(frame: &mut Frame, area: Rect, is_flipped: bool)`**
  - Renders rank labels (1-8) on the left side of the board
  - Order adjusts based on board flip state

- **`render_file_labels(frame: &mut Frame, area: Rect, is_flipped: bool)`**
  - Renders file labels (A-H) below the board
  - Order adjusts based on board flip state

#### History Rendering

- **`history_render(area: Rect, frame: &mut Frame, game: &Game)`**
  - Displays move history in Standard Algebraic Notation
  - Shows move number, piece symbol, and move (e.g., "1. ♙ e4 ♟ e5")
  - Formats moves in pairs (white, black)
  - Uses fixed-width formatting to ensure consistent column alignment:
    - Line numbers: Right-aligned in 3 characters + ". " (5 chars total)
    - White moves: Icon + space + move notation in 8 characters (left-aligned)
    - Black moves: Icon + space + move notation in 8 characters (left-aligned)
  - Maintains alignment regardless of move length or piece type

#### Material Rendering

- **`white_material_render(area: Rect, frame: &mut Frame, white_taken_pieces: &[Role])`**
  - Displays captured white pieces
  - Shows piece symbols for taken pieces
  - Displays "Press ? for help" at the bottom

- **`black_material_render(area: Rect, frame: &mut Frame, black_taken_pieces: &[Role])`**
  - Displays captured black pieces
  - Shows piece symbols for taken pieces

### Piece Rendering

- **`render_piece_paragraph(piece_type: Option<Role>, piece_color: Option<Color>, square: Rect) -> Paragraph`**
  - Renders individual piece based on display mode
  - **DEFAULT mode**: Multi-line Unicode art
  - **ASCII mode**: Single character (K, Q, R, B, N, P)

### Utility

- **`reset()`** - Resets all UI state to defaults

## Display Modes

### DEFAULT Mode
- Uses Unicode chess piece symbols (♔ ♕ ♖ ♗ ♘ ♙)
- Multi-line piece art for board rendering
- Color-coded pieces (white/black)

### ASCII Mode
- Single character representation (K, Q, R, B, N, P)
- Uppercase for white, lowercase for black
- Underlined for white pieces

## Coordinate System

The UI handles coordinate conversion between:
- **Visual coordinates** - What the user sees (may be flipped)
- **Standard coordinates** - Internal shakmaty coordinates (a1-h8)

The `is_flipped` flag in `GameBoard` determines if coordinates need conversion when:
- Selecting pieces
- Moving pieces
- Rendering the board

## Integration with Game

The `UI` is owned by the `Game` struct alongside `GameLogic`:

```rust
pub struct Game {
    pub logic: GameLogic,
    pub ui: UI,
}
```

The UI reads game state from `GameLogic` but doesn't modify it directly. All game state changes go through `Game` methods which coordinate between UI and logic.

