use super::{
    board::{init_board, Board},
    coord::Coord,
};
use crate::{
    pieces::{PieceColor, PieceMove, PieceType},
    utils::{get_piece_type, is_getting_checked},
};

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
pub struct GameBoard {
    // the 8x8 board
    pub board: Board,
    // historic of the past Moves of the board
    pub move_history: Vec<PieceMove>,
    // historic of the past gameboards states
    pub board_history: Vec<Board>,
    // the number of consecutive non pawn or capture moves
    consecutive_non_pawn_or_capture: i32,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            board: init_board(),
            move_history: vec![],
            board_history: vec![init_board()],
            consecutive_non_pawn_or_capture: 0,
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
        }
    }

    pub fn get_authorized_positions(
        &self,
        player_turn: PieceColor,
        coordinates: Coord,
    ) -> Vec<Coord> {
        if let Some((piece_type, piece_color)) =
            self.board[coordinates.row as usize][coordinates.col as usize]
        {
            return piece_type.authorized_positions(
                &coordinates,
                piece_color,
                self.board,
                &self.move_history,
                is_getting_checked(self.board, player_turn, &self.move_history),
            );
        } else {
            return vec![];
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

    // Check if the latest move is en passant
    pub fn is_latest_move_en_passant(&self, from: Coord, to: Coord) -> bool {
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
    pub fn is_latest_move_castling(&self, from: Coord, to: Coord) -> bool {
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
    pub fn is_latest_move_promotion(&self) -> bool {
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

    // Method to get the number of authorized positions for the current player (used for the end condition)
    pub fn number_of_authorized_positions(&self, player_turn: PieceColor) -> usize {
        let mut possible_moves: Vec<Coord> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                let coord = Coord::new(i, j);
                if let Some((piece_type, piece_color)) = self.board[&coord] {
                    if piece_color == player_turn {
                        possible_moves.extend(self.get_authorized_positions(player_turn, coord));
                    }
                }
            }
        }
        possible_moves.len()
    }

    // Check if the game is checkmate
    pub fn is_checkmate(&self, player_turn: PieceColor) -> bool {
        if !is_getting_checked(self.board, player_turn, &self.move_history) {
            return false;
        }

        self.number_of_authorized_positions(player_turn) == 0
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
    pub fn is_draw(&mut self, player_turn: PieceColor) -> bool {
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
