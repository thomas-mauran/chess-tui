//! Board skin and piece style types.

use crate::app::AppResult;
use crate::constants::{DisplayMode, config_dir};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, Write};
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

/// JSON top-level wrapper for a list of [`Skin`] entries in `skins.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinCollection {
    pub skins: Vec<Skin>,
}

/// JSON top-level wrapper for a list of [`PieceStyle`] entries.
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

    /// Writes the built-in `default_skins.json` to `skins_path`, creating parent dirs as needed.
    pub fn create_default_skins_file(skins_path: &Path) -> AppResult<()> {
        // Ensure the directory exists
        if let Some(parent) = skins_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Default skins.json content (embedded at compile time)
        const DEFAULT_SKINS: &str = include_str!("default_skins.json");

        let mut file = File::create(skins_path)?;
        file.write_all(DEFAULT_SKINS.as_bytes())?;

        Ok(())
    }

    #[must_use]
    pub fn get_skin_by_name(skins: &[Skin], name: &str) -> Option<Skin> {
        skins.iter().find(|s| s.name == name).cloned()
    }

    /// Loads all piece styles from a JSON file containing a [`PieceStyleCollection`].
    pub fn load_all_piece_styles<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<PieceStyle>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let collection: PieceStyleCollection = serde_json::from_str(&content)?;
        Ok(collection.piece_styles)
    }

    /// Runs the --update-skins command: prompt for confirmation, archive current skins.json, write default.
    pub fn run_update_skins() -> AppResult<()> {
        const DEFAULT_SKINS: &str = include_str!("default_skins.json");

        let config_dir = config_dir()?;
        let skins_dir = config_dir.join("chess-tui");
        let skins_path = skins_dir.join("skins.json");

        fs::create_dir_all(&skins_dir).map_err(|e| e.to_string())?;

        if !skins_path.exists() {
            let mut file = File::create(&skins_path).map_err(|e| e.to_string())?;
            file.write_all(DEFAULT_SKINS.as_bytes())
                .map_err(|e| e.to_string())?;
            println!(
                "Created skins.json with default content at {}",
                skins_path.display()
            );
            return Ok(());
        }

        print!(
            "This will replace your skins config with the default. \
         Your current file will be archived in the same folder. Continue? (y/n): "
        );
        std::io::stdout().flush().map_err(|e| e.to_string())?;

        let mut line = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut line)
            .map_err(|e| e.to_string())?;
        let answer = line.trim().to_lowercase();

        if answer != "y" && answer != "yes" {
            println!("Aborted.");
            return Ok(());
        }

        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
        let archive_name = format!("skins_{}.json", timestamp);
        let archive_path = skins_dir.join(&archive_name);

        fs::copy(&skins_path, &archive_path).map_err(|e| e.to_string())?;
        let mut file = File::create(&skins_path).map_err(|e| e.to_string())?;
        file.write_all(DEFAULT_SKINS.as_bytes())
            .map_err(|e| e.to_string())?;

        println!(
            "Archived previous config to {} and updated skins.json with default.",
            archive_path.display()
        );
        Ok(())
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
