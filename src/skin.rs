use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skin {
    pub name: String,
    pub board_white_color: Color,
    pub board_black_color: Color,
    pub piece_white_color: Color,
    pub piece_black_color: Color,
    pub cursor_color: Color,
    pub selection_color: Color,
    pub last_move_color: Color,
}

impl Default for Skin {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
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

impl Skin {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let skin: Skin = serde_json::from_str(&content)?;
        Ok(skin)
    }

    pub fn load_all_skins<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<Skin>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let collection: SkinCollection = serde_json::from_str(&content)?;
        Ok(collection.skins)
    }

    pub fn get_skin_by_name(skins: &[Skin], name: &str) -> Option<Skin> {
        skins.iter().find(|s| s.name == name).cloned()
    }

    /// Creates a special "Default" display mode skin entry
    pub fn default_display_mode() -> Self {
        Self {
            name: "Default".to_string(),
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
    pub fn ascii_display_mode() -> Self {
        Self {
            name: "ASCII".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_skin() {
        let skin = Skin::default();
        assert_eq!(skin.name, "Default");
        assert_eq!(skin.board_white_color, Color::Rgb(160, 160, 160));
        assert_eq!(skin.piece_white_color, Color::White);
    }

    #[test]
    fn test_load_skin() {
        let json = r#"{
            "name": "Test Skin",
            "board_white_color": "Red",
            "board_black_color": "Blue",
            "piece_white_color": "Green",
            "piece_black_color": "Yellow",
            "cursor_color": "LightBlue",
            "selection_color": "LightGreen",
            "last_move_color": "LightGreen"
        }"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", json).unwrap();

        let skin = Skin::load_from_file(file.path()).unwrap();

        assert_eq!(skin.name, "Test Skin");
        assert_eq!(skin.board_white_color, Color::Red);
        assert_eq!(skin.board_black_color, Color::Blue);
        assert_eq!(skin.piece_white_color, Color::Green);
    }
}
