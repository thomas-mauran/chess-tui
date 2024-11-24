use super::coord::Coord;

pub struct UI {
    // the cursor position
    pub cursor_coordinates: Coord,
    // the selected cell
    pub selected_coordinates: Coord,
    // the selected piece cursor when we already selected a piece
    pub selected_piece_cursor: i8,
    // the old cursor position used when unslecting a cell
    pub old_cursor_position: Coord,
    // coordinates of the interactable part of the screen (either normal chess board or promotion screen)
    pub top_x: u16,
    pub top_y: u16,
    // dimension of a selectable cell (either 1 of the 64 cells, or 1 of the 4 promotion options)
    pub width: u16,
    pub height: u16,
}

impl Default for UI {
    fn default() -> Self {
        UI {
            top_x: 0,
            top_y: 0,
            width: 0,
            height: 0,
            cursor_coordinates: Coord::new(4, 4),
            selected_coordinates: Coord::undefined(),
            selected_piece_cursor: 0,
            old_cursor_position: Coord::undefined(),
        }
    }
}
