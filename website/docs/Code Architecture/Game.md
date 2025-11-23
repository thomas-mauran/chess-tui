## Global architecture

```mermaid
classDiagram

class App {
        +bool running
        +Game game
        +Pages current_page
        +Option~Popups~ current_popup
        +Option~Color~ selected_color
        +Option~bool~ hosting
        +Option~String~ host_ip
        +u8 menu_cursor
        +Option~String~ chess_engine_path
        +LevelFilter log_level
        +u8 bot_depth
        +Option~Receiver~Move~~ bot_move_receiver

        +toggle_help_popup()
        +toggle_credit_popup()
        +go_to_home()
        +tick()
        +quit()
        +start_bot_thinking()
        +check_bot_move() bool
        +is_bot_thinking() bool
        +apply_bot_move(Move)
        +menu_cursor_up(l: u8)
        +menu_cursor_right(l: u8)
        +menu_cursor_left(l: u8)
        +menu_cursor_down(l: u8)
        +color_selection()
        +restart()
        +menu_select()
        +update_config()
        +setup_game_server(host_color: Color)
        +create_opponent()
        +hosting_selection()
        +bot_setup()
        +get_host_ip() Option~IpAddr~
    }
    
    class Game {
        +GameLogic logic
        +UI ui

        +new(logic: GameLogic)
        +select_cell()
        +handle_cell_click()
        +switch_player_turn()
        +already_selected_cell_action()
        +handle_promotion()
    }
    
    class GameLogic {
        +GameBoard game_board
        +Option~Bot~ bot
        +Option~Opponent~ opponent
        +Color player_turn
        +GameState game_state

        +new() GameLogic
        +execute_bot_move()
        +execute_opponent_move() bool
        +promote_piece(promotion_cursor: u8)
        +switch_player_turn()
        +update_game_state()
    }

    class GameBoard {
        +Vec~Chess~ position_history
        +Vec~Move~ move_history
        +i32 consecutive_non_pawn_or_capture
        +Vec~Piece~ taken_pieces
        +bool is_flipped
        
        +new() GameBoard
        +position_ref() Chess
        +current_position() Option~Chess~
        +move_to_san(index: usize) String
        +execute_move(from: Square, to: Square, promotion: Option~Role~) Option~Move~
        +get_authorized_positions(player_turn: Color, square: Square) Vec~Square~
        +get_role_at_square(square: Square) Option~Role~
        +get_piece_color_at_square(square: Square) Option~Color~
        +white_taken_pieces() Vec~Role~
        +black_taken_pieces() Vec~Role~
        +reset()
        +flip_the_board()
        +is_checkmate() bool
        +is_draw() bool
        +is_draw_by_repetition() bool
        +fen_position() String
    }


    class UI {
        +Coord cursor_coordinates
        +Option~Square~ selected_square
        +i32 selected_piece_cursor
        +i32 promotion_cursor
        +Coord old_cursor_position
        +u16 top_x
        +u16 top_y
        +u16 width
        +u16 height
        +bool mouse_used
        +DisplayMode display_mode
        +Prompt prompt
        
        +reset()
        +is_cell_selected() bool
        +move_selected_piece_cursor(first_time_moving: bool, direction: i8, authorized_positions: Vec~Coord~)
        +cursor_up(authorized_positions: Vec~Coord~)
        +cursor_down(authorized_positions: Vec~Coord~)
        +cursor_left(authorized_positions: Vec~Coord~)
        +cursor_left_promotion()
        +cursor_right(authorized_positions: Vec~Coord~)
        +cursor_right_promotion()
        +unselect_cell()
        +history_render(area: Rect, frame: Frame, game: Game)
        +white_material_render(area: Rect, frame: Frame, white_taken_pieces: Vec~Role~)
        +black_material_render(area: Rect, frame: Frame, black_taken_pieces: Vec~Role~)
        +board_render(area: Rect, frame: Frame, logic: GameLogic)
    }

    class Bot {
        +String engine_path
        +bool bot_will_move
        +bool is_bot_starting
        +u8 depth
        
        +new(engine_path: str, is_bot_starting: bool, depth: u8) Bot
        +get_move(fen: str) UciMove
    }

    class Opponent {
        +Option~TcpStream~ stream
        +bool opponent_will_move
        +Color color
        +bool game_started

        +copy() Opponent
        +new(addr: String, color: Option~Color~) Opponent
        +start_stream(addr: str)
        +send_end_game_to_server()
        +send_move_to_server(move: Move, promotion: Option~Role~)
        +read_stream() String
    }

    class Coord {
        +u8 row
        +u8 col
        
        +is_valid() bool
        +new(row: u8, col: u8) Coord
        +to_square() Option~Square~
        +try_to_square() Option~Square~
        +from_square(square: Square) Coord
    }

    class Color {
        <<enumeration>>
        White
        Black
    }

    class Role {
        <<enumeration>>
        King
        Queen
        Rook
        Bishop
        Knight
        Pawn
    }

    class GameState {
        <<enumeration>>
        Checkmate
        Draw
        Playing
        Promotion
    }

    App --> Game : "owns"
    Game "1" --> "1" GameLogic
    Game "1" --> "1" UI
    GameLogic "1" --> "1" GameBoard
    GameLogic "1" --> "0..1" Bot
    GameLogic "1" --> "0..1" Opponent
    GameLogic "1" --> "1" GameState : "has state"
    GameLogic "1" --> "1" Color : "player turn"
    GameBoard "1" --> "0..*" Move : "move history"
    UI "1" --> "1" Coord : "cursor position"
    Bot "1" --> "1" Move : "generates"
    Opponent "1" --> "1" Color : "color"
```



## Architecture Overview

This class diagram provides a comprehensive overview of the chess-tui architecture and how different components interact.

## Core Components

### App

The `App` struct is the main application container that orchestrates the entire application. It manages application lifecycle, configuration, bot computation, and multiplayer setup.

[See App documentation for details →](./App.md)

### Game

The `Game` struct represents a chess game session. It has been refactored to separate concerns:
- **UI state** (`ui` field) - cursor position, selected pieces, display settings
- **Game logic** (`logic` field) - board state, move execution, game rules

This separation improves code maintainability and reduces borrow checker issues.

### GameLogic

The `GameLogic` struct encapsulates all game logic, separating it from UI concerns. It manages board state, move execution, bot/opponent handling, and game state tracking.

[See GameLogic documentation for details →](./GameLogic.md)

### GameBoard

The `GameBoard` struct manages the chess board state using the **shakmaty** chess library. It maintains position history, move history, and handles all chess rule validation.

[See GameBoard documentation for details →](./GameBoard.md)

### UI

The `UI` struct handles all rendering and user interaction. It manages cursor position, piece selection, and renders the board, move history, and captured pieces.

[See UI documentation for details →](./UI.md)

### Bot

The `Bot` struct interfaces with UCI-compatible chess engines using the **ruci** library. It manages the engine process and converts between FEN positions and UCI moves.

[See Bot documentation for details →](./Bot.md)

### Opponent

The `Opponent` struct handles online multiplayer via TCP streams. It manages network communication, move synchronization, and color assignment between players.

[See Opponent documentation for details →](./Opponent.md)

## Supporting Types

- **Coord**: Board coordinates (row, col) with safe conversion to/from shakmaty `Square`
- **Color**: Player color (White/Black) from shakmaty
- **Role**: Piece type (King, Queen, Rook, Bishop, Knight, Pawn) from shakmaty
- **GameState**: Current game state enum (Playing, Checkmate, Draw, Promotion)
- **Square**: Board square representation from shakmaty (a1-h8)
- **Move**: Chess move representation from shakmaty with promotion support
- **Chess**: Chess position type from shakmaty, maintains board state and legal moves


