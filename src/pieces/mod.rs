pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::constants::DisplayMode;
use shakmaty::{Color, Role};

/// Convert piece type to UTF-8 character
pub fn role_to_utf_enum(role: &Role, color: Option<Color>) -> &'static str {
    match color {
        Some(Color::White) => match role {
            Role::King => "♔",
            Role::Queen => "♕",
            Role::Rook => "♖",
            Role::Bishop => "♗",
            Role::Knight => "♘",
            Role::Pawn => "♙",
        },
        Some(Color::Black) => match role {
            Role::King => "♚",
            Role::Queen => "♛",
            Role::Rook => "♜",
            Role::Bishop => "♝",
            Role::Knight => "♞",
            Role::Pawn => "♟",
        },
        None => " ",
    }
}

/// Convert piece type to string based on display mode
/// Note: This is used for the board grid. For multi-line designs, see individual piece modules.
pub fn role_to_string_enum(role: Option<Role>, display_mode: &DisplayMode) -> &'static str {
    match display_mode {
        DisplayMode::DEFAULT | DisplayMode::CUSTOM => match role {
            // Custom single-character designs using Unicode box drawing and symbols
            Some(Role::King) => "♚",   // King with cross
            Some(Role::Queen) => "♛",  // Queen with multiple points
            Some(Role::Rook) => "♜",   // Rook/Castle tower
            Some(Role::Bishop) => "♝", // Bishop with pointed top
            Some(Role::Knight) => "♞", // Knight/Horse head
            Some(Role::Pawn) => "♟",   // Simple pawn
            None => " ",
        },

        DisplayMode::ASCII => match role {
            Some(Role::King) => "K",
            Some(Role::Queen) => "Q",
            Some(Role::Rook) => "R",
            Some(Role::Bishop) => "B",
            Some(Role::Knight) => "N",
            Some(Role::Pawn) => "P",
            None => " ",
        },
    }
}
