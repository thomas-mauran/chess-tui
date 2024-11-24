use super::{
    coord::Coord,
    game::{self, Game},
    game_board::{self, GameBoard},
};
use crate::constants::UNDEFINED_POSITION;

pub struct UI {
    /// the cursor position
    pub cursor_coordinates: Coord,
    // the selected cell
    pub selected_coordinates: Coord,
    // the selected piece cursor when we already selected a piece
    pub selected_piece_cursor: i8,
    // the cursor for the promotion popup
    pub promotion_cursor: i8,
    // the old cursor position used when unslecting a cell
    pub old_cursor_position: Coord,
    // coordinates of the interactable part of the screen (either normal chess board or promotion screen)
    pub top_x: u16,
    pub top_y: u16,
    // dimension of a selectable cell (either 1 of the 64 cells, or 1 of the 4 promotion options)
    pub width: u16,
    pub height: u16,
    // last move was with a mouse
    pub mouse_used: bool,
}

impl Default for UI {
    fn default() -> Self {
        UI {
            cursor_coordinates: Coord::new(4, 4),
            selected_coordinates: Coord::undefined(),
            selected_piece_cursor: 0,
            promotion_cursor: 0,
            old_cursor_position: Coord::undefined(),
            top_x: 0,
            top_y: 0,
            width: 0,
            height: 0,
            mouse_used: false,
        }
    }
}

impl UI {
    /// Check if a cell has been selected
    pub fn is_cell_selected(&self) -> bool {
        self.selected_coordinates.row != UNDEFINED_POSITION
            && self.selected_coordinates.col != UNDEFINED_POSITION
    }

    /* Method to move the selected piece cursor
    We make sure that the cursor is in the authorized positions
    */
    pub fn move_selected_piece_cursor(
        &mut self,
        first_time_moving: bool,
        direction: i8,
        mut authorized_positions: Vec<Coord>,
    ) {
        if authorized_positions.is_empty() {
            self.cursor_coordinates = Coord::undefined();
        } else {
            self.selected_piece_cursor = if self.selected_piece_cursor == 0 && first_time_moving {
                0
            } else {
                let new_cursor =
                    (self.selected_piece_cursor + direction) % authorized_positions.len() as i8;
                if new_cursor == -1 {
                    authorized_positions.len() as i8 - 1
                } else {
                    new_cursor
                }
            };

            authorized_positions.sort();

            if let Some(position) = authorized_positions.get(self.selected_piece_cursor as usize) {
                self.cursor_coordinates = *position;
            }
        }
    }

    /// Cursor movement methods
    pub fn cursor_up(&mut self, mut authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1, authorized_positions);
        } else if self.cursor_coordinates.row > 0 {
            self.cursor_coordinates.row -= 1;
        }
    }

    pub fn cursor_down(&mut self, mut authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, 1, authorized_positions);
        } else if self.cursor_coordinates.row < 7 {
            self.cursor_coordinates.row += 1;
        }
    }
    pub fn cursor_left(&mut self, mut authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1, authorized_positions);
        } else if self.cursor_coordinates.col > 0 {
            self.cursor_coordinates.col -= 1;
        }
    }

    pub fn cursor_left_promotion(&mut self) {
        self.promotion_cursor = if self.promotion_cursor > 0 {
            self.promotion_cursor - 1
        } else {
            3
        };
    }

    pub fn cursor_right(&mut self, mut authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, 1, authorized_positions);
        } else if self.cursor_coordinates.col < 7 {
            self.cursor_coordinates.col += 1;
        }
    }

    pub fn cursor_right_promotion(&mut self) {
        self.promotion_cursor = (self.promotion_cursor + 1) % 4;
    }

    /// Method to unselect a cell
    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_coordinates = Coord::undefined();
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position;
        }
    }
}
