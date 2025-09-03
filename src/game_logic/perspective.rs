use super::coord::Coord;
use crate::pieces::{PieceColor, PieceMove};

/// Represents the perspective from which the board is viewed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerPerspective {
    /// White pieces at bottom (traditional view) - board coordinates match display coordinates
    White,
    /// Black pieces at bottom (flipped view) - board coordinates need transformation for display
    Black,
}

impl PlayerPerspective {
    /// Get the perspective for a given player color
    pub fn for_player(player_color: PieceColor) -> Self {
        match player_color {
            PieceColor::White => PlayerPerspective::White,
            PieceColor::Black => PlayerPerspective::Black,
        }
    }

    /// Get the opposite perspective
    pub fn opposite(self) -> Self {
        match self {
            PlayerPerspective::White => PlayerPerspective::Black,
            PlayerPerspective::Black => PlayerPerspective::White,
        }
    }
}

/// Manages the current perspective and provides coordinate transformations
#[derive(Clone, Debug)]
pub struct PerspectiveManager {
    /// The current perspective from which the board is viewed
    pub current_perspective: PlayerPerspective,
    /// The color of the human player (used to determine default perspective)
    pub player_color: PieceColor,
}

impl PerspectiveManager {
    /// Create a new perspective manager
    pub fn new(player_color: PieceColor) -> Self {
        Self {
            current_perspective: PlayerPerspective::for_player(player_color),
            player_color,
        }
    }

    /// Create a perspective manager with white perspective (for local games)
    pub fn white_perspective() -> Self {
        Self {
            current_perspective: PlayerPerspective::White,
            player_color: PieceColor::White,
        }
    }

    /// Set the perspective based on player color
    pub fn set_perspective_for_player(&mut self, player_color: PieceColor) {
        self.player_color = player_color;
        self.current_perspective = PlayerPerspective::for_player(player_color);
    }

    /// Toggle the perspective (useful for local games where players alternate views)
    pub fn toggle_perspective(&mut self) {
        self.current_perspective = self.current_perspective.opposite();
    }

    /// Check if we need to transform coordinates for display
    pub fn needs_transformation(&self) -> bool {
        self.current_perspective == PlayerPerspective::Black
    }

    /// Transform a coordinate from logical board position to display position
    pub fn logical_to_display(&self, coord: Coord) -> Coord {
        if !coord.is_valid() {
            return coord;
        }
        if self.needs_transformation() {
            Coord::new(7 - coord.row, 7 - coord.col)
        } else {
            coord
        }
    }

    /// Transform a coordinate from display position to logical board position
    pub fn display_to_logical(&self, coord: Coord) -> Coord {
        if !coord.is_valid() {
            return coord;
        }
        if self.needs_transformation() {
            Coord::new(7 - coord.row, 7 - coord.col)
        } else {
            coord
        }
    }

    /// Transform a move from logical coordinates to display coordinates
    pub fn transform_move_for_display(&self, piece_move: &PieceMove) -> PieceMove {
        if self.needs_transformation() {
            PieceMove {
                piece_type: piece_move.piece_type,
                piece_color: piece_move.piece_color,
                from: self.logical_to_display(piece_move.from),
                to: self.logical_to_display(piece_move.to),
            }
        } else {
            *piece_move
        }
    }

    /// Transform a move from display coordinates to logical coordinates
    pub fn transform_move_for_logic(&self, piece_move: &PieceMove) -> PieceMove {
        if self.needs_transformation() {
            PieceMove {
                piece_type: piece_move.piece_type,
                piece_color: piece_move.piece_color,
                from: self.display_to_logical(piece_move.from),
                to: self.display_to_logical(piece_move.to),
            }
        } else {
            *piece_move
        }
    }

    /// Get the perspective that should be used for a given game mode
    pub fn get_mode_perspective(
        player_color: PieceColor,
        is_bot_game: bool,
        is_multiplayer: bool,
    ) -> PlayerPerspective {
        if is_multiplayer {
            // In multiplayer, use the player's color perspective
            PlayerPerspective::for_player(player_color)
        } else if is_bot_game {
            // In bot games, use the player's color perspective
            PlayerPerspective::for_player(player_color)
        } else {
            // In local games, traditionally start with white perspective
            // but can be toggled
            PlayerPerspective::White
        }
    }
}

impl Default for PerspectiveManager {
    fn default() -> Self {
        Self::white_perspective()
    }
}
