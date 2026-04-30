//! Holds the loaded [`Skin`] list, the active skin index, and the current display mode.

use crate::skin::{PieceStyle, Skin};

/// Skin and piece-style state held by [`crate::app::App`].
///
/// `available_skins` is populated at startup from `skins.json`. The active
/// skin is tracked by name in `selected_skin_name` and by value in
/// `loaded_skin` so it can be cheaply applied to the board UI.
pub struct ThemeState {
    /// The loaded skin
    pub loaded_skin: Option<Skin>,
    /// Available skins loaded from skins.json
    pub available_skins: Vec<Skin>,
    /// Available Piece Styles
    pub available_piece_styles: Vec<PieceStyle>,
    /// Selected skin name
    pub selected_skin_name: String,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            loaded_skin: Some(Skin::default()),
            available_skins: Vec::new(),
            available_piece_styles: Vec::new(),
            selected_skin_name: "Default".to_string(),
        }
    }
}

impl ThemeState {
    /// Returns the next (`true`) or previous (`false`) skin relative to the
    /// currently selected one, wrapping around the available skins list.
    pub fn get_skin(&self, next: bool) -> Skin {
        if self.available_skins.is_empty() {
            return Skin::default();
        }

        // Find current skin index
        let current_index = self
            .available_skins
            .iter()
            .position(|s| s.name == self.selected_skin_name)
            .unwrap_or(0);

        match next {
            true => {
                let next_index = (current_index + 1) % self.available_skins.len();
                self.available_skins[next_index].clone()
            }
            // previous skin
            false => {
                let previous_index = if current_index == 0 {
                    self.available_skins.len() - 1
                } else {
                    current_index - 1
                };
                self.available_skins[previous_index].clone()
            }
        }
    }
}
