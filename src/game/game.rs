use super::board::{Board, GameBoard};
use crate::pieces::{PieceColor, PieceMove};

pub struct Game {
    pub board: Board,
    pub move_history: Vec<PieceMove>,
    pub current_turn: PieceColor,
    pub is_king_checked: bool,
    pub is_king_checkmated: bool,
    pub is_stalemate: bool,
    pub is_draw: bool,
    pub is_game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            move_history: Vec::new(),
            current_turn: PieceColor::White,
            is_king_checked: false,
            is_king_checkmated: false,
            is_stalemate: false,
            is_draw: false,
            is_game_over: false,
        }
    }
}
