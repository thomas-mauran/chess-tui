use crate::{
    constants::{DisplayMode, BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{PieceColor, PieceMove, PieceType},
    utils::{
        col_to_letter, convert_notation_into_position, convert_position_into_notation,
        did_piece_already_move, get_cell_paragraph, get_int_from_char, get_king_coordinates,
        get_piece_color, get_piece_type, invert_position, is_getting_checked,
    },
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};
use uci::Engine;

/// only the pure gameboard, no additional information
pub type GameBoard = [[Option<(PieceType, PieceColor)>; 8]; 8];

#[derive(PartialEq, Clone, Debug, Eq, PartialOrd, Ord, Copy)]
pub struct Coord {
    /// rank, horizontal row, line, y axis
    pub row: u8,
    /// file, vertical column, x axis
    pub col: u8,
}
impl Coord {
    pub fn new<U1: Into<u8>, U2: Into<u8>>(row: U1, col: U2) -> Self {
        Coord {
            row: row.into(),
            col: col.into(),
        }
    }
    /// optional new: try making a valid [`Coord`], if can't, return [`None`]
    pub fn opt_new<U1: TryInto<u8>, U2: TryInto<u8>>(row: U1, col: U2) -> Option<Self> {
        let row: u8 = row.try_into().ok()?;
        let col: u8 = col.try_into().ok()?;

        let ret = Coord { row, col };
        if ret.is_valid() {
            Some(ret)
        } else {
            None
        }
    }
    /// not yet set position, has to later be set and only used afterwards
    pub fn undefined() -> Self {
        Coord {
            row: UNDEFINED_POSITION,
            col: UNDEFINED_POSITION,
        }
    }
    /// checks whether `self` is valid as a chess board coordinate
    pub fn is_valid(&self) -> bool {
        (0..8).contains(&self.col) && (0..8).contains(&self.row)
    }
}

impl std::ops::Index<&Coord> for GameBoard {
    type Output = Option<(PieceType, PieceColor)>;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self[index.row as usize][index.col as usize]
    }
}

impl std::ops::IndexMut<&Coord> for GameBoard {
    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        &mut self[index.row as usize][index.col as usize]
    }
}

/// ## visual representation
///
/// ### how it's stored:
///
/// . 0 1 2 3 4 5 6 7 .
/// 0 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 0
/// 1 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 1
/// 2 . . . . . . . . 2
/// 3 . . . . . . . . 3
/// 4 . . . . . . . . 4
/// 5 . . . . . . . . 5
/// 6 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 6
/// 7 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 7
/// . 0 1 2 3 4 5 6 7 .
///
/// ### how it's rendered:
///
/// . a b c d e f g h .
/// 8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 8
/// 7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 7
/// 6 . . . . . . . . 6
/// 5 . . . . . . . . 5
/// 4 . . . . . . . . 4
/// 3 . . . . . . . . 3
/// 2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 2
/// 1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 1
/// . a b c d e f g h .
pub struct Board {
    // the actual board
    pub board: GameBoard,
    // the cursor position
    pub cursor_coordinates: Coord,
    // the selected cell
    pub selected_coordinates: Coord,
    // the selected piece cursor when we already selected a piece
    pub selected_piece_cursor: i8,
    // the old cursor position used when unslecting a cell
    pub old_cursor_position: Coord,
    // the player turn
    pub player_turn: PieceColor,
    // the move history
    pub move_history: Vec<PieceMove>,
    // historic of the past position of the board
    pub board_history: Vec<GameBoard>,
    // if the game is a draw
    pub is_draw: bool,
    // if the game is a checkmate
    pub is_checkmate: bool,
    // if we are doing a promotion
    pub is_promotion: bool,
    // the cursor for the promotion popup
    pub promotion_cursor: i8,
    // the number of consecutive non pawn or capture moves
    pub consecutive_non_pawn_or_capture: i32,
    // the chess engine
    pub engine: Option<Engine>,
    // if the game is against a bot
    pub is_game_against_bot: bool,
    // the display mode
    pub display_mode: DisplayMode,
    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
    // The white piece that got taken
    pub white_taken_pieces: Vec<PieceType>,
    // The black piece that got taken
    pub black_taken_pieces: Vec<PieceType>,
}

fn init_board() -> GameBoard {
    [
        [
            Some((PieceType::Rook, PieceColor::Black)),
            Some((PieceType::Knight, PieceColor::Black)),
            Some((PieceType::Bishop, PieceColor::Black)),
            Some((PieceType::Queen, PieceColor::Black)),
            Some((PieceType::King, PieceColor::Black)),
            Some((PieceType::Bishop, PieceColor::Black)),
            Some((PieceType::Knight, PieceColor::Black)),
            Some((PieceType::Rook, PieceColor::Black)),
        ],
        [
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
        ],
        [
            Some((PieceType::Rook, PieceColor::White)),
            Some((PieceType::Knight, PieceColor::White)),
            Some((PieceType::Bishop, PieceColor::White)),
            Some((PieceType::Queen, PieceColor::White)),
            Some((PieceType::King, PieceColor::White)),
            Some((PieceType::Bishop, PieceColor::White)),
            Some((PieceType::Knight, PieceColor::White)),
            Some((PieceType::Rook, PieceColor::White)),
        ],
    ]
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: init_board(),
            board_history: vec![init_board()],
            cursor_coordinates: Coord::new(4, 4),
            selected_coordinates: Coord::undefined(),
            selected_piece_cursor: 0,
            old_cursor_position: Coord::undefined(),
            player_turn: PieceColor::White,
            move_history: vec![],
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            promotion_cursor: 0,
            consecutive_non_pawn_or_capture: 0,
            engine: None,
            is_game_against_bot: false,
            display_mode: DisplayMode::DEFAULT,
            is_bot_starting: false,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }
}

impl Board {
    pub fn new(board: GameBoard, player_turn: PieceColor, move_history: Vec<PieceMove>) -> Self {
        Self {
            board,
            board_history: Vec::new(),
            cursor_coordinates: Coord::new(4, 4),
            selected_coordinates: Coord::undefined(),
            selected_piece_cursor: 0,
            old_cursor_position: Coord::undefined(),
            player_turn,
            move_history,
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            promotion_cursor: 0,
            consecutive_non_pawn_or_capture: 0,
            engine: None,
            is_game_against_bot: false,
            display_mode: DisplayMode::DEFAULT,
            is_bot_starting: false,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }

    // Setters
    pub fn set_board(&mut self, board: GameBoard) {
        self.board = board;
    }

    pub fn set_player_turn(&mut self, player_turn: PieceColor) {
        self.player_turn = player_turn;
    }

    pub fn set_engine(&mut self, engine_path: &str) {
        self.is_game_against_bot = true;

        self.engine = match Engine::new(engine_path) {
            Ok(engine) => Some(engine),
            _ => panic!("An error occcured with the selected chess engine path: {engine_path} Make sure you specified the right path using chess-tui -e"),
        }
    }

    // Check if a cell has been selected
    fn is_cell_selected(&self) -> bool {
        self.selected_coordinates.row != UNDEFINED_POSITION
            && self.selected_coordinates.col != UNDEFINED_POSITION
    }

    fn get_authorized_positions(
        &self,
        piece_type: Option<PieceType>,
        piece_color: Option<PieceColor>,
        coordinates: Coord,
    ) -> Vec<Coord> {
        match (piece_type, piece_color) {
            (Some(piece_type), Some(piece_color)) => piece_type.authorized_positions(
                &coordinates,
                piece_color,
                self.board,
                &self.move_history,
                is_getting_checked(self.board, self.player_turn, &self.move_history),
            ),
            _ => Vec::new(),
        }
    }

    pub fn switch_player_turn(&mut self) {
        match self.player_turn {
            PieceColor::White => self.player_turn = PieceColor::Black,
            PieceColor::Black => self.player_turn = PieceColor::White,
        }
    }

    pub fn flip_the_board(&mut self) {
        let mut flipped_board = [[None; 8]; 8]; // Create a new empty board of the same type

        for (i, row) in self.board.iter().enumerate() {
            for (j, &square) in row.iter().enumerate() {
                // Place each square in the mirrored position
                flipped_board[7 - i][7 - j] = square;
            }
        }

        self.board = flipped_board;
    }

    // Cursor movement methods
    pub fn cursor_up(&mut self) {
        if !self.is_checkmate && !self.is_draw && !self.is_promotion {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, -1);
            } else if self.cursor_coordinates.row > 0 {
                self.cursor_coordinates.row -= 1;
            }
        }
    }
    pub fn cursor_down(&mut self) {
        if !self.is_checkmate && !self.is_draw && !self.is_promotion {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, 1);
            } else if self.cursor_coordinates.row < 7 {
                self.cursor_coordinates.row += 1;
            }
        }
    }
    pub fn cursor_left(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.is_promotion {
            self.promotion_cursor = if self.promotion_cursor > 0 {
                self.promotion_cursor - 1
            } else {
                3
            };
        } else if !self.is_checkmate && !self.is_draw {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, -1);
            } else if self.cursor_coordinates.col > 0 {
                self.cursor_coordinates.col -= 1;
            }
        }
    }
    pub fn cursor_right(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.is_promotion {
            self.promotion_cursor = (self.promotion_cursor + 1) % 4;
        } else if !self.is_checkmate && !self.is_draw {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, 1);
            } else if self.cursor_coordinates.col < 7 {
                self.cursor_coordinates.col += 1;
            }
        }
    }

    // Method to unselect a cell
    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_coordinates = Coord::undefined();
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position;
        }
    }

    /* Method to move the selected piece cursor
       We make sure that the cursor is in the authorized positions
    */
    fn move_selected_piece_cursor(&mut self, first_time_moving: bool, direction: i8) {
        let piece_color = get_piece_color(self.board, &self.selected_coordinates);
        let piece_type = get_piece_type(self.board, &self.selected_coordinates);

        let mut authorized_positions =
            self.get_authorized_positions(piece_type, piece_color, self.selected_coordinates);

        if authorized_positions.is_empty() {
            self.cursor_coordinates = Coord::undefined();
        } else {
            self.selected_piece_cursor = if self.selected_piece_cursor == 0 && first_time_moving {
                0
            } else {
                let new_cursor =
                    (self.selected_piece_cursor + direction) % authorized_positions.len() as i8;
                if new_cursor == -1 {
                    authorized_positions.len() as i8 - 1
                } else {
                    new_cursor
                }
            };

            authorized_positions.sort();

            if let Some(position) = authorized_positions.get(self.selected_piece_cursor as usize) {
                self.cursor_coordinates = *position;
            }
        }
    }

    // Methods to select a cell on the board
    pub fn select_cell(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.is_promotion {
            self.promote_piece();
        } else if !self.is_checkmate && !self.is_draw {
            if self.is_cell_selected() {
                // We already selected a piece so we apply the move
                if self.cursor_coordinates.is_valid() {
                    let selected_coords_usize = &self.selected_coordinates.clone();
                    let cursor_coords_usize = &self.cursor_coordinates.clone();
                    self.move_piece_on_the_board(selected_coords_usize, cursor_coords_usize);
                    self.unselect_cell();
                    self.switch_player_turn();
                    self.is_draw = self.is_draw();
                    if (!self.is_game_against_bot || self.is_bot_starting)
                        && (!self.is_latest_move_promotion()
                            || self.is_draw()
                            || self.is_checkmate())
                    {
                        self.flip_the_board();
                    }
                    // If we play against a bot we will play his move and switch the player turn again
                    if self.is_game_against_bot {
                        self.is_promotion = self.is_latest_move_promotion();
                        if !self.is_promotion {
                            self.is_checkmate = self.is_checkmate();
                            self.is_promotion = self.is_latest_move_promotion();
                            if !self.is_checkmate {
                                self.bot_move();
                                self.switch_player_turn();
                            }
                        }
                    }
                }
            } else {
                // Check if the piece on the cell can move before selecting it
                let piece_color = get_piece_color(self.board, &self.cursor_coordinates);
                let piece_type = get_piece_type(self.board, &self.cursor_coordinates);

                let authorized_positions =
                    self.get_authorized_positions(piece_type, piece_color, self.cursor_coordinates);

                if authorized_positions.is_empty() {
                    return;
                }
                if let Some(piece_color) = get_piece_color(self.board, &self.cursor_coordinates) {
                    if piece_color == self.player_turn {
                        self.selected_coordinates = self.cursor_coordinates;
                        self.old_cursor_position = self.cursor_coordinates;
                        self.move_selected_piece_cursor(true, 1);
                    }
                }
            }
        }
        self.is_checkmate = self.is_checkmate();
        self.is_promotion = self.is_latest_move_promotion();
        self.is_draw = self.is_draw();
    }

    // Check if the king has already moved (used for castling)
    pub fn did_king_already_move(&self) -> bool {
        for i in 0..self.move_history.len() {
            if self.move_history[i].piece_type == PieceType::King
                && self.player_turn as usize == i % 2
            {
                return true;
            }
        }
        false
    }

    /* Method to make a move for the bot
       We use the UCI protocol to communicate with the chess engine
    */
    pub fn bot_move(&mut self) {
        let engine = self.engine.clone().expect("Missing the chess engine");
        let fen_position = self.fen_position();

        engine.set_position(&(fen_position as String)).unwrap();
        let best_move = engine.bestmove();
        let Ok(movement) = best_move else {
            panic!("An error has occured")
        };
        let converted_move = convert_notation_into_position(&movement);

        let from_y = get_int_from_char(converted_move.chars().next());
        let from_x = get_int_from_char(converted_move.chars().nth(1));
        let to_y = get_int_from_char(converted_move.chars().nth(2));
        let to_x = get_int_from_char(converted_move.chars().nth(3));

        let mut promotion_piece: Option<PieceType> = None;
        if movement.chars().count() == 5 {
            promotion_piece = match movement.chars().nth(4) {
                Some('q') => Some(PieceType::Queen),
                Some('r') => Some(PieceType::Rook),
                Some('b') => Some(PieceType::Bishop),
                Some('n') => Some(PieceType::Knight),
                _ => None,
            };
        }

        self.move_piece_on_the_board(
            &Coord::new(from_y as u8, from_x as u8),
            &Coord::new(to_y as u8, to_x as u8),
        );

        if promotion_piece.is_some() {
            self.board[to_y as usize][to_x as usize] =
                Some((promotion_piece.unwrap(), self.player_turn));
        }
        if self.is_bot_starting {
            self.flip_the_board();
        }
    }

    // Convert the history and game status to a FEN string
    pub fn fen_position(&mut self) -> String {
        let mut result = String::new();
        let bot_color = if self.is_bot_starting {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        // We loop through the board and convert it to a FEN string
        for i in 0..8u8 {
            for j in 0..8u8 {
                // We get the piece type and color
                let (piece_type, piece_color) = (
                    get_piece_type(self.board, &Coord::new(i, j)),
                    get_piece_color(self.board, &Coord::new(i, j)),
                );
                let letter = PieceType::piece_to_fen_enum(piece_type, piece_color);
                // Pattern match directly on the result of piece_to_fen_enum
                match letter {
                    "" => {
                        // Check if the string is not empty before using chars().last()
                        if let Some(last_char) = result.chars().last() {
                            if last_char.is_ascii_digit() {
                                let incremented_char =
                                    char::from_digit(last_char.to_digit(10).unwrap_or(0) + 1, 10)
                                        .unwrap_or_default();
                                // Remove the old number and add the new incremented one
                                result.pop();
                                result.push_str(incremented_char.to_string().as_str());
                            } else {
                                result.push('1');
                            }
                        } else {
                            result.push('1');
                        }
                    }
                    letter => {
                        // If the result is not an empty string, push '1'
                        result.push_str(letter);
                    }
                };
            }
            result.push('/');
        }

        // we remove the last / and specify the player turn (black)
        result.pop();

        // We say it is blacks turn to play
        result.push_str(if bot_color == PieceColor::Black {
            " b"
        } else {
            " w"
        });

        // We add the castles availabilities for black
        if !did_piece_already_move(
            &self.move_history,
            (
                Some(PieceType::King),
                Some(self.player_turn),
                Coord::new(0, 4),
            ),
        ) && !is_getting_checked(self.board, PieceColor::Black, &self.move_history)
        {
            // king side black castle availability
            if !did_piece_already_move(
                &self.move_history,
                (
                    Some(PieceType::Rook),
                    Some(self.player_turn),
                    Coord::new(0, 7),
                ),
            ) {
                result.push_str(" k");
            }
            // queen side black castle availability
            if !did_piece_already_move(
                &self.move_history,
                (
                    Some(PieceType::Rook),
                    Some(self.player_turn),
                    Coord::new(0, 0),
                ),
            ) {
                result.push('q');
            }
        } else {
            result.push_str(" -");
        }

        // We check if the latest move is a pawn moving 2 cells, meaning the next move can be en passant
        if self.did_pawn_move_two_cells() {
            // Use an if-let pattern for better readability
            if let Some(last_move) = self.move_history.last() {
                let mut converted_move = String::new();

                converted_move += &col_to_letter(last_move.from.col);
                // FEN starts counting from 1 not 0
                converted_move += &format!("{}", 8 - last_move.from.row + 1).to_string();

                result.push(' ');
                result.push_str(&converted_move);
            }
        } else {
            result.push_str(" -");
        }

        result.push(' ');

        result.push_str(&self.consecutive_non_pawn_or_capture.to_string());
        result.push(' ');

        result.push_str(&(self.move_history.len() / 2).to_string());

        result
    }

    // Check if the pawn moved two cells (used for en passant)
    pub fn did_pawn_move_two_cells(&self) -> bool {
        match self.move_history.last() {
            Some(last_move) => {
                let distance = (last_move.to.row as i8 - last_move.from.row as i8).abs();

                if last_move.piece_type == PieceType::Pawn && distance == 2 {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    // Method to promote a pawn
    pub fn promote_piece(&mut self) {
        if let Some(last_move) = self.move_history.last() {
            let new_piece = match self.promotion_cursor {
                0 => PieceType::Queen,
                1 => PieceType::Rook,
                2 => PieceType::Bishop,
                3 => PieceType::Knight,
                _ => unreachable!("Promotion cursor out of boundaries"),
            };

            let current_piece_color =
                get_piece_color(self.board, &Coord::new(last_move.to.row, last_move.to.col));
            if let Some(piece_color) = current_piece_color {
                // we replace the piece by the new piece type
                self.board[last_move.to.row as usize][last_move.to.col as usize] =
                    Some((new_piece, piece_color));
            }
        }
        self.is_promotion = false;
        self.promotion_cursor = 0;
        if !self.is_draw() && !self.is_checkmate() {
            self.flip_the_board();
        }
    }

    // Move a piece from a cell to another
    pub fn move_piece_on_the_board(&mut self, from: &Coord, to: &Coord) {
        if !from.is_valid() || !to.is_valid() {
            return;
        }
        let direction_y: i32 = if self.player_turn == PieceColor::White {
            -1
        } else {
            1
        };

        let piece_type_from = get_piece_type(self.board, from);
        let piece_type_to = get_piece_type(self.board, to);

        // Check if moving a piece
        let Some(piece_type_from) = piece_type_from else {
            return;
        };

        // We increment the consecutive_non_pawn_or_capture if the piece type is a pawn or if there is no capture
        match (piece_type_from, piece_type_to) {
            (PieceType::Pawn, _) | (_, Some(_)) => {
                self.consecutive_non_pawn_or_capture = 0;
            }
            _ => {
                self.consecutive_non_pawn_or_capture += 1;
            }
        }

        // We check if the move is a capture and add the piece to the taken pieces
        match (piece_type_from, piece_type_to) {
            (_, None) => {}
            (_, Some(piece)) => {
                let piece_color = get_piece_color(self.board, to);
                // We check if there is a piece and we are not doing a castle
                if piece_color.is_some()
                    && (piece_type_to != Some(PieceType::Rook)
                        && piece_color != Some(self.player_turn))
                {
                    match piece_color {
                        Some(PieceColor::Black) => {
                            self.white_taken_pieces.push(piece);
                            self.white_taken_pieces.sort();
                        }
                        Some(PieceColor::White) => {
                            self.black_taken_pieces.push(piece);
                            self.black_taken_pieces.sort();
                        }
                        _ => {}
                    }
                }
            }
        }

        // We check for en passant as the latest move
        if self.is_latest_move_en_passant(*from, *to) {
            // we kill the pawn
            let row_index = to.row as i32 - direction_y;

            self.board[row_index as usize][to.col as usize] = None;
        }

        // We check for castling as the latest move
        if self.is_latest_move_castling(*from, *to) {
            // we set the king 2 cells on where it came from
            let from_x: i32 = from.col as i32;
            let mut new_to = to;
            let to_x: i32 = to.col as i32;

            let distance = from_x - to_x;
            // We set the direction of the rook > 0 meaning he went on the left else on the right
            let direction_x = if distance > 0 { -1 } else { 1 };

            let col_king = from_x + direction_x * 2;

            // We put move the king 2 cells
            self.board[to.row as usize][col_king as usize] = self.board[from];

            // We put the rook 3 cells from it's position if it's a big castling else 2 cells
            // If it is playing against a bot we will receive 4 -> 6  and 4 -> 2 for to_x instead of 4 -> 7 and 4 -> 0
            if self.is_game_against_bot && to_x == 6 && to.row == 0 {
                new_to = &Coord { row: 0, col: 7 };
            }
            if self.is_game_against_bot && to_x == 2 && to.row == 0 {
                new_to = &Coord { row: 0, col: 0 };
            }

            let col_rook = if distance > 0 {
                col_king + 1
            } else {
                col_king - 1
            };

            self.board[new_to.row as usize][col_rook as usize] =
                Some((PieceType::Rook, self.player_turn));

            // We remove the latest rook
            self.board[new_to] = None;
        } else {
            self.board[to] = self.board[from];
        }

        self.board[from] = None;

        // We store it in the history
        self.move_history.push(PieceMove {
            piece_type: piece_type_from,
            piece_color: self.player_turn,
            from: *from,
            to: *to,
        });
        // We store the current position of the board
        self.board_history.push(self.board);
    }

    // Method to get the number of authorized positions for the current player (used for the end condition)
    pub fn number_of_authorized_positions(&self) -> usize {
        let mut possible_moves: Vec<Coord> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                let coord = Coord::new(i, j);
                if let Some((piece_type, piece_color)) = self.board[&coord] {
                    if piece_color == self.player_turn {
                        possible_moves.extend(self.get_authorized_positions(
                            Some(piece_type),
                            Some(piece_color),
                            coord,
                        ));
                    }
                }
            }
        }
        possible_moves.len()
    }

    // Check if the latest move is en passant
    fn is_latest_move_en_passant(&self, from: Coord, to: Coord) -> bool {
        let piece_type_from = get_piece_type(self.board, &from);
        let piece_type_to = get_piece_type(self.board, &to);

        let from_y: i32 = from.row as i32;
        let from_x: i32 = from.col as i32;
        let to_y: i32 = to.row as i32;
        let to_x: i32 = to.col as i32;
        match (piece_type_from, piece_type_to) {
            (Some(PieceType::Pawn), _) => {
                // Check if it's a diagonal move, and the destination is an empty cell
                from_y != to_y && from_x != to_x && self.board[&to].is_none()
            }
            _ => false,
        }
    }

    // Check if the latest move is castling
    fn is_latest_move_castling(&self, from: Coord, to: Coord) -> bool {
        let piece_type_from = get_piece_type(self.board, &from);
        let piece_type_to = get_piece_type(self.board, &to);

        let from_x: i32 = from.col as i32;
        let to_x: i32 = to.col as i32;
        let distance = (from_x - to_x).abs();

        match (piece_type_from, piece_type_to) {
            (Some(PieceType::King), _) => distance > 1,
            _ => false,
        }
    }

    // Check if the latest move is a promotion
    fn is_latest_move_promotion(&self) -> bool {
        if let Some(last_move) = self.move_history.last() {
            if let Some(piece_type_to) =
                get_piece_type(self.board, &Coord::new(last_move.to.row, last_move.to.col))
            {
                let last_row = 0;
                if last_move.to.row == last_row && piece_type_to == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }

    // Check if the game is checkmate
    pub fn is_checkmate(&self) -> bool {
        if !is_getting_checked(self.board, self.player_turn, &self.move_history) {
            return false;
        }

        self.number_of_authorized_positions() == 0
    }

    // Check if the game is a draw
    pub fn draw_by_repetition(&mut self) -> bool {
        // A new game has started
        if self.move_history.is_empty() {
            self.board_history.clear();
            self.board_history.push(self.board);
            return false;
        }

        // Index mapping
        let mut position_counts = std::collections::HashMap::new();
        for board in self.board_history.iter() {
            let count = position_counts.entry(board).or_insert(0);
            *count += 1;

            if *count >= 3 {
                return true;
            }
        }

        false
    }

    // Check if the game is a draw
    pub fn is_draw(&mut self) -> bool {
        self.number_of_authorized_positions() == 0
            || self.consecutive_non_pawn_or_capture == 50
            || self.draw_by_repetition()
    }

    // Method to render the board
    pub fn board_render(&self, area: Rect, frame: &mut Frame) {
        let width = area.width / 8;
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);
        // We have 8 vertical lines
        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    // spread the excess border
                    Constraint::Length(border_height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(border_height),
                ]
                .as_ref(),
            )
            .split(area);

        // For each line we set 8 layout
        for i in 0..8u8 {
            let lines = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(border_width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(border_width),
                    ]
                    .as_ref(),
                )
                .split(columns[i as usize + 1]);
            for j in 0..8u8 {
                // Color of the cell to draw the board
                let cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                let last_move;
                let mut last_move_from = Coord::undefined();
                let mut last_move_to = Coord::undefined();
                if !self.move_history.is_empty() {
                    last_move = self.move_history.last();
                    if self.is_game_against_bot && !self.is_bot_starting {
                        last_move_from = last_move.map(|m| m.from).unwrap();
                        last_move_to = last_move.map(|m| m.to).unwrap();
                    } else {
                        last_move_from = invert_position(&last_move.map(|m| m.from).unwrap());
                        last_move_to = invert_position(&last_move.map(|m| m.to).unwrap());
                    }
                }

                let mut positions: Vec<Coord> = vec![];
                let is_cell_in_positions = |positions: &Vec<Coord>, i: u8, j: u8| {
                    positions.iter().any(|&coord| coord == Coord::new(i, j))
                };
                // Draw the available moves for the selected piece
                if self.is_cell_selected() {
                    let selected_piece_type =
                        get_piece_type(self.board, &self.selected_coordinates);
                    let selected_piece_color: Option<PieceColor> =
                        get_piece_color(self.board, &self.selected_coordinates);
                    positions = self.get_authorized_positions(
                        selected_piece_type,
                        selected_piece_color,
                        self.selected_coordinates,
                    );

                    // Draw grey if the color is in the authorized positions
                    for coords in positions.clone() {
                        if i == coords.row && j == coords.col {
                            // cell_color = Color::Rgb(100, 100, 100);
                        }
                    }
                }

                let square = lines[j as usize + 1];
                // Here we have all the possibilities for a cell:
                // - selected cell: green
                // - cursor cell: blue
                // - available move cell: grey
                // - checked king cell: magenta
                // - last move cell: green
                // - default cell: white or black
                // Draw the cell blue if this is the current cursor cell
                if i == self.cursor_coordinates.row && j == self.cursor_coordinates.col {
                    Board::render_cell(frame, square, Color::LightBlue, None);
                }
                // Draw the cell magenta if the king is getting checked
                else if is_getting_checked(self.board, self.player_turn, &self.move_history)
                    && Coord::new(i, j) == get_king_coordinates(self.board, self.player_turn)
                {
                    Board::render_cell(frame, square, Color::Magenta, Some(Modifier::SLOW_BLINK));
                }
                // Draw the cell green if this is the selected cell or if the cell is part of the last move
                else if (i == self.selected_coordinates.row && j == self.selected_coordinates.col)
                    || (last_move_from == Coord::new(i, j) // If the last move from 
                        || (last_move_to == Coord::new(i, j) // If last move to
                            && !is_cell_in_positions(&positions, i, j)))
                // and not in the authorized positions (grey instead of green)
                {
                    Board::render_cell(frame, square, Color::LightGreen, None);
                } else if is_cell_in_positions(&positions, i, j) {
                    Board::render_cell(frame, square, Color::Rgb(100, 100, 100), None);
                }
                // else as a last resort we draw the cell with the default color either white or black
                else {
                    let mut cell = Block::default();
                    cell = match self.display_mode {
                        DisplayMode::DEFAULT => cell.bg(cell_color),
                        DisplayMode::ASCII => match cell_color {
                            WHITE => cell.bg(Color::White).fg(Color::Black),
                            BLACK => cell.bg(Color::Black).fg(Color::White),
                            _ => cell.bg(cell_color),
                        },
                    };
                    frame.render_widget(cell.clone(), square);
                }

                // Get piece and color
                let coord = Coord::new(i, j);
                let paragraph = get_cell_paragraph(self, &coord, square);

                frame.render_widget(paragraph, square);
            }
        }
    }
    fn render_cell(frame: &mut Frame, square: Rect, color: Color, modifier: Option<Modifier>) {
        let mut cell = Block::default().bg(color);
        if let Some(modifier) = modifier {
            cell = cell.add_modifier(modifier);
        }
        frame.render_widget(cell, square);
    }

    // Method to render the right panel history
    pub fn history_render(&self, area: Rect, frame: &mut Frame) {
        // We write the history board on the side
        let history_block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(5, 10, 1, 2));

        let mut lines: Vec<Line> = vec![];

        for i in (0..self.move_history.len()).step_by(2) {
            let piece_type_from = self.move_history[i].piece_type;

            let utf_icon_white =
                PieceType::piece_to_utf_enum(piece_type_from, Some(PieceColor::White));
            let move_white = convert_position_into_notation(&format!(
                "{}{}{}{}",
                self.move_history[i].from.row,
                self.move_history[i].from.col,
                self.move_history[i].to.row,
                self.move_history[i].to.col
            ));

            let mut utf_icon_black = "   ";
            let mut move_black: String = "   ".to_string();

            // If there is something for black
            if i + 1 < self.move_history.len() {
                let piece_type_to = self.move_history[i + 1].piece_type;

                move_black = convert_position_into_notation(&format!(
                    "{}{}{}{}",
                    self.move_history[i + 1].from.row,
                    self.move_history[i + 1].from.col,
                    self.move_history[i + 1].to.row,
                    self.move_history[i + 1].to.col
                ));
                utf_icon_black =
                    PieceType::piece_to_utf_enum(piece_type_to, Some(PieceColor::Black));
            }

            lines.push(Line::from(vec![
                Span::raw(format!("{}.  ", i / 2 + 1)), // line number
                Span::styled(format!("{utf_icon_white} "), Style::default().fg(WHITE)), // white symbol
                Span::raw(move_white.to_string()), // white move
                Span::raw("     "),                // separator
                Span::styled(format!("{utf_icon_black} "), Style::default().fg(WHITE)), // white symbol
                Span::raw(move_black.to_string()), // black move
            ]));
        }

        let history_paragraph = Paragraph::new(lines).alignment(Alignment::Center);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        frame.render_widget(history_block.clone(), right_panel_layout[0]);
        frame.render_widget(
            history_paragraph,
            history_block.inner(right_panel_layout[0]),
        );
    }

    pub fn white_material_render(&self, area: Rect, frame: &mut Frame) {
        let white_block = Block::default()
            .title("White material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for i in 0..self.white_taken_pieces.len() {
            let utf_icon_white =
                PieceType::piece_to_utf_enum(self.white_taken_pieces[i], Some(PieceColor::Black));

            pieces.push_str(&format!("{utf_icon_white} "));
        }

        let white_material_paragraph = Paragraph::new(pieces)
            .alignment(Alignment::Center)
            .add_modifier(Modifier::BOLD);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        frame.render_widget(white_block.clone(), right_panel_layout[0]);
        frame.render_widget(
            white_material_paragraph,
            white_block.inner(right_panel_layout[0]),
        );
        // Bottom paragraph help text
        let text = vec![Line::from("Press ? for help").alignment(Alignment::Center)];

        let help_paragraph = Paragraph::new(text)
            .block(Block::new())
            .alignment(Alignment::Center);
        frame.render_widget(help_paragraph, right_panel_layout[1]);
    }

    pub fn black_material_render(&self, area: Rect, frame: &mut Frame) {
        let black_block = Block::default()
            .title("Black material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for i in 0..self.black_taken_pieces.len() {
            let utf_icon_black =
                PieceType::piece_to_utf_enum(self.black_taken_pieces[i], Some(PieceColor::White));

            pieces.push_str(&format!("{utf_icon_black} "));
        }

        let black_material_paragraph = Paragraph::new(pieces)
            .alignment(Alignment::Center)
            .add_modifier(Modifier::BOLD);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        frame.render_widget(black_block.clone(), right_panel_layout[0]);
        frame.render_widget(
            black_material_paragraph,
            black_block.inner(right_panel_layout[0]),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, Coord},
        pieces::{PieceColor, PieceMove, PieceType},
        utils::is_getting_checked,
    };

    #[test]
    fn is_getting_checked_true() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(is_getting_checked(custom_board, PieceColor::White, &[]));
    }

    #[test]
    fn is_getting_checked_false() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::White, &[]));
    }

    #[test]
    fn is_getting_checked_piece_in_front_false() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::Black, &[]));
    }

    #[test]
    fn is_getting_checked_piece_in_with_gap_false() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::Black, &[]));
    }

    #[test]
    fn is_checkmate_true() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(board.is_checkmate());
    }

    #[test]
    fn is_checkmate_false() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(!board.is_checkmate());
    }

    #[test]
    fn is_checkmate_false_2() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Queen, PieceColor::White)),
                None,
            ],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(!board.is_checkmate());
    }

    #[test]
    fn is_draw_true() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(board.is_draw());
    }

    #[test]
    fn is_draw_false() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(!board.is_draw());
    }

    #[test]
    fn is_promote_false() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];
        let board = Board::new(
            custom_board,
            PieceColor::Black,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    from: Coord::new(7, 3),
                    to: Coord::new(6, 3),
                }),
            ],
        );

        assert!(!board.is_latest_move_promotion());
    }
    #[test]
    fn is_promote_true() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];
        let board = Board::new(
            custom_board,
            PieceColor::Black,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    from: Coord::new(1, 4),
                    to: Coord::new(0, 4),
                }),
            ],
        );

        assert!(board.is_latest_move_promotion());
    }

    #[test]
    fn promote_and_checkmate() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];
        // We setup the board
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        assert!(!board.is_latest_move_promotion());

        // Move the pawn to a promote cell
        board.move_piece_on_the_board(&Coord::new(1, 4), &Coord::new(0, 4));
        assert!(board.is_latest_move_promotion());

        // Promote the pawn
        board.promote_piece();

        // The black king gets checkmated
        board.player_turn = PieceColor::Black;
        assert!(board.is_checkmate());
    }

    #[test]
    fn is_promote_true_black() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::Black,
                    from: Coord::new(1, 4),
                    to: Coord::new(0, 4),
                }),
            ],
        );

        assert!(board.is_latest_move_promotion());
    }

    #[test]
    fn promote_and_draw() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
            [
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the board
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        assert!(!board.is_latest_move_promotion());

        // Move the pawn to a promote cell
        board.move_piece_on_the_board(&Coord::new(1, 5), &Coord::new(0, 5));
        assert!(board.is_latest_move_promotion());

        // Promote the pawn
        board.promote_piece();

        // The black king gets checkmated
        board.player_turn = PieceColor::White;
        assert!(board.is_draw());
    }
    #[test]
    fn fifty_moves_draw() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the board
        let mut board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                // We don't use the history for a fifty draw
            ],
        );

        board.consecutive_non_pawn_or_capture = 49;
        assert!(!board.is_draw());

        // Move the pawn to a make the 50th move
        board.move_piece_on_the_board(&Coord::new(1, 6), &Coord::new(1, 5));
        assert!(board.is_draw());
    }

    #[test]
    fn consecutive_position_draw() {
        let custom_board = [
            [
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        // We setup the board
        let mut board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 2),
                    to: Coord::new(0, 1),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 6),
                    to: Coord::new(0, 5),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 1),
                    to: Coord::new(0, 2),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 5),
                    to: Coord::new(0, 6),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 2),
                    to: Coord::new(0, 1),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 6),
                    to: Coord::new(0, 5),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 1),
                    to: Coord::new(0, 2),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 5),
                    to: Coord::new(0, 6),
                }),
            ],
        );

        let mut copy_move_history = board.move_history.clone();

        for piece_move in copy_move_history.iter_mut() {
            board.move_piece_on_the_board(&piece_move.from, &piece_move.to);

            // In a chess game, board.is_draw() is called after every move
            assert!(!board.is_draw());
        }

        // Move the king to replicate a third time the same position
        board.move_piece_on_the_board(&Coord::new(0, 2), &Coord::new(0, 1));
        assert!(board.is_draw());
    }

    #[test]
    fn fen_converter_1() {
        let custom_board = [
            [
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the board
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);

        // Move the king to replicate a third time the same position
        assert_eq!(board.fen_position(), "2k4R/8/4K3/8/8/8/8/8 b - - 0 0");
    }

    #[test]
    fn fen_converter_en_passant() {
        let custom_board = [
            [
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the board
        let mut board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    from: Coord::new(6, 2),
                    to: Coord::new(4, 2),
                }),
            ],
        );

        // Move the king to replicate a third time the same position
        assert_eq!(board.fen_position(), "2k4R/8/4K3/8/2P5/8/8/8 b - c3 0 0");
    }
    #[test]
    fn fen_converter_castling() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];
        // We setup the board
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);

        // Move the king to replicate a third time the same position
        assert_eq!(
            board.fen_position(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b kq - 0 0"
        );
    }
}
