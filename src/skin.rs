use crate::constants::DisplayMode;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

fn default_piece_style_str() -> String {
    DisplayMode::DEFAULT.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skin {
    pub name: String,
    #[serde(default = "default_piece_style_str", alias = "display_mode")]
    pub piece_style: String,
    pub board_white_color: Color,
    pub board_black_color: Color,
    pub piece_white_color: Color,
    pub piece_black_color: Color,
    pub cursor_color: Color,
    pub selection_color: Color,
    pub last_move_color: Color,
}

/// Piece characters for a single size (small / compact / extended / large).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PieceStyleSize {
    pub bishop: String,
    pub king: String,
    pub knight: String,
    pub pawn: String,
    pub queen: String,
    pub rook: String,
}

/// Named piece style with different character sets per size.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PieceStyle {
    pub name: String,
    pub small: PieceStyleSize,
    pub compact: PieceStyleSize,
    pub extended: PieceStyleSize,
    pub large: PieceStyleSize,
}

impl Default for Skin {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            piece_style: DisplayMode::DEFAULT.to_string(),
            board_white_color: Color::Rgb(160, 160, 160),
            board_black_color: Color::Rgb(128, 95, 69),
            piece_white_color: Color::White,
            piece_black_color: Color::Black,
            cursor_color: Color::LightBlue,
            selection_color: Color::LightGreen,
            last_move_color: Color::LightGreen,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinCollection {
    pub skins: Vec<Skin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PieceStyleCollection {
    pub piece_styles: Vec<PieceStyle>,
}

impl Skin {
    /// Loads a skin from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let skin: Skin = serde_json::from_str(&content)?;
        Ok(skin)
    }

    /// Loads all skins from a JSON file containing a collection.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_all_skins<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<Skin>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let collection: SkinCollection = serde_json::from_str(&content)?;
        Ok(collection.skins)
    }

    #[must_use]
    pub fn get_skin_by_name(skins: &[Skin], name: &str) -> Option<Skin> {
        skins.iter().find(|s| s.name == name).cloned()
    }

    pub fn load_all_piece_styles<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<PieceStyle>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let collection: PieceStyleCollection = serde_json::from_str(&content)?;
        Ok(collection.piece_styles)
    }

    /// Creates a special "Default" display mode skin entry
    #[must_use]
    pub fn default_display_mode() -> Self {
        Self {
            name: "Default".to_string(),
            piece_style: DisplayMode::DEFAULT.to_string(),
            board_white_color: Color::Rgb(160, 160, 160),
            board_black_color: Color::Rgb(128, 95, 69),
            piece_white_color: Color::White,
            piece_black_color: Color::Black,
            cursor_color: Color::LightBlue,
            selection_color: Color::LightGreen,
            last_move_color: Color::LightGreen,
        }
    }

    /// Creates a special "ASCII" display mode skin entry
    #[must_use]
    pub fn ascii_display_mode() -> Self {
        Self {
            name: "ASCII".to_string(),
            piece_style: DisplayMode::ASCII.to_string(),
            board_white_color: Color::Rgb(160, 160, 160),
            board_black_color: Color::Rgb(128, 95, 69),
            piece_white_color: Color::White,
            piece_black_color: Color::Black,
            cursor_color: Color::LightBlue,
            selection_color: Color::LightGreen,
            last_move_color: Color::LightGreen,
        }
    }
}
