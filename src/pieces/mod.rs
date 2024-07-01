use crate::board::{Coord, GameBoard};

use self::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};
use super::constants::DisplayMode;

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
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
        is_king_checked: bool,
    ) -> Vec<Coord> {
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
        selected_coordinates: &Coord,
        piece_type: PieceType,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
    ) -> Vec<Coord> {
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
        piece_type: PieceType,
        piece_color: Option<PieceColor>,
    ) -> &'static str {
        match (piece_type, piece_color) {
            (PieceType::Queen, Some(PieceColor::Black)) => "♕",
            (PieceType::Queen, Some(PieceColor::White)) => "♛",
            (PieceType::King, Some(PieceColor::Black)) => "♔",
            (PieceType::King, Some(PieceColor::White)) => "♚",
            (PieceType::Rook, Some(PieceColor::Black)) => "♖",
            (PieceType::Rook, Some(PieceColor::White)) => "♜",
            (PieceType::Bishop, Some(PieceColor::Black)) => "♗",
            (PieceType::Bishop, Some(PieceColor::White)) => "♝",
            (PieceType::Knight, Some(PieceColor::Black)) => "♘",
            (PieceType::Knight, Some(PieceColor::White)) => "♞",
            (PieceType::Pawn, Some(PieceColor::Black)) => "♙",
            (PieceType::Pawn, Some(PieceColor::White)) => "♟",
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

    pub fn piece_type_to_string_enum(
        piece_type: Option<PieceType>,
        display_mode: &DisplayMode,
    ) -> &'static str {
        match piece_type {
            Some(PieceType::Queen) => Queen::to_string(display_mode),
            Some(PieceType::King) => King::to_string(display_mode),
            Some(PieceType::Rook) => Rook::to_string(display_mode),
            Some(PieceType::Bishop) => Bishop::to_string(display_mode),
            Some(PieceType::Knight) => Knight::to_string(display_mode),
            Some(PieceType::Pawn) => Pawn::to_string(display_mode),
            None => " ",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PieceMove {
    pub piece_type: PieceType,
    pub from: Coord,
    pub to: Coord,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

pub trait Movable {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        move_history: &[PieceMove],
    ) -> Vec<Coord>;
}

pub trait Position {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
        is_king_checked: bool,
    ) -> Vec<Coord>;

    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
    ) -> Vec<Coord>;
}
