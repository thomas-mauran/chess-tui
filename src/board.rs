use crate::{
    constants::{DisplayMode, BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{PieceColor, PieceMove, PieceType},
    utils::{
        col_to_letter, convert_notation_into_position, convert_position_into_notation,
        did_piece_already_move, get_cell_paragraph, get_int_from_char, get_king_coordinates,
        get_piece_color, get_piece_type, get_player_turn_in_modulo, is_getting_checked, is_valid,
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

pub struct Board {
    pub board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    pub display_board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    pub cursor_coordinates: [i8; 2],
    pub selected_coordinates: [i8; 2],
    pub selected_piece_cursor: i8,
    pub old_cursor_position: [i8; 2],
    pub player_turn: PieceColor,
    pub move_history: Vec<PieceMove>,
    pub history_position: usize,
    pub is_draw: bool,
    pub is_checkmate: bool,
    pub is_promotion: bool,
    pub promotion_cursor: i8,
    pub consecutive_non_pawn_or_capture: i32,
    pub engine: Option<Engine>,
    pub is_game_against_bot: bool,
    pub display_mode: DisplayMode,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
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
            ],
            display_board: [
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
            ],
            cursor_coordinates: [4, 4],
            selected_coordinates: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            selected_piece_cursor: 0,
            old_cursor_position: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            player_turn: PieceColor::White,
            move_history: vec![],
            history_position: 0,
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            promotion_cursor: 0,
            consecutive_non_pawn_or_capture: 0,
            engine: None,
            is_game_against_bot: false,
            display_mode: DisplayMode::DEFAULT,
        }
    }
}

impl Board {
    pub fn new(
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        player_turn: PieceColor,
        move_history: Vec<PieceMove>,
    ) -> Self {
        Self {
            board,
            display_board: board,
            cursor_coordinates: [4, 4],
            selected_coordinates: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            selected_piece_cursor: 0,
            old_cursor_position: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            player_turn,
            move_history: move_history.clone(),
            history_position: move_history.len(),
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            promotion_cursor: 0,
            consecutive_non_pawn_or_capture: 0,
            engine: None,
            is_game_against_bot: false,
            display_mode: DisplayMode::DEFAULT,
        }
    }

    // Setters
    pub fn set_board(&mut self, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) {
        self.board = board;
    }

    pub fn set_player_turn(&mut self, player_turn: PieceColor) {
        self.player_turn = player_turn;
    }

    pub fn set_engine(&mut self, engine_path: &str) {
        self.is_game_against_bot = true;

        self.engine = match Engine::new(engine_path) {
            Ok(engine) => Some(engine),
            _ => panic!("An error occcured with the selected chess engine path: {} Make sure you specified the right path using chess-tui -e", engine_path),
        }
    }

    // Check if a cell has been selected
    fn is_cell_selected(&self) -> bool {
        self.selected_coordinates[0] != UNDEFINED_POSITION
            && self.selected_coordinates[1] != UNDEFINED_POSITION
    }

    fn get_authorized_positions(
        &self,
        piece_type: Option<PieceType>,
        piece_color: Option<PieceColor>,
        coordinates: [i8; 2],
    ) -> Vec<Vec<i8>> {
        match (piece_type, piece_color) {
            (Some(piece_type), Some(piece_color)) => piece_type.authorized_positions(
                coordinates,
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

    // Cursor movement methods
    pub fn cursor_up(&mut self) {
        if !self.is_checkmate && !self.is_draw && !self.is_promotion {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, -1)
            } else if self.cursor_coordinates[0] > 0 {
                self.cursor_coordinates[0] -= 1
            }
        }
    }
    pub fn cursor_down(&mut self) {
        if !self.is_checkmate && !self.is_draw && !self.is_promotion {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, 1)
            } else if self.cursor_coordinates[0] < 7 {
                self.cursor_coordinates[0] += 1
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
                self.move_selected_piece_cursor(false, -1)
            } else if self.cursor_coordinates[1] > 0 {
                self.cursor_coordinates[1] -= 1
            }
        }
    }
    pub fn cursor_right(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.is_promotion {
            self.promotion_cursor = (self.promotion_cursor + 1) % 4;
        } else if !self.is_checkmate && !self.is_draw {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, 1)
            } else if self.cursor_coordinates[1] < 7 {
                self.cursor_coordinates[1] += 1
            }
        }
    }

    // Method to unselect a cell
    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_coordinates[0] = UNDEFINED_POSITION;
            self.selected_coordinates[1] = UNDEFINED_POSITION;
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position
        }
    }

    /* Method to move the selected piece cursor
       We make sure that the cursor is in the authorized positions
    */
    fn move_selected_piece_cursor(&mut self, first_time_moving: bool, direction: i8) {
        let piece_color = get_piece_color(self.board, self.selected_coordinates);
        let piece_type = get_piece_type(self.board, self.selected_coordinates);

        let mut authorized_positions =
            self.get_authorized_positions(piece_type, piece_color, self.selected_coordinates);

        if !authorized_positions.is_empty() {
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
                self.cursor_coordinates = [position[0], position[1]];
            }
        } else {
            self.cursor_coordinates = [UNDEFINED_POSITION, UNDEFINED_POSITION];
        }
    }

    // Methods to select a cell on the board
    pub fn select_cell(&mut self) {
        // If looking at history, bring back to the present
        if self.is_history_mode() {
            self.display_board = self.board;
            self.history_position = self.move_history.len();
            return;
        }

        // If we are doing a promotion the cursor is used for the popup
        if self.is_promotion {
            self.promote_piece();
        } else if !self.is_checkmate && !self.is_draw {
            if !self.is_cell_selected() {
                // Check if the piece on the cell can move before selecting it
                let piece_color = get_piece_color(self.board, self.cursor_coordinates);
                let piece_type = get_piece_type(self.board, self.cursor_coordinates);

                let authorized_positions =
                    self.get_authorized_positions(piece_type, piece_color, self.cursor_coordinates);

                if authorized_positions.is_empty() {
                    return;
                }
                if let Some(piece_color) = get_piece_color(self.board, self.cursor_coordinates) {
                    if piece_color == self.player_turn {
                        self.selected_coordinates = self.cursor_coordinates;
                        self.old_cursor_position = self.cursor_coordinates;
                        self.move_selected_piece_cursor(true, 1);
                    }
                }
            } else {
                // We already selected a piece
                if is_valid(self.cursor_coordinates) {
                    let selected_coords_usize: [usize; 2] = [
                        self.selected_coordinates[0] as usize,
                        self.selected_coordinates[1] as usize,
                    ];
                    let cursor_coords_usize: [usize; 2] = [
                        self.cursor_coordinates[0] as usize,
                        self.cursor_coordinates[1] as usize,
                    ];
                    self.move_piece_on_the_board(selected_coords_usize, cursor_coords_usize);
                    self.unselect_cell();
                    self.switch_player_turn();
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
                    self.is_draw = self.is_draw();
                }
            }
        }
        self.is_checkmate = self.is_checkmate();
        self.is_promotion = self.is_latest_move_promotion();
    }

    // Check if the king has already moved (used for castling)
    pub fn did_king_already_move(&self) -> bool {
        for i in 0..self.move_history.len() {
            if self.move_history[i].piece_type == PieceType::King
                && get_player_turn_in_modulo(self.player_turn) == i % 2
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
        let engine = match &self.engine {
            Some(engine) => engine,
            None => panic!("Missing the chess engine"),
        };

        engine.set_position(&self.fen_position()).unwrap();

        let best_move = engine.bestmove();
        let movement = match best_move {
            Ok(movement) => movement,
            Err(_) => panic!("An error as occured"),
        };
        let converted_move = convert_notation_into_position(movement);

        let from_y = get_int_from_char(converted_move.chars().next());
        let from_x = get_int_from_char(converted_move.chars().nth(1));
        let to_y = get_int_from_char(converted_move.chars().nth(2));
        let to_x = get_int_from_char(converted_move.chars().nth(3));

        self.move_piece_on_the_board(
            [from_y as usize, from_x as usize],
            [to_y as usize, to_x as usize],
        );
    }

    // Convert the history and game status to a FEN string
    pub fn fen_position(&self) -> String {
        let mut result = String::new();

        // We loop through the board and convert it to a FEN string
        for i in 0..8i8 {
            for j in 0..8i8 {
                // We get the piece type and color
                let (piece_type, piece_color) = (
                    get_piece_type(self.board, [i, j]),
                    get_piece_color(self.board, [i, j]),
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
            result.push('/')
        }

        // we remove the last / and specify the player turn (black)
        result.pop();

        // We say it is blacks turn to play
        result.push_str(" b");

        // We add the castles availabilities for black
        if !did_piece_already_move(&self.move_history, (Some(PieceType::King), [0, 4]))
            && !is_getting_checked(self.board, PieceColor::Black, &self.move_history)
        {
            // king side black castle availability
            if !did_piece_already_move(&self.move_history, (Some(PieceType::Rook), [0, 7])) {
                result.push_str(" k");
            }
            // queen side black castle availability
            if !did_piece_already_move(&self.move_history, (Some(PieceType::Rook), [0, 0])) {
                result.push('q');
            }
        } else {
            result.push_str(" -")
        }

        // We check if the latest move is a pawn moving 2 cells, meaning the next move can be en passant
        if self.did_pawn_move_two_cells() {
            // Use an if-let pattern for better readability
            if let Some(last_move) = self.move_history.last() {
                let mut converted_move = String::new();

                converted_move += &col_to_letter(last_move.from_x);
                // FEN starts counting from 1 not 0
                converted_move += &format!("{}", 8 - last_move.from_y + 1).to_string();

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
                let distance = (last_move.to_y - last_move.from_y).abs();

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

            let current_piece_color = get_piece_color(self.board, [last_move.to_y, last_move.to_x]);
            if let Some(piece_color) = current_piece_color {
                // we replace the piece by the new piece type
                self.board[last_move.to_y as usize][last_move.to_x as usize] =
                    Some((new_piece, piece_color));
                self.display_board[last_move.to_y as usize][last_move.to_x as usize] =
                    Some((new_piece, piece_color));

                // Promotion happened, update the previous record
                self.move_history[self.history_position - 1].promotion_piece = Some(new_piece);
            }
        }
        self.is_promotion = false;
        self.promotion_cursor = 0;
    }

    // Move a piece from a cell to another
    pub fn move_piece_on_the_board(&mut self, from: [usize; 2], to: [usize; 2]) {
        if !is_valid([from[0] as i8, from[1] as i8]) || !is_valid([to[0] as i8, to[1] as i8]) {
            return;
        }

        // Check if we are in the history, this will be used to know if we need
        // to update the real board later on
        let history = self.is_history_mode();

        let piece_type_from = get_piece_type(self.display_board, [from[0] as i8, from[1] as i8]);
        let mut piece_type_to = get_piece_type(self.display_board, [to[0] as i8, to[1] as i8]);

        // Check if moving a piece
        let piece_type_from = match piece_type_from {
            Some(piece) => piece,
            None => return,
        };

        // piece_type_to is used for captures, ignore it if castling
        if self.is_latest_move_castling(from, to) {
            piece_type_to = None;
        }

        // We increment the consecutive_non_pawn_or_capture if the piece type is
        // a pawn or if there is no capture and we are not looking at the
        // history
        if !history {
            match (piece_type_from, piece_type_to) {
                (PieceType::Pawn, _) | (_, Some(_)) => {
                    self.consecutive_non_pawn_or_capture = 0;
                }
                _ => {
                    self.consecutive_non_pawn_or_capture += 1;
                }
            }
        }

        // We check for en passant as the latest move
        let mut is_en_passant = false;
        if self.is_latest_move_en_passant(from, to) {
            is_en_passant = true;

            // we kill the pawn
            self.display_board[from[0]][to[1]] = None;
        }

        // We check for castling as the latest move
        if self.is_latest_move_castling(from, to) {
            // we set the king 2 cells on where it came from

            let from_x: i32 = from[1] as i32;
            let mut to_x: i32 = to[1] as i32;

            let distance = from_x - to_x;
            let direction_x = if distance > 0 { -1 } else { 1 };

            let row_index_rook;

            let row_index = from_x + direction_x * 2;

            // We put move the king 2 cells
            self.display_board[to[0]][row_index as usize] = self.display_board[from[0]][from[1]];

            // We put the rook 3 cells from it's position if it's a big castling else 2 cells
            // If it is playing against a bot we will receive 4 -> 6  and 4 -> 2 for to_x instead of 4 -> 7 and 4 -> 0
            // big castling
            match distance {
                distance if distance > 0 => {
                    row_index_rook = 3;
                    if self.is_game_against_bot && self.player_turn == PieceColor::Black {
                        to_x = 0;
                    }
                }
                distance if distance < 0 => {
                    row_index_rook = 5;
                    if self.is_game_against_bot && self.player_turn == PieceColor::Black {
                        to_x = 7;
                    }
                }
                _ => unreachable!("Undefined distance for castling"),
            }

            self.display_board[to[0]][row_index_rook as usize] =
                self.display_board[to[0]][to_x as usize];

            // We remove the latest rook
            self.display_board[to[0]][to_x as usize] = None;
        } else {
            self.display_board[to[0]][to[1]] = self.display_board[from[0]][from[1]];
        }

        self.display_board[from[0]][from[1]] = None;

        // History mode checks for en passant and promotions
        if history {
            if self.move_history[self.history_position].is_en_passant {
                // we kill the pawn
                self.display_board[from[0]][to[1]] = None;
            }

            if let Some(piece) = self.move_history[self.history_position].promotion_piece {
                let mut color = PieceColor::Black;
                if to[0] == 0 {
                    color = PieceColor::White;
                }

                self.display_board[to[0]][to[1]] = Some((piece, color));
            }
        }

        // We store it in the history if not looking at history and update board
        self.history_position += 1;
        if !history {
            self.board = self.display_board;
            self.move_history.push(PieceMove {
                piece_type: piece_type_from,
                piece_captured: piece_type_to,
                from_y: from[0] as i8,
                from_x: from[1] as i8,
                to_y: to[0] as i8,
                to_x: to[1] as i8,
                is_en_passant,
                promotion_piece: None,
            });
        }
    }

    // Method to get the number of authorized positions for the current player (used for the end condition)
    pub fn number_of_authorized_positions(&self) -> usize {
        let mut possible_moves: Vec<Vec<i8>> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                if let Some((piece_type, piece_color)) = self.board[i][j] {
                    if piece_color == self.player_turn {
                        possible_moves.extend(self.get_authorized_positions(
                            Some(piece_type),
                            Some(piece_color),
                            [i as i8, j as i8],
                        ))
                    }
                }
            }
        }
        possible_moves.len()
    }

    // Check if the latest move is en passant
    fn is_latest_move_en_passant(&self, from: [usize; 2], to: [usize; 2]) -> bool {
        let piece_type_from = get_piece_type(self.board, [from[0] as i8, from[1] as i8]);
        let piece_type_to = get_piece_type(self.board, [to[0] as i8, to[1] as i8]);

        let from_y: i32 = from[0] as i32;
        let from_x: i32 = from[1] as i32;
        let to_y: i32 = to[0] as i32;
        let to_x: i32 = to[1] as i32;
        match (piece_type_from, piece_type_to) {
            (Some(PieceType::Pawn), _) => {
                // Check if it's a diagonal move, and the destination is an empty cell
                from_y != to_y && from_x != to_x && self.board[to[0]][to[1]].is_none()
            }
            _ => false,
        }
    }

    // Check if the latest move is castling
    fn is_latest_move_castling(&self, from: [usize; 2], to: [usize; 2]) -> bool {
        let piece_type_from = get_piece_type(self.display_board, [from[0] as i8, from[1] as i8]);
        let piece_type_to = get_piece_type(self.display_board, [to[0] as i8, to[1] as i8]);

        let from_x: i32 = from[1] as i32;
        let to_x: i32 = to[1] as i32;
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
                get_piece_type(self.board, [last_move.to_y, last_move.to_x])
            {
                if let Some(piece_color) =
                    get_piece_color(self.board, [last_move.to_y, last_move.to_x])
                {
                    let last_row = if piece_color == PieceColor::White {
                        0
                    } else {
                        7
                    };

                    if last_move.to_y == last_row && piece_type_to == PieceType::Pawn {
                        return true;
                    }
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
    pub fn draw_by_repetition(&self) -> bool {
        if self.move_history.len() >= 9 {
            let last_ten: Vec<PieceMove> =
                self.move_history.iter().rev().take(9).cloned().collect();

            if (last_ten[0], last_ten[1]) == (last_ten[4], last_ten[5])
                && last_ten[4] == last_ten[8]
                && (last_ten[2], last_ten[3]) == (last_ten[6], last_ten[7])
            {
                return true;
            }
        }
        false
    }

    // Check if the game is a draw
    pub fn is_draw(&self) -> bool {
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
        for i in 0..8i8 {
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
            for j in 0..8i8 {
                // Color of the cell to draw the board
                let mut cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                // Draw the available moves for the selected piece and not
                // looking at history
                if self.is_cell_selected() && !self.is_history_mode() {
                    let selected_piece_type =
                        get_piece_type(self.display_board, self.selected_coordinates);
                    let selected_piece_color: Option<PieceColor> =
                        get_piece_color(self.display_board, self.selected_coordinates);
                    let positions = self.get_authorized_positions(
                        selected_piece_type,
                        selected_piece_color,
                        self.selected_coordinates,
                    );

                    // Draw grey if the color is in the authorized positions
                    for coords in positions.clone() {
                        if i == coords[0] && j == coords[1] {
                            cell_color = Color::Rgb(100, 100, 100)
                        }
                    }
                }

                let square = lines[j as usize + 1];
                // Draw the cell blue if this is the current cursor cell
                if i == self.cursor_coordinates[0] && j == self.cursor_coordinates[1] {
                    let cell = Block::default().bg(Color::LightBlue);
                    frame.render_widget(cell.clone(), square);
                } else if is_getting_checked(
                    self.display_board,
                    self.player_turn,
                    &self.move_history,
                ) && [i, j] == get_king_coordinates(self.display_board, self.player_turn)
                {
                    let cell = Block::default()
                        .bg(Color::Magenta)
                        .add_modifier(Modifier::SLOW_BLINK);
                    frame.render_widget(cell.clone(), square);
                }
                // Draw the cell green if this is the selected cell
                else if i == self.selected_coordinates[0]
                    && j == self.selected_coordinates[1]
                    && !self.is_history_mode()
                {
                    let cell = Block::default().bg(Color::LightGreen);
                    frame.render_widget(cell.clone(), square);
                } else {
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
                let paragraph = get_cell_paragraph(self, [i, j], square);

                frame.render_widget(paragraph, square);
            }
        }
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
            let move_white = convert_position_into_notation(format!(
                "{}{}{}{}",
                self.move_history[i].from_y,
                self.move_history[i].from_x,
                self.move_history[i].to_y,
                self.move_history[i].to_x
            ));

            let mut utf_icon_black = "   ";
            let mut move_black: String = "   ".to_string();

            // If there is something for black
            if i + 1 < self.move_history.len() {
                let piece_type_to = self.move_history[i + 1].piece_type;

                move_black = convert_position_into_notation(format!(
                    "{}{}{}{}",
                    self.move_history[i + 1].from_y,
                    self.move_history[i + 1].from_x,
                    self.move_history[i + 1].to_y,
                    self.move_history[i + 1].to_x
                ));
                utf_icon_black =
                    PieceType::piece_to_utf_enum(piece_type_to, Some(PieceColor::Black))
            }

            // Highlighting for history, i=0 on the first move but history_position=0 means 0 moves
            // have been played (representing the beginning of the game)
            let white_string = if i + 1 == self.history_position {
                Span::styled(move_white.to_string(), Style::default().bg(WHITE))
            } else {
                Span::raw(move_white.to_string())
            };

            // i+1 is the move in history and we need to add 1 for blacks move as well
            let black_string = if i + 2 == self.history_position {
                Span::styled(move_black.to_string(), Style::default().bg(WHITE))
            } else {
                Span::raw(move_black.to_string())
            };

            lines.push(Line::from(vec![
                Span::raw(format!("{}.  ", i / 2 + 1)), // line number
                Span::styled(format!("{} ", utf_icon_white), Style::default().fg(WHITE)), // white symbol
                white_string,       // white move
                Span::raw("     "), // separator
                Span::styled(format!("{} ", utf_icon_black), Style::default().fg(WHITE)), // white symbol
                black_string, // black move
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

        // Bottom paragraph help text
        let text = vec![Line::from("Press ? for help").alignment(Alignment::Center)];

        let help_paragraph = Paragraph::new(text)
            .block(Block::new())
            .alignment(Alignment::Center);
        frame.render_widget(help_paragraph, right_panel_layout[1]);
    }

    pub fn is_history_mode(&self) -> bool {
        if self.board == self.display_board {
            return false;
        }
        true
    }

    pub fn history_forward(&mut self) {
        // Check if already up to date
        if !self.is_history_mode() {
            return;
        }

        self.move_piece_on_the_board(
            [
                self.move_history[self.history_position].from_y as usize,
                self.move_history[self.history_position].from_x as usize,
            ],
            [
                self.move_history[self.history_position].to_y as usize,
                self.move_history[self.history_position].to_x as usize,
            ],
        );
    }

    pub fn history_backward(&mut self) {
        // Check if at the beginning of the game
        if self.history_position == 0 {
            return;
        }

        let previous_move = self.move_history[self.history_position - 1];

        // If it is whites turn we are undoing black's move
        let color_move = if self.history_position % 2 == 0 {
            PieceColor::Black
        } else {
            PieceColor::White
        };
        let color_not_move = if self.history_position % 2 == 0 {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        // Move the piece back to where it was using the color of the turn
        self.display_board[previous_move.from_y as usize][previous_move.from_x as usize] =
            Some((previous_move.piece_type, color_move));

        // Replace with what was captured (opposite color of the turn), even if
        // it was none
        match previous_move.piece_captured {
            Some(previous_piece) => {
                self.display_board[previous_move.to_y as usize][previous_move.to_x as usize] =
                    Some((previous_piece, color_not_move));
            }
            None => {
                self.display_board[previous_move.to_y as usize][previous_move.to_x as usize] = None
            }
        }

        // Check for castling
        let distance = (previous_move.from_x - previous_move.to_x).abs();

        if previous_move.piece_type == PieceType::King && distance > 1 {
            // Add rook back where it was
            self.display_board[previous_move.from_y as usize][previous_move.to_x as usize] =
                Some((PieceType::Rook, color_move));

            // Remove rook and king from previous spot, 0 is long castle
            if previous_move.to_x == 0 {
                self.display_board[previous_move.from_y as usize][2] = None; // King
                self.display_board[previous_move.from_y as usize][3] = None; // Rook
            } else {
                self.display_board[previous_move.from_y as usize][5] = None; // Rook
                self.display_board[previous_move.from_y as usize][6] = None; // King
            }
        }

        // Check for en passant and replace pawn
        if previous_move.is_en_passant {
            self.display_board[previous_move.from_y as usize][previous_move.to_x as usize] =
                Some((PieceType::Pawn, color_not_move));
        }

        // Decrement history position
        self.history_position -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
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
        let board = Board::new(custom_board, PieceColor::White, vec![]);

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
        let board = Board::new(custom_board, PieceColor::White, vec![]);

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
                    from_y: 7,
                    from_x: 3,
                    to_y: 6,
                    to_x: 3,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
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
                    from_y: 1,
                    from_x: 4,
                    to_y: 0,
                    to_x: 4,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: Some(PieceType::Queen),
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
        board.move_piece_on_the_board([1, 4], [0, 4]);
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
        ];
        let board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    from_y: 6,
                    from_x: 4,
                    to_y: 7,
                    to_x: 4,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: Some(PieceType::Queen),
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
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the board
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        assert!(!board.is_latest_move_promotion());

        // Move the pawn to a promote cell
        board.move_piece_on_the_board([6, 5], [7, 5]);
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
        board.move_piece_on_the_board([1, 6], [1, 5]);
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
                    from_y: 0,
                    from_x: 2,
                    to_y: 0,
                    to_x: 1,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    from_y: 0,
                    from_x: 6,
                    to_y: 0,
                    to_x: 5,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    from_y: 0,
                    from_x: 1,
                    to_y: 0,
                    to_x: 2,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    from_y: 0,
                    from_x: 5,
                    to_y: 0,
                    to_x: 6,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    from_y: 0,
                    from_x: 2,
                    to_y: 0,
                    to_x: 1,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    from_y: 0,
                    from_x: 6,
                    to_y: 0,
                    to_x: 5,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    from_y: 0,
                    from_x: 1,
                    to_y: 0,
                    to_x: 2,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    from_y: 0,
                    from_x: 5,
                    to_y: 0,
                    to_x: 6,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
                }),
            ],
        );

        assert!(!board.is_draw());

        // Move the king to replicate a third time the same position
        board.move_piece_on_the_board([0, 2], [0, 1]);
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
        let board = Board::new(custom_board, PieceColor::White, vec![]);

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
        let board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    from_y: 6,
                    from_x: 2,
                    to_y: 4,
                    to_x: 2,
                    is_en_passant: false,
                    piece_captured: None,
                    promotion_piece: None,
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
        let board = Board::new(custom_board, PieceColor::White, vec![]);

        // Move the king to replicate a third time the same position
        assert_eq!(
            board.fen_position(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b kq - 0 0"
        );
    }

    #[test]
    fn history_forward_backward() {
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

        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        board.move_piece_on_the_board([6, 0], [5, 0]);
        board.history_backward();
        board.history_forward();
    }

    // Checking overflow and underflow cases by going back and forth through
    // history in many directions
    #[test]
    fn history_forward_backward_overflow_underflow() {
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

        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        board.move_piece_on_the_board([6, 0], [5, 0]);
        board.move_piece_on_the_board([1, 0], [2, 0]);
        board.history_backward();
        board.history_forward();
        board.history_backward();
        board.history_backward();
        board.history_backward();
        board.history_backward();
        board.history_forward();
        board.history_forward();
        board.history_forward();
        board.history_forward();
    }

    #[test]
    fn history_promition_white() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
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
        ];
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        board.move_piece_on_the_board([1, 1], [0, 1]);
        board.promotion_cursor = 0;
        board.promote_piece();
        board.history_backward();
        board.history_backward();
        board.history_forward();
        board.history_forward();
        assert_eq!(board.display_board, board.board);
    }

    #[test]
    fn history_promition_black() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
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
            [
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.move_piece_on_the_board([6, 1], [7, 1]);
        board.promotion_cursor = 0;
        board.promote_piece();
        board.history_backward();
        board.history_backward();
        board.history_forward();
        board.history_forward();
        assert_eq!(board.display_board, board.board);
    }

    #[test]
    fn history_en_passant() {
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
                None,
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
            [
                Some((PieceType::Pawn, PieceColor::Black)),
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

        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        board.move_piece_on_the_board([6, 1], [4, 1]);
        board.move_piece_on_the_board([4, 0], [3, 1]);
        board.move_piece_on_the_board([2, 2], [3, 1]);
        board.history_backward();
        board.history_backward();
        board.history_backward();
        board.history_backward();
        board.history_forward();
        board.history_forward();
        board.history_forward();
        board.history_forward();
        assert_eq!(board.display_board, board.board);
    }

    #[test]
    fn history_castling() {
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
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];

        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        board.move_piece_on_the_board([7, 4], [7, 7]);
        board.history_backward();
        board.history_backward();
        board.history_backward();
        board.history_backward();
        board.history_forward();
        board.history_forward();
        board.history_forward();
        board.history_forward();
        assert_eq!(board.display_board, board.board);
    }
}
