use std::cmp::Ordering;

use self::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};
use super::constants::DisplayMode;
use crate::game_logic::{coord::Coord, game_board::GameBoard, perspective::PerspectiveManager};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

/// The different type of pieces in the game
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

impl PieceType {
    /// The authorized position for a piece at a certain coordinate
    pub fn authorized_positions(
        self,
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        is_king_checked: bool,
    ) -> Vec<Coord> {
        match self {
            PieceType::Pawn => {
                Pawn::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::Rook => {
                Rook::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::Bishop => {
                Bishop::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::Queen => {
                Queen::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::King => {
                King::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::Knight => {
                Knight::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
        }
    }

    /// The authorized position for a piece at a certain coordinate with perspective information
    pub fn authorized_positions_with_perspective(
        self,
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        is_king_checked: bool,
        perspective: Option<&PerspectiveManager>,
    ) -> Vec<Coord> {
        match self {
            PieceType::Pawn => Pawn::authorized_positions_with_perspective(
                coordinates,
                color,
                game_board,
                is_king_checked,
                perspective,
            ),
            // For other pieces, perspective doesn't affect movement, so use the regular method
            PieceType::Rook => {
                Rook::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::Bishop => {
                Bishop::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::Queen => {
                Queen::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::King => {
                King::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
            PieceType::Knight => {
                Knight::authorized_positions(coordinates, color, game_board, is_king_checked)
            }
        }
    }

    /// The cells a given piece is protecting
    pub fn protected_positions(
        selected_coordinates: &Coord,
        piece_type: PieceType,
        color: PieceColor,
        game_board: &GameBoard,
    ) -> Vec<Coord> {
        match piece_type {
            PieceType::Pawn => Pawn::protected_positions(selected_coordinates, color, game_board),
            PieceType::Rook => Rook::protected_positions(selected_coordinates, color, game_board),
            PieceType::Bishop => {
                Bishop::protected_positions(selected_coordinates, color, game_board)
            }
            PieceType::Queen => Queen::protected_positions(selected_coordinates, color, game_board),
            PieceType::King => King::protected_positions(selected_coordinates, color, game_board),
            PieceType::Knight => {
                Knight::protected_positions(selected_coordinates, color, game_board)
            }
        }
    }

    /// Convert a PieceType to a symbol
    pub fn piece_to_utf_enum(
        piece_type: &PieceType,
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

    /// Convert a PieceType fo a conform fen character
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

/// Implementing the PartialOrd trait for PieceType to allow comparison between PieceType
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for PieceType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match (self, other) {
            (PieceType::Pawn, _) => Some(Ordering::Less),
            (PieceType::Queen, _) => Some(Ordering::Greater),
            (_, PieceType::Pawn) => Some(Ordering::Greater),
            (_, PieceType::Queen) => Some(Ordering::Less),
            (PieceType::Rook, PieceType::Bishop) => Some(Ordering::Greater),
            (PieceType::Rook, PieceType::Knight) => Some(Ordering::Greater),
            (PieceType::Bishop, PieceType::Knight) => Some(Ordering::Greater), // just for visual purpose
            (PieceType::Bishop, PieceType::Rook) => Some(Ordering::Less),
            (PieceType::Knight, PieceType::Rook) => Some(Ordering::Less),
            (PieceType::Knight, PieceType::Bishop) => Some(Ordering::Less), // just for visual purpose
            _ => Some(Ordering::Equal),
        }
    }
}
impl Ord for PieceType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for PieceType {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PieceMove {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
    pub from: Coord,
    pub to: Coord,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PieceColor {
    White = 0,
    Black = 1,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

pub trait Movable {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
    ) -> Vec<Coord>;
}

pub trait Position {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        is_king_checked: bool,
    ) -> Vec<Coord>;

    /// Perspective-aware authorized positions (default implementation calls regular method)
    fn authorized_positions_with_perspective(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        is_king_checked: bool,
        _perspective: Option<&PerspectiveManager>,
    ) -> Vec<Coord> {
        // Default implementation ignores perspective
        Self::authorized_positions(coordinates, color, game_board, is_king_checked)
    }

    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
    ) -> Vec<Coord>;
}
