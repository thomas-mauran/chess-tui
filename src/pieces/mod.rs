use self::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};
use super::constants::DisplayMode;
use crate::game_logic::{coord::Coord, game_board::GameBoard};
use shakmaty::Color;
use std::cmp::Ordering;

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
        color: Color,
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

    /// The cells a given piece is protecting
    pub fn protected_positions(
        selected_coordinates: &Coord,
        piece_type: PieceType,
        color: Color,
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
    pub fn piece_to_utf_enum(piece_type: &PieceType, piece_color: Option<Color>) -> &'static str {
        match (piece_type, piece_color) {
            (PieceType::Queen, Some(Color::Black)) => "♕",
            (PieceType::Queen, Some(Color::White)) => "♛",
            (PieceType::King, Some(Color::Black)) => "♔",
            (PieceType::King, Some(Color::White)) => "♚",
            (PieceType::Rook, Some(Color::Black)) => "♖",
            (PieceType::Rook, Some(Color::White)) => "♜",
            (PieceType::Bishop, Some(Color::Black)) => "♗",
            (PieceType::Bishop, Some(Color::White)) => "♝",
            (PieceType::Knight, Some(Color::Black)) => "♘",
            (PieceType::Knight, Some(Color::White)) => "♞",
            (PieceType::Pawn, Some(Color::Black)) => "♙",
            (PieceType::Pawn, Some(Color::White)) => "♟",
            _ => "NONE",
        }
    }

    /// Convert a PieceType fo a conform fen character
    pub fn piece_to_fen_enum(
        piece_type: Option<PieceType>,
        piece_color: Option<Color>,
    ) -> &'static str {
        match (piece_type, piece_color) {
            (Some(PieceType::Queen), Some(Color::Black)) => "q",
            (Some(PieceType::Queen), Some(Color::White)) => "Q",
            (Some(PieceType::King), Some(Color::Black)) => "k",
            (Some(PieceType::King), Some(Color::White)) => "K",
            (Some(PieceType::Rook), Some(Color::Black)) => "r",
            (Some(PieceType::Rook), Some(Color::White)) => "R",
            (Some(PieceType::Bishop), Some(Color::Black)) => "b",
            (Some(PieceType::Bishop), Some(Color::White)) => "B",
            (Some(PieceType::Knight), Some(Color::Black)) => "n",
            (Some(PieceType::Knight), Some(Color::White)) => "N",
            (Some(PieceType::Pawn), Some(Color::Black)) => "p",
            (Some(PieceType::Pawn), Some(Color::White)) => "P",
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
    pub piece_color: Color,
    pub from: Coord,
    pub to: Coord,
}

pub trait Movable {
    fn piece_move(
        coordinates: &Coord,
        color: Color,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
    ) -> Vec<Coord>;
}

pub trait Position {
    fn authorized_positions(
        coordinates: &Coord,
        color: Color,
        game_board: &GameBoard,
        is_king_checked: bool,
    ) -> Vec<Coord>;

    fn protected_positions(coordinates: &Coord, color: Color, game_board: &GameBoard)
        -> Vec<Coord>;
}
