use crate::board::Coords;

use self::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

impl PieceType {
    pub fn authorized_positions(
        self,
        coordinates: &Coords,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[(Option<PieceType>, String)],
        is_king_checked: bool,
    ) -> Vec<Coords> {
        match self {
            PieceType::Pawn => {
                Pawn::authorized_positions(coordinates, color, board, move_history, is_king_checked)
            }
            PieceType::Rook => {
                Rook::authorized_positions(coordinates, color, board, move_history, is_king_checked)
            }
            PieceType::Bishop => Bishop::authorized_positions(
                coordinates,
                color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceType::Queen => Queen::authorized_positions(
                coordinates,
                color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceType::King => {
                King::authorized_positions(coordinates, color, board, move_history, is_king_checked)
            }
            PieceType::Knight => Knight::authorized_positions(
                coordinates,
                color,
                board,
                move_history,
                is_king_checked,
            ),
        }
    }

    pub fn protected_positions(
        selected_coordinates: &Coords,
        piece_type: PieceType,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[(Option<PieceType>, String)],
    ) -> Vec<Coords> {
        match piece_type {
            PieceType::Pawn => {
                Pawn::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Rook => {
                Rook::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Bishop => {
                Bishop::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Queen => {
                Queen::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::King => {
                King::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Knight => {
                Knight::protected_positions(selected_coordinates, color, board, move_history)
            }
        }
    }

    pub fn piece_to_utf_enum(
        piece_type: Option<PieceType>,
        piece_color: Option<PieceColor>,
    ) -> &'static str {
        match (piece_type, piece_color) {
            (Some(PieceType::Queen), Some(PieceColor::Black)) => "♕",
            (Some(PieceType::Queen), Some(PieceColor::White)) => "♛",
            (Some(PieceType::King), Some(PieceColor::Black)) => "♔",
            (Some(PieceType::King), Some(PieceColor::White)) => "♚",
            (Some(PieceType::Rook), Some(PieceColor::Black)) => "♖",
            (Some(PieceType::Rook), Some(PieceColor::White)) => "♜",
            (Some(PieceType::Bishop), Some(PieceColor::Black)) => "♗",
            (Some(PieceType::Bishop), Some(PieceColor::White)) => "♝",
            (Some(PieceType::Knight), Some(PieceColor::Black)) => "♘",
            (Some(PieceType::Knight), Some(PieceColor::White)) => "♞",
            (Some(PieceType::Pawn), Some(PieceColor::Black)) => "♙",
            (Some(PieceType::Pawn), Some(PieceColor::White)) => "♟",
            _ => "NONE",
        }
    }

    pub fn piece_to_fen_enum(
        piece_type: Option<PieceType>,
        piece_color: Option<PieceColor>,
    ) -> &'static str {
        match (piece_type, piece_color) {
            (Some(PieceType::Queen), Some(PieceColor::Black)) => "q",
            (Some(PieceType::Queen), Some(PieceColor::White)) => "Q",
            (Some(PieceType::King), Some(PieceColor::Black)) => "k",
            (Some(PieceType::King), Some(PieceColor::White)) => "K",
            (Some(PieceType::Rook), Some(PieceColor::Black)) => "r",
            (Some(PieceType::Rook), Some(PieceColor::White)) => "R",
            (Some(PieceType::Bishop), Some(PieceColor::Black)) => "b",
            (Some(PieceType::Bishop), Some(PieceColor::White)) => "B",
            (Some(PieceType::Knight), Some(PieceColor::Black)) => "n",
            (Some(PieceType::Knight), Some(PieceColor::White)) => "N",
            (Some(PieceType::Pawn), Some(PieceColor::Black)) => "p",
            (Some(PieceType::Pawn), Some(PieceColor::White)) => "P",
            (None, None) => "",
            _ => unreachable!("Undefined piece and piece color tuple"),
        }
    }

    pub fn piece_type_to_string_enum(piece_type: Option<PieceType>) -> &'static str {
        match piece_type {
            Some(PieceType::Queen) => Queen::to_string(),
            Some(PieceType::King) => King::to_string(),
            Some(PieceType::Rook) => Rook::to_string(),
            Some(PieceType::Bishop) => Bishop::to_string(),
            Some(PieceType::Knight) => Knight::to_string(),
            Some(PieceType::Pawn) => Pawn::to_string(),
            None => " ",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

pub trait Movable {
    fn piece_move(
        coordinates: &Coords,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        move_history: &[(Option<PieceType>, String)],
    ) -> Vec<Coords>;
}

pub trait Position {
    fn authorized_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[(Option<PieceType>, String)],
        is_king_checked: bool,
    ) -> Vec<Coords>;

    fn protected_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[(Option<PieceType>, String)],
    ) -> Vec<Coords>;
}
