use crate::{
    constants::{BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{PieceColor, PieceType},
    utils::{
        col_to_letter, color_to_ratatui_enum, convert_notation_into_position,
        convert_position_into_notation, did_piece_already_move, get_int_from_char,
        get_king_coordinates, get_piece_color, get_piece_type, get_player_turn_in_modulo,
        is_getting_checked, is_valid,
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

/// Coordinates
///
/// Columns/x/j:
/// |  |  |
/// |  |  |
/// |  |  |
/// 0  1  2
///
/// Rows/y/i:
/// _  _  _ 0
/// _  _  _ 1
/// _  _  _ 2
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Coord {
    /// column/x/j/[1]
    /// |  |  |
    /// |  |  |
    /// |  |  |
    /// 0  1  2
    pub col: i8,
    /// row/y/i/[0]
    /// _  _  _ 0
    /// _  _  _ 1
    /// _  _  _ 2
    pub row: i8,
}
impl Coord {
    pub fn new(col: i8, row: i8) -> Self {
        Self { col: row, row: col }
    }
    pub fn to_hist(&self) -> String {
        format!("{}{}", self.col, self.row)
    }
    /// Convert history record to coordinates
    pub fn from_hist(hist_item: &str) -> Self {
        assert_eq!(hist_item.chars().count(), 2);
        Self::new(
            hist_item
                .chars()
                .nth(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            hist_item
                .chars()
                .nth(0)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        )
    }
}
impl Default for Coord {
    fn default() -> Self {
        Self::new(UNDEFINED_POSITION, UNDEFINED_POSITION)
    }
}

pub struct Board {
    pub board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    pub cursor_coordinates: Coord,
    pub selected_coordinates: Coord,
    pub selected_piece_cursor: i8,
    pub old_cursor_position: Coord,
    pub player_turn: PieceColor,
    pub move_history: Vec<(Option<PieceType>, String)>,
    pub is_draw: bool,
    pub is_checkmate: bool,
    pub is_promotion: bool,
    pub promotion_cursor: i8,
    pub consecutive_non_pawn_or_capture: i32,
    pub engine: Option<Engine>,
    pub is_game_against_bot: bool,
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
            cursor_coordinates: Coord::new(4, 4),
            selected_coordinates: Coord::default(),
            selected_piece_cursor: 0,
            old_cursor_position: Coord::default(),
            player_turn: PieceColor::White,
            move_history: vec![],
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            promotion_cursor: 0,
            consecutive_non_pawn_or_capture: 0,
            engine: None,
            is_game_against_bot: false,
        }
    }
}

impl Board {
    pub fn new(
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        player_turn: PieceColor,
        move_history: Vec<(Option<PieceType>, String)>,
    ) -> Self {
        Self {
            board,
            cursor_coordinates: Coord::new(4, 4),
            selected_coordinates: Coord::default(),
            selected_piece_cursor: 0,
            old_cursor_position: Coord::default(),
            player_turn,
            move_history,
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            promotion_cursor: 0,
            consecutive_non_pawn_or_capture: 0,
            engine: None,
            is_game_against_bot: false,
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
        self.selected_coordinates.row != UNDEFINED_POSITION
            && self.selected_coordinates.col != UNDEFINED_POSITION
    }

    fn get_mut(&mut self, coord: &Coord) -> &mut Option<(PieceType, PieceColor)> {
        &mut self.board[coord.row as usize][coord.col as usize]
    }

    fn get_authorized_positions(
        &self,
        piece_type: Option<PieceType>,
        piece_color: Option<PieceColor>,
        coordinates: &Coord,
    ) -> Vec<Coord> {
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

    // Methods to change the position of the cursor
    pub fn cursor_up(&mut self) {
        if !self.is_checkmate && !self.is_draw && !self.is_promotion {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, -1)
            } else if self.cursor_coordinates.row > 0 {
                self.cursor_coordinates.row -= 1
            }
        }
    }
    pub fn cursor_down(&mut self) {
        if !self.is_checkmate && !self.is_draw && !self.is_promotion {
            if self.is_cell_selected() {
                self.move_selected_piece_cursor(false, 1)
            } else if self.cursor_coordinates.row < 7 {
                self.cursor_coordinates.row += 1
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
            } else if self.cursor_coordinates.col > 0 {
                self.cursor_coordinates.col -= 1
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
            } else if self.cursor_coordinates.col < 7 {
                self.cursor_coordinates.col += 1
            }
        }
    }

    pub fn did_king_already_move(&self) -> bool {
        for i in 0..self.move_history.len() {
            match self.move_history[i] {
                (Some(piece_type), _) => {
                    if piece_type == PieceType::King
                        && get_player_turn_in_modulo(self.player_turn) == i % 2
                    {
                        return true;
                    }
                }
                _ => unreachable!("Invalid move in history"),
            }
        }
        false
    }

    fn move_selected_piece_cursor(&mut self, first_time_moving: bool, direction: i8) {
        let piece_color = get_piece_color(self.board, &self.selected_coordinates);
        let piece_type = get_piece_type(self.board, &self.selected_coordinates);

        let mut authorized_positions =
            self.get_authorized_positions(piece_type, piece_color, &self.selected_coordinates);

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
                self.cursor_coordinates = position.clone();
            }
        } else {
            self.cursor_coordinates = Coord::default();
        }
    }

    // Methods to select a cell on the board
    pub fn select_cell(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.is_promotion {
            self.promote_piece();
        } else if !self.is_checkmate && !self.is_draw {
            if !self.is_cell_selected() {
                // Check if the piece on the cell can move before selecting it
                let piece_color = get_piece_color(self.board, &self.cursor_coordinates);
                let piece_type = get_piece_type(self.board, &self.cursor_coordinates);

                let authorized_positions = self.get_authorized_positions(
                    piece_type,
                    piece_color,
                    &self.cursor_coordinates,
                );

                if authorized_positions.is_empty() {
                    return;
                }
                if let Some(piece_color) = get_piece_color(self.board, &self.cursor_coordinates) {
                    if piece_color == self.player_turn {
                        self.selected_coordinates = self.cursor_coordinates.clone();
                        self.old_cursor_position = self.cursor_coordinates.clone();
                        self.move_selected_piece_cursor(true, 1);
                    }
                }
            } else {
                // We already selected a piece
                if is_valid(&self.cursor_coordinates) {
                    let selected_coords = &self.selected_coordinates.clone();
                    let cursor_coords = &self.cursor_coordinates.clone();
                    self.move_piece_on_the_board(selected_coords, cursor_coords);
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

        self.move_piece_on_the_board(&Coord::new(from_x, from_y), &Coord::new(to_x, to_y));
    }
    // Convert the history and game status to a FEN string
    pub fn fen_position(&self) -> String {
        let mut result = String::new();

        for i in 0..8i8 {
            for j in 0..8i8 {
                let (piece_type, piece_color) = (
                    get_piece_type(self.board, &Coord::new(i, j)),
                    get_piece_color(self.board, &Coord::new(i, j)),
                );
                match PieceType::piece_to_fen_enum(piece_type, piece_color) {
                    // Pattern match directly on the result of piece_to_fen_enum
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
                }
            }
            result.push('/')
        }

        // we remove the last / and specify the player turn (black)
        result.pop();

        // We say it is blacks turn to play
        result.push_str(" b");

        // We add the castles availabilities for black
        if !did_piece_already_move(
            &self.move_history,
            (Some(PieceType::King), Coord::new(0, 4)),
        ) && !is_getting_checked(self.board, PieceColor::Black, &self.move_history)
        {
            // king side black castle availability
            if !did_piece_already_move(
                &self.move_history,
                (Some(PieceType::Rook), Coord::new(0, 7)),
            ) {
                result.push_str(" k");
            }
            // queen side black castle availability
            if !did_piece_already_move(
                &self.move_history,
                (Some(PieceType::Rook), Coord::new(0, 0)),
            ) {
                result.push('q');
            }
        } else {
            result.push_str(" -")
        }

        // We check if the latest move is a pawn moving 2 cells, meaning the next move can be en passant
        if self.did_pawn_move_two_cells() {
            // Use an if-let pattern for better readability
            if let Some((_, latest_move_string)) = self.move_history.last() {
                let mut converted_move: String = String::new();

                if let (Some(from_y_char), Some(from_x_char)) = (
                    latest_move_string.chars().nth(0),
                    latest_move_string.chars().nth(1),
                ) {
                    let from_y = get_int_from_char(Some(from_y_char)) - 1;
                    let from_x = get_int_from_char(Some(from_x_char));

                    converted_move += &col_to_letter(from_x);
                    converted_move += &format!("{}", 8 - from_y).to_string();

                    result.push(' ');
                    result.push_str(&converted_move);
                }
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

    pub fn did_pawn_move_two_cells(&self) -> bool {
        match self.move_history.last() {
            Some((Some(piece_type), move_string)) => {
                let from_y = get_int_from_char(move_string.chars().next());
                let to_y = get_int_from_char(move_string.chars().nth(2));

                let distance = (to_y - from_y).abs();

                if piece_type == &PieceType::Pawn && distance == 2 {
                    return true;
                }
                false
            }
            _ => false,
        }
    }
    pub fn promote_piece(&mut self) {
        if let Some(position) = self.move_history.last() {
            let to_y = get_int_from_char(position.1.chars().nth(2));
            let to_x = get_int_from_char(position.1.chars().nth(3));

            let new_piece = match self.promotion_cursor {
                0 => PieceType::Queen,
                1 => PieceType::Rook,
                2 => PieceType::Bishop,
                3 => PieceType::Knight,
                _ => unreachable!("Promotion cursor out of boundaries"),
            };

            let current_piece_color = get_piece_color(self.board, &Coord::new(to_x, to_y));
            if let Some(piece_color) = current_piece_color {
                // we replace the piece by the new piece type
                self.board[to_y as usize][to_x as usize] = Some((new_piece, piece_color));
            }
        }
        self.is_promotion = false;
        self.promotion_cursor = 0;
    }

    pub fn move_piece_on_the_board(&mut self, from: &Coord, to: &Coord) {
        if !is_valid(from) || !is_valid(to) {
            return;
        }
        let direction_y = if self.player_turn == PieceColor::White {
            -1
        } else {
            1
        };

        let piece_type_from = get_piece_type(self.board, from);
        let piece_type_to = get_piece_type(self.board, to);
        let position_number: String = format!("{}{}", from.to_hist(), to.to_hist());

        // We increment the consecutive_non_pawn_or_capture if the piece type is a pawn or if there is no capture
        match (piece_type_from, piece_type_to) {
            (Some(PieceType::Pawn), _) | (Some(_), Some(_)) => {
                self.consecutive_non_pawn_or_capture = 0;
            }
            _ => {
                self.consecutive_non_pawn_or_capture += 1;
            }
        }

        let tuple = (piece_type_from, position_number);

        // We check for en passant as the latest move
        if self.is_latest_move_en_passant(from, to) {
            // we kill the pawn
            let row_index = to.row as i32 - direction_y;

            self.board[row_index as usize][to.col as usize] = None;
        }

        // We check for castling as the latest move
        if self.is_latest_move_castling(from, to) {
            // we set the king 2 cells on where it came from

            let from_x: i32 = from.col as i32;
            let mut to_x: i32 = to.col as i32;

            let distance = from_x - to_x;
            let direction_x = if distance > 0 { -1 } else { 1 };

            let mut row_index_rook = 0;

            let row_index = from_x + direction_x * 2;

            // We put move the king 2 cells
            self.board[to.row as usize][row_index as usize] =
                self.board[from.row as usize][from.col as usize];

            // We put the rook 3 cells from it's position if it's a big castling else 2 cells
            // If it is playing against a bot we will receive 4 -> 6  and 4 -> 2 for to_x instead of 4 -> 7 and 4 -> 0
            // big castling
            if distance > 0 {
                row_index_rook = 3;
                if self.is_game_against_bot && self.player_turn == PieceColor::Black {
                    to_x = 0;
                }
            } else if distance < 0 {
                row_index_rook = 5;
                if self.is_game_against_bot && self.player_turn == PieceColor::Black {
                    to_x = 7;
                }
            }

            self.board[to.row as usize][row_index_rook as usize] =
                self.board[to.row as usize][to_x as usize];

            // We remove the latest rook
            self.board[to.row as usize][to_x as usize] = None;
        } else {
            self.board[to.row as usize][to.col as usize] =
                self.board[from.row as usize][from.col as usize];
        }

        self.board[from.row as usize][from.col as usize] = None;

        // We store it in the history
        self.move_history.push(tuple.clone());
    }

    /// move history of `self` contains this coordinate, either as moved to or from
    fn history_has(&self, coord: &Coord, to: bool) -> Option<PieceType> {
        let hist = &self.move_history;
        if hist.is_empty() {
            return None;
        }

        let mut i = hist.len() - 1;
        while i > 0 {
            let hist_rec = &hist[i].1;
            if to {
                if hist_rec[2..4] == coord.to_hist() {
                    return hist[i].0;
                }
            } else if hist_rec[0..2] == coord.to_hist() {
                return hist[i].0;
            }
            i -= 1;
        }
        // if let Some(pos) = Self::default().board[coord[0]][coord[1]] {
        //     Some(pos.0)
        // } else {
        None
        // }
    }

    /// takeback
    pub fn takeback(&mut self) {
        if let Some((_, prev_move)) = self.move_history.pop() {
            let to = Coord::from_hist(&prev_move[0..2]);
            let from = Coord::from_hist(&prev_move[2..4]);

            // take last moved piece back to where it came from
            self.board[to.row as usize][to.col as usize] =
                self.board[from.row as usize][from.col as usize];

            // pseudo kind of code
            // if history.contains(board[from], Moved::To) && !history.contains(board[from], Moved::From) {
            //     board[from] = history[from]
            // }

            // optionally fill the cell if something was taken off it
            self.board[from.row as usize][from.col as usize] =
                // check if there was anything on the cell where it was before takeback:
                // if anything has moved to this cell and not away from it, there probably was
                if self.history_has(&from, true).is_some() && self.history_has(&from, false).is_none()  {
                    let piece_type = self.history_has(&from, true).unwrap();
                    Some((piece_type, self.player_turn))
                } else if let Some((piece_type, _)) = Self::default().board[from.row as usize][from.col as usize] {
                    Some((piece_type, self.player_turn))
                } else {
                        None
                };

            self.switch_player_turn();
        }
    }

    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_coordinates = Coord::default();
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position.clone()
        }
    }

    pub fn number_of_authorized_positions(&self) -> usize {
        let mut possible_moves: Vec<Coord> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                if let Some((piece_type, piece_color)) = self.board[i][j] {
                    if piece_color == self.player_turn {
                        possible_moves.extend(self.get_authorized_positions(
                            Some(piece_type),
                            Some(piece_color),
                            &Coord::new(i as i8, j as i8),
                        ))
                    }
                }
            }
        }
        possible_moves.len()
    }

    fn is_latest_move_en_passant(&self, from: &Coord, to: &Coord) -> bool {
        let piece_type_from = get_piece_type(self.board, from);
        let piece_type_to = get_piece_type(self.board, to);

        let from_y = from.row as i32;
        let from_x = from.col as i32;
        let to_y = to.row as i32;
        let to_x = to.col as i32;
        match (piece_type_from, piece_type_to) {
            (Some(PieceType::Pawn), _) => {
                // Check if it's a diagonal move, and the destination is an empty cell
                from_y != to_y
                    && from_x != to_x
                    && self.board[to.row as usize][to.col as usize].is_none()
            }
            _ => false,
        }
    }

    fn is_latest_move_castling(&self, from: &Coord, to: &Coord) -> bool {
        let piece_type_from = get_piece_type(self.board, from);
        let piece_type_to = get_piece_type(self.board, to);

        let from_x = from.col as i32;
        let to_x = to.col as i32;
        let distance = (from_x - to_x).abs();

        match (piece_type_from, piece_type_to) {
            (Some(PieceType::King), _) => distance > 1,
            _ => false,
        }
    }

    fn is_latest_move_promotion(&self) -> bool {
        if let Some(position) = self.move_history.last() {
            let to_y = get_int_from_char(position.1.chars().nth(2));
            let to_x = get_int_from_char(position.1.chars().nth(3));

            if let Some(piece_type_from) = get_piece_type(self.board, &Coord::new(to_y, to_x)) {
                if let Some(piece_color) = get_piece_color(self.board, &Coord::new(to_y, to_x)) {
                    let last_row = if piece_color == PieceColor::White {
                        0
                    } else {
                        7
                    };

                    if to_y == last_row && piece_type_from == PieceType::Pawn {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_checkmate(&self) -> bool {
        if !is_getting_checked(self.board, self.player_turn, &self.move_history) {
            return false;
        }

        self.number_of_authorized_positions() == 0
    }

    pub fn draw_by_repetition(&self) -> bool {
        if self.move_history.len() >= 9 {
            let last_ten: Vec<(Option<PieceType>, String)> =
                self.move_history.iter().rev().take(9).cloned().collect();

            if (last_ten[0].clone(), last_ten[1].clone())
                == (last_ten[4].clone(), last_ten[5].clone())
                && last_ten[4].clone() == last_ten[8].clone()
                && (last_ten[2].clone(), last_ten[3].clone())
                    == (last_ten[6].clone(), last_ten[7].clone())
            {
                return true;
            }
        }
        false
    }

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

                // Draw the available moves for the selected piece
                if self.is_cell_selected() {
                    let selected_piece_type =
                        get_piece_type(self.board, &self.selected_coordinates);
                    let selected_piece_color: Option<PieceColor> =
                        get_piece_color(self.board, &self.selected_coordinates);
                    let positions = self.get_authorized_positions(
                        selected_piece_type,
                        selected_piece_color,
                        &self.selected_coordinates,
                    );

                    // Draw grey if the color is in the authorized positions
                    for coords in positions.clone() {
                        if i == coords.row && j == coords.col {
                            cell_color = Color::Rgb(100, 100, 100)
                        }
                    }
                }

                let square = lines[j as usize + 1];
                // Draw the cell blue if this is the current cursor cell
                if i == self.cursor_coordinates.row && j == self.cursor_coordinates.col {
                    let cell = Block::default().bg(Color::LightBlue);
                    frame.render_widget(cell.clone(), square);
                } else if is_getting_checked(self.board, self.player_turn, &self.move_history)
                    && Coord::new(i, j) == get_king_coordinates(self.board, self.player_turn)
                {
                    let cell = Block::default()
                        .bg(Color::Magenta)
                        .add_modifier(Modifier::SLOW_BLINK);
                    frame.render_widget(cell.clone(), square);
                }
                // Draw the cell green if this is the selected cell
                else if i == self.selected_coordinates.row && j == self.selected_coordinates.col {
                    let cell = Block::default().bg(Color::LightGreen);
                    frame.render_widget(cell.clone(), square);
                } else {
                    let cell = Block::default().bg(cell_color);
                    frame.render_widget(cell.clone(), square);
                }

                // We check if the current king is getting checked

                // Get piece and color
                let piece_color = get_piece_color(self.board, &Coord::new(i, j));
                let piece_type = get_piece_type(self.board, &Coord::new(i, j));

                let color_enum = color_to_ratatui_enum(piece_color);
                let piece_enum = PieceType::piece_type_to_string_enum(piece_type);

                // Place the pieces on the board
                let paragraph = Paragraph::new(piece_enum)
                    .alignment(Alignment::Center)
                    .fg(color_enum);
                frame.render_widget(paragraph, square);
            }
        }
    }

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
            let piece_type_from = self.move_history[i].0;
            let number_move = &self.move_history[i].1;

            let utf_icon_white =
                PieceType::piece_to_utf_enum(piece_type_from, Some(PieceColor::White));
            let move_white = convert_position_into_notation(number_move.to_string());

            let mut utf_icon_black = "   ";
            let mut move_black: String = "   ".to_string();

            // If there is something for black
            if i + 1 < self.move_history.len() {
                let piece_type_to = self.move_history[i + 1].0;
                let number = &self.move_history[i + 1].1;

                move_black = convert_position_into_notation(number.to_string());
                utf_icon_black =
                    PieceType::piece_to_utf_enum(piece_type_to, Some(PieceColor::Black))
            }

            lines.push(Line::from(vec![
                Span::raw(format!("{}.  ", i / 2 + 1)), // line number
                Span::styled(format!("{} ", utf_icon_white), Style::default().fg(WHITE)), // white symbol
                Span::raw(move_white.to_string()), // white move
                Span::raw("     "),                // separator
                Span::styled(format!("{} ", utf_icon_black), Style::default().fg(WHITE)), // white symbol
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

        // Bottom paragraph help text
        let text = vec![Line::from("Press ? for help").alignment(Alignment::Center)];

        let help_paragraph = Paragraph::new(text)
            .block(Block::new())
            .alignment(Alignment::Center);
        frame.render_widget(help_paragraph, right_panel_layout[1]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, Coord},
        pieces::{PieceColor, PieceType},
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
            vec![(Some(PieceType::Pawn), "7363".to_string())],
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
            vec![(Some(PieceType::Pawn), "1404".to_string())],
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
            vec![(Some(PieceType::Pawn), "6474".to_string())],
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
        board.move_piece_on_the_board(&Coord::new(6, 5), &Coord::new(7, 5));
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
        board.move_piece_on_the_board(&Coord::new(0, 6), &Coord::new(0, 5));
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
                (Some(PieceType::King), "0201".to_string()),
                (Some(PieceType::King), "0605".to_string()),
                (Some(PieceType::King), "0102".to_string()),
                (Some(PieceType::King), "0506".to_string()),
                (Some(PieceType::King), "0201".to_string()),
                (Some(PieceType::King), "0605".to_string()),
                (Some(PieceType::King), "0102".to_string()),
                (Some(PieceType::King), "0506".to_string()),
            ],
        );

        assert!(!board.is_draw());

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
            vec![(Some(PieceType::Pawn), "6242".to_string())],
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
}
