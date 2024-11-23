use super::board::{init_board, Board};
use crate::pieces::PieceMove;

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
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            board: init_board(),
            move_history: vec![],
            board_history: vec![init_board()],
        }
    }
}

impl GameBoard {
    pub fn new(board: Board, move_history: Vec<PieceMove>, board_history: Vec<Board>) -> Self {
        Self {
            board,
            move_history,
            board_history,
        }
    }
}
