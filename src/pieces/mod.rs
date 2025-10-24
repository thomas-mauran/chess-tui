pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::constants::DisplayMode;
use crate::game_logic::coord::Coord;
use shakmaty::{Color, Role};

/// Piece type wrapper around shakmaty::Role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    /// Convert to shakmaty Role
    pub fn to_role(&self) -> Role {
        match self {
            PieceType::King => Role::King,
            PieceType::Queen => Role::Queen,
            PieceType::Rook => Role::Rook,
            PieceType::Bishop => Role::Bishop,
            PieceType::Knight => Role::Knight,
            PieceType::Pawn => Role::Pawn,
        }
    }

    /// Convert from shakmaty Role
    pub fn from_role(role: Role) -> Self {
        match role {
            Role::King => PieceType::King,
            Role::Queen => PieceType::Queen,
            Role::Rook => PieceType::Rook,
            Role::Bishop => PieceType::Bishop,
            Role::Knight => PieceType::Knight,
            Role::Pawn => PieceType::Pawn,
        }
    }

    /// Convert piece type to UTF-8 character
    pub fn piece_to_utf_enum(piece_type: &PieceType, color: Option<Color>) -> &'static str {
        match color {
            Some(Color::White) => match piece_type {
                PieceType::King => "♔",
                PieceType::Queen => "♕",
                PieceType::Rook => "♖",
                PieceType::Bishop => "♗",
                PieceType::Knight => "♘",
                PieceType::Pawn => "♙",
            },
            Some(Color::Black) => match piece_type {
                PieceType::King => "♚",
                PieceType::Queen => "♛",
                PieceType::Rook => "♜",
                PieceType::Bishop => "♝",
                PieceType::Knight => "♞",
                PieceType::Pawn => "♟",
            },
            None => " ",
        }
    }

    /// Convert piece type to string based on display mode
    /// Note: This is used for the board grid. For multi-line designs, see individual piece modules.
    pub fn piece_type_to_string_enum(
        piece_type: Option<PieceType>,
        display_mode: &DisplayMode,
    ) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => match piece_type {
                // Custom single-character designs using Unicode box drawing and symbols
                Some(PieceType::King) => "♚",   // King with cross
                Some(PieceType::Queen) => "♛",  // Queen with multiple points
                Some(PieceType::Rook) => "♜",   // Rook/Castle tower
                Some(PieceType::Bishop) => "♝", // Bishop with pointed top
                Some(PieceType::Knight) => "♞", // Knight/Horse head
                Some(PieceType::Pawn) => "♟",   // Simple pawn
                None => " ",
            },
            DisplayMode::ASCII => match piece_type {
                Some(PieceType::King) => "K",
                Some(PieceType::Queen) => "Q",
                Some(PieceType::Rook) => "R",
                Some(PieceType::Bishop) => "B",
                Some(PieceType::Knight) => "N",
                Some(PieceType::Pawn) => "P",
                None => " ",
            },
        }
    }
}

/// A move of a piece on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PieceMove {
    pub piece_type: PieceType,
    pub piece_color: Color,
    pub from: Coord,
    pub to: Coord,
}

impl PieceMove {
    pub fn new(piece_type: PieceType, piece_color: Color, from: Coord, to: Coord) -> Self {
        Self {
            piece_type,
            piece_color,
            from,
            to,
        }
    }
}
