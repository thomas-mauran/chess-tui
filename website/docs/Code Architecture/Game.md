## Global architecture

```mermaid
classDiagram

class App {
        +bool running
        +Game game
        +Pages current_page
        +Option<Popups> current_popup
        +Option<PieceColor> selected_color
        +Option<bool> hosting
        +Option<String> host_ip
        +u8 menu_cursor
        +Option<String> chess_engine_path

        +toggle_help_popup()
        +toggle_credit_popup()
        +go_to_home()
        +tick()
        +quit()
        +menu_cursor_up(l: u8)
        +menu_cursor_right(l: u8)
        +menu_cursor_left(l: u8)
        +menu_cursor_down(l: u8)
        +color_selection()
        +restart()
        +menu_select()
        +update_config()
        +setup_game_server(host_color: PieceColor)
        +create_opponent()
        +hosting_selection()
        +bot_setup()
        +get_host_ip() IpAddr
    }
    class Game {
        +GameBoard game_board
        +UI ui
        +Option<Bot> bot
        +Option<Opponent> opponent
        +PieceColor player_turn
        +GameState game_state

        +new(game_board: GameBoard, player_turn: PieceColor)
        +set_board(game_board: GameBoard)
        +set_player_turn(player_turn: PieceColor)
        +switch_player_turn()
        +select_cell()
        +execute_bot_move()
        +promote_piece()
        +execute_move(from: Coord, to: Coord)
        +handle_cell_click()
        +already_selected_cell_action()
        +handle_promotion()
        +execute_opponent_move()
        +handle_multiplayer_promotion()
    }

    class GameBoard {
        +board : Board
        +move_history: Vec<PieceMove>
        +board_history: Vec<Board>
        +consecutive_non_pawn_or_capture: i32
        +white_taken_pieces: Vec<PieceType>
        +black_taken_pieces: Vec<PieceType>
        +new(board: Board, move_history: List<PieceMove>, board_history: List<Board>) : GameBoard
        +get_authorized_positions(player_turn: PieceColor, coordinates: Coord) : List<Coord>
        +add_piece_to_taken_pieces(from: Coord, to: Coord, player_turn: PieceColor) : void
        +reset() : void
        +flip_the_board() : void
        +is_checkmate(player_turn: PieceColor) : bool
        +is_draw() : bool
        +fen_position(is_bot_starting: bool, player_turn: PieceColor) : string
    }


    class UI {
        +cursor_coordinates : Coord
        +selected_coordinates : Coord
        +selected_piece_cursor : int
        +promotion_cursor : int
        +old_cursor_position : Coord
        +top_x : u16
        +top_y : u16
        +width : u16
        +height : u16
        +mouse_used : bool
        +display_mode : DisplayMode
        +reset() : void
        +is_cell_selected() : bool
        +move_selected_piece_cursor(first_time_moving: bool, direction: i8, authorized_positions: Vec<Coord>) : void
        +cursor_up(authorized_positions: Vec<Coord>) : void
        +cursor_down(authorized_positions: Vec<Coord>) : void
        +cursor_left(authorized_positions: Vec<Coord>) : void
        +cursor_left_promotion() : void
        +cursor_right(authorized_positions: Vec<Coord>) : void
        +cursor_right_promotion() : void
        +unselect_cell() : void
        +history_render(area: Rect, frame: Frame, move_history: Vec<PieceMove>) : void
        +white_material_render(area: Rect, frame: Frame, white_taken_pieces: Vec<PieceType>) : void
        +black_material_render(area: Rect, frame: Frame, black_taken_pieces: Vec<PieceType>) : void
        +board_render(area: Rect, frame: Frame, game: Game) : void
    }

    class Bot {
        +engine: Engine
        +bot_will_move: bool
        +is_bot_starting
        +set_engine(engine_path: &str)
        +create_engine(engine_path: &str): Engine
        +get_bot_move(fen_position: String): String
    }

    class Opponent {
        +Option<TcpStream> stream
        +bool opponent_will_move
        +PieceColor color
        +bool game_started

        +copy() : Opponent
        +new(addr: String, color: Option<PieceColor>) : Opponent
        +start_stream(addr: &str) : void
        +send_end_game_to_server() : void
        +send_move_to_server(move_to_send: &PieceMove, promotion_type: Option<String>) : void
        +read_stream() : String
    }

    class Coord {
        +row: int
        +col: int
        +is_valid(): bool
        +new(row: int, col: int): Coord
    }

    class PieceMove {
        +piece_type: PieceType
        +piece_color: PieceColor
        +from: Coord
        +to: Coord
    }

    class PieceColor {
        <<enumeration>>
        +White
        +Black
    }

    class PieceType {
        <<enumeration>>
        +King
        +Queen
        +Rook
        +Bishop
        +Knight
        +Pawn
    }

    class GameState {
        <<enumeration>>
        +Checkmate
        +Draw
        +Playing
        +Promotion
    }

    App --> Game : "owns"
    Game "1" --> "1" GameBoard
    Game --> "1" GameState : type
    Game "1" --> "1" UI
    Game "1" --> "0..1" Bot
    GameBoard "1" --> "0..*" PieceMove
    GameBoard "1" --> "0..*" Coord
    UI "1" --> "1" Coord
    Bot "1" --> "1" PieceMove : generates
    PieceMove "1" --> "1" Coord : from_to
    Coord "1" --> "1" PieceColor : color
    Coord "1" --> "1" PieceType : type
    Game "1" --> "0..1" Opponent : "has"
    Opponent "1" --> "1" PieceColor : color
```


This Class diagram allows us to have a quick overview of the game architecture and how the different struct interact.

### App

The App struct is the main struct of the game. It will be the one storing the game when we play it as well as the different pages we can navigate to.


### Game

The Game struct represent a new game. As variable a game stores a GameBoard, a UI, a Bot (if it's again a bot), the player turn and the game state.


### GameBoard

The GameBoard struct represent everything to the board, the table storing the pieces, the move history, the taken pieces, the consecutive non pawn or capture moves and the player turn.

### UI

The UI struct represent the user interface. It stores the cursor position, the selected piece, the promotion cursor, the old cursor position, the top left corner of the board, the width and height of the board, if the mouse is used and the display mode.
It also handles the rendering of the board, taken pieces and the rendering of the move history.