use super::{
    game::Game,
};
use crate::{
    pieces::{pawn::Pawn, PieceMove, PieceType},
    utils::col_to_letter,
};
use shakmaty::Color;

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
/// only the pure gameboard, no additional information
///
#[derive(Debug, Clone)]
pub struct GameBoard {
    // the 8x8 board
    pub board: Board,
    // historic of the past Moves of the board
    pub move_history: Vec<PieceMove>,
    // historic of the past gameboards states
    pub board_history: Vec<Board>,
    // the number of consecutive non pawn or capture moves
    consecutive_non_pawn_or_capture: i32,
    // The white piece that got taken
    pub white_taken_pieces: Vec<PieceType>,
    // The black piece that got taken
    pub black_taken_pieces: Vec<PieceType>,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            board: init_board(),
            move_history: vec![],
            board_history: vec![init_board()],
            consecutive_non_pawn_or_capture: 0,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }
}

impl GameBoard {
    pub fn new(board: Board, move_history: Vec<PieceMove>, board_history: Vec<Board>) -> Self {
        Self {
            board,
            move_history,
            board_history,
            consecutive_non_pawn_or_capture: 0,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }

    pub fn get_last_move_piece_type_as_string(&self) -> String {
        if let Some(last_move) = self.move_history.last() {
            match last_move.piece_type {
                PieceType::Pawn => return String::from("p"),
                PieceType::Rook => return String::from("r"),
                PieceType::Knight => return String::from("n"),
                PieceType::Bishop => return String::from("b"),
                PieceType::Queen => return String::from("q"),
                PieceType::King => return String::from("k"),
            }
        }
        String::from("")
    }

    pub fn increment_consecutive_non_pawn_or_capture(
        &mut self,
        piece_type_from: PieceType,
        piece_type_to: Option<PieceType>,
    ) {
        match (piece_type_from, piece_type_to) {
            (PieceType::Pawn, _) | (_, Some(_)) => {
                self.set_consecutive_non_pawn_or_capture(0);
            }
            _ => {
                let value = self.get_consecutive_non_pawn_or_capture() + 1;
                self.set_consecutive_non_pawn_or_capture(value);
            }
        }
    }

    pub fn add_piece_to_taken_pieces(&mut self, from: &Coord, to: &Coord, player_turn: Color) {
        if self.is_latest_move_en_passant(from, to) {
            self.push_to_taken_piece(PieceType::Pawn, player_turn.other());
        }

        let piece_type_to = self.get_piece_type(to);
        let piece_color = self.get_piece_color(to);
        // We check if there is a piece and we are not doing a castle
        if piece_color.is_some()
            && piece_type_to.is_some()
            && (piece_type_to != Some(PieceType::Rook) && piece_color != Some(player_turn))
        {
            if let Some(piece_type) = piece_type_to {
                self.push_to_taken_piece(piece_type, piece_color.unwrap())
            }
        }
    }

    pub fn push_to_taken_piece(&mut self, piece_type: PieceType, piece_color: Color) {
        match piece_color {
            Color::Black => {
                self.white_taken_pieces.push(piece_type);
                self.white_taken_pieces.sort();
            }
            Color::White => {
                self.black_taken_pieces.push(piece_type);
                self.black_taken_pieces.sort();
            }
        }
    }

    pub fn reset(&mut self) {
        self.board = init_board();
        self.move_history.clear();
        self.board_history.clear();
        self.board_history.push(init_board());
        self.consecutive_non_pawn_or_capture = 0;
    }

    // Check if the latest move is en passant
    pub fn is_latest_move_en_passant(&self, from: &Coord, to: &Coord) -> bool {
        let piece_type_from = self.get_piece_type(from);
        let piece_type_to = self.get_piece_type(to);

        let from_y: i32 = from.row as i32;
        let from_x: i32 = from.col as i32;
        let to_y: i32 = to.row as i32;
        let to_x: i32 = to.col as i32;
        match (piece_type_from, piece_type_to) {
            (Some(PieceType::Pawn), _) => {
                // Check if it's a diagonal move, and the destination is an empty cell
                from_y != to_y && from_x != to_x && self.board[to].is_none()
            }
            _ => false,
        }
    }

    // Check if the latest move is a promotion
    pub fn is_latest_move_promotion(&self) -> bool {
        if let Some(last_move) = self.move_history.last() {
            if let Some(piece_type_to) =
                self.get_piece_type(&Coord::new(last_move.to.row, last_move.to.col))
            {
                let last_row = 0;
                if last_move.to.row == last_row && piece_type_to == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }

    // Check if the game is a draw
    pub fn is_draw_by_repetition(&mut self) -> bool {
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
    pub fn is_draw(&mut self, player_turn: Color) -> bool {
        self.number_of_authorized_positions(player_turn) == 0
            || self.consecutive_non_pawn_or_capture == 50
            || self.is_draw_by_repetition()
    }

    pub fn set_consecutive_non_pawn_or_capture(&mut self, value: i32) {
        self.consecutive_non_pawn_or_capture = value;
    }

    pub fn get_consecutive_non_pawn_or_capture(&self) -> i32 {
        self.consecutive_non_pawn_or_capture
    }
}
