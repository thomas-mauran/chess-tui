use super::{Movable, PieceColor, PieceMove, PieceType, Position};
use crate::constants::DisplayMode;
use crate::game_logic::coord::Coord;
use crate::game_logic::game_board::GameBoard;
use crate::game_logic::perspective::PerspectiveManager;
use crate::utils::{cleaned_positions, invert_position, is_cell_color_ally};

pub struct Pawn;

impl Movable for Pawn {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
    ) -> Vec<Coord> {
        // Pawns can only move in one direction depending of their color
        // -1 we go up
        let direction: i8 = if allow_move_on_ally_positions { 1 } else { -1 };

        let mut positions: Vec<Coord> = vec![];

        let (y, x) = (coordinates.row, coordinates.col);

        // move one in front
        let new_x_front_one = x;
        let new_y_front_one = y as i8 + direction;
        let new_coordinates_front_one = Coord::new(new_y_front_one as u8, new_x_front_one);

        if new_coordinates_front_one.is_valid()
            && !allow_move_on_ally_positions
            && game_board
                .get_piece_color(&new_coordinates_front_one)
                .is_none()
        {
            // Empty cell
            positions.push(new_coordinates_front_one);

            // move front a second cell
            let new_x_front_two = x;
            let new_y_front_two = y as i8 + direction * 2;
            let new_coordinates_front_two = Coord::new(new_y_front_two as u8, new_x_front_two);

            if new_coordinates_front_two.is_valid()
                && game_board
                    .get_piece_color(&new_coordinates_front_two)
                    .is_none()
                && (y == 6)
            {
                positions.push(new_coordinates_front_two);
            }
        }

        // check for enemy piece on the right
        let new_x_right = x + 1;
        let new_y_right = y as i8 + direction;
        let new_coordinates_right =
            if let Some(new_coord) = Coord::opt_new(new_y_right, new_x_right) {
                new_coord
            } else {
                Coord::undefined()
            };

        // check for enemy piece on the left
        let new_x_left = x as i8 - 1;
        let new_y_left = y as i8 + direction;
        let new_coordinates_left = if let Some(new_coord) = Coord::opt_new(new_y_left, new_x_left) {
            new_coord
        } else {
            Coord::undefined()
        };

        // If we allow on ally position we push it anyway

        if allow_move_on_ally_positions {
            if new_coordinates_right.is_valid() {
                positions.push(new_coordinates_right);
            };
            if new_coordinates_left.is_valid() {
                positions.push(new_coordinates_left);
            };
        } else {
            // else we check if it's an ally piece
            if new_coordinates_right.is_valid()
                && game_board.get_piece_color(&new_coordinates_right).is_some()
                && !is_cell_color_ally(game_board, &new_coordinates_right, color)
            {
                positions.push(new_coordinates_right);
            }
            if new_coordinates_left.is_valid()
                && game_board.get_piece_color(&new_coordinates_left).is_some()
                && !is_cell_color_ally(game_board, &new_coordinates_left, color)
            {
                positions.push(new_coordinates_left);
            }
        }

        // We check for en passant
        if let Some(latest_move) = game_board.move_history.last() {
            let number_of_cells_move = latest_move.to.row as i8 - latest_move.from.row as i8;

            let last_coords = invert_position(&Coord::new(latest_move.to.row, latest_move.to.col));
            // We check if the latest move was on the right start cell
            // if it moved 2 cells
            // and if the current pawn is next to this pawn latest position
            if latest_move.piece_type == PieceType::Pawn
                && number_of_cells_move == -2
                && y == last_coords.row
                && (x as i8 == (last_coords.col as i8) - 1 || x == last_coords.col + 1)
            {
                let new_y = y - 1;
                let new_x = last_coords.col;
                positions.push(Coord::new(new_y, new_x));
            }
        }
        cleaned_positions(&positions)
    }
}

impl Position for Pawn {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        _is_king_checked: bool,
    ) -> Vec<Coord> {
        // If the king is not checked we get then normal moves
        // if the king is checked we clean all the position not resolving the check
        game_board.impossible_positions_king_checked(
            coordinates,
            Self::piece_move(coordinates, color, game_board, false),
            color,
        )
    }

    fn authorized_positions_with_perspective(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        _is_king_checked: bool,
        perspective: Option<&PerspectiveManager>,
    ) -> Vec<Coord> {
        // If the king is not checked we get then normal moves
        // if the king is checked we clean all the position not resolving the check
        game_board.impossible_positions_king_checked(
            coordinates,
            Self::piece_move_with_perspective(coordinates, color, game_board, false, perspective),
            color,
        )
    }

    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, game_board, true)
    }
}

impl Pawn {
    /// Perspective-aware piece movement for pawns
    pub fn piece_move_with_perspective(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
        perspective: Option<&PerspectiveManager>,
    ) -> Vec<Coord> {
        // Determine the correct direction based on piece color and perspective
        let base_direction: i8 = if allow_move_on_ally_positions { 1 } else { -1 };

        // Adjust direction based on perspective
        let direction = if let Some(perspective_manager) = perspective {
            if perspective_manager.needs_transformation() {
                // When board is flipped (black perspective), reverse the direction
                -base_direction
            } else {
                base_direction
            }
        } else {
            // No perspective information, use default logic
            base_direction
        };

        let mut positions: Vec<Coord> = vec![];
        let (y, x) = (coordinates.row, coordinates.col);

        // move one in front
        let new_x_front_one = x;
        let new_y_front_one = y as i8 + direction;
        let new_coordinates_front_one = Coord::new(new_y_front_one as u8, new_x_front_one);

        if new_coordinates_front_one.is_valid()
            && !allow_move_on_ally_positions
            && game_board
                .get_piece_color(&new_coordinates_front_one)
                .is_none()
        {
            // Empty cell
            positions.push(new_coordinates_front_one);

            // move front a second cell
            let new_x_front_two = x;
            let new_y_front_two = y as i8 + direction * 2;
            let new_coordinates_front_two = Coord::new(new_y_front_two as u8, new_x_front_two);

            // Check if it's the starting position for a pawn
            let starting_row = match color {
                PieceColor::White => 6, // White pawns start at row 6
                PieceColor::Black => 1, // Black pawns start at row 1
            };

            if new_coordinates_front_two.is_valid()
                && game_board
                    .get_piece_color(&new_coordinates_front_two)
                    .is_none()
                && (y == starting_row)
            {
                positions.push(new_coordinates_front_two);
            }
        }

        // check for enemy piece on the right
        let new_x_right = x + 1;
        let new_y_right = y as i8 + direction;
        let new_coordinates_right =
            if let Some(new_coord) = Coord::opt_new(new_y_right, new_x_right) {
                new_coord
            } else {
                Coord::undefined()
            };

        // check for enemy piece on the left
        let new_x_left = x as i8 - 1;
        let new_y_left = y as i8 + direction;
        let new_coordinates_left = if let Some(new_coord) = Coord::opt_new(new_y_left, new_x_left) {
            new_coord
        } else {
            Coord::undefined()
        };

        // Handle capture logic (similar to original but with perspective-aware coordinates)
        if allow_move_on_ally_positions {
            if new_coordinates_right.is_valid() {
                positions.push(new_coordinates_right);
            }
            if new_coordinates_left.is_valid() {
                positions.push(new_coordinates_left);
            }
        } else {
            // Check for enemy pieces to capture
            if new_coordinates_right.is_valid() {
                if let Some(piece_color) = game_board.get_piece_color(&new_coordinates_right) {
                    if piece_color != color {
                        positions.push(new_coordinates_right);
                    }
                }
            }
            if new_coordinates_left.is_valid() {
                if let Some(piece_color) = game_board.get_piece_color(&new_coordinates_left) {
                    if piece_color != color {
                        positions.push(new_coordinates_left);
                    }
                }
            }

            // Check for en passant opportunities
            if let Some(en_passant_positions) =
                Self::get_en_passant_positions(coordinates, color, game_board, perspective)
            {
                positions.extend(en_passant_positions);
            }
        }
        cleaned_positions(&positions)
    }
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
        \n\
        \n\
      ▟█▙\n\
      ▜█▛\n\
     ▟███▙\n\
    "
            }
            DisplayMode::ASCII => "P",
        }
    }

    // Check if the pawn moved two cells (used for en passant)
    pub fn did_pawn_move_two_cells(last_move: Option<&PieceMove>) -> bool {
        match last_move {
            Some(last_move) => {
                let distance = (last_move.to.row as i8 - last_move.from.row as i8).abs();

                if last_move.piece_type == PieceType::Pawn && distance == 2 {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    /// Get en passant capture positions for a pawn
    /// En passant is a special pawn capture that can occur when:
    /// 1. An enemy pawn moves two squares from its starting position
    /// 2. Our pawn is on an adjacent file (column) to the enemy pawn
    /// 3. We can capture the enemy pawn as if it had only moved one square
    fn get_en_passant_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        perspective: Option<&PerspectiveManager>,
    ) -> Option<Vec<Coord>> {
        let latest_move = game_board.move_history.last()?;

        // Only proceed if the latest move was a pawn moving exactly 2 squares
        if !Self::is_pawn_two_square_move(latest_move) {
            return None;
        }

        let x = coordinates.col;
        let mut en_passant_positions = Vec::new();

        // Check adjacent columns for enemy pawns that can be captured en passant
        for adj_col in Self::get_adjacent_columns(x) {
            if let Some(en_passant_pos) = Self::check_en_passant_capture(
                coordinates,
                adj_col,
                color,
                latest_move,
                game_board,
                perspective,
            ) {
                en_passant_positions.push(en_passant_pos);
            }
        }

        if en_passant_positions.is_empty() {
            None
        } else {
            Some(en_passant_positions)
        }
    }

    /// Check if the given move was a pawn moving exactly 2 squares
    fn is_pawn_two_square_move(last_move: &PieceMove) -> bool {
        last_move.piece_type == PieceType::Pawn
            && (last_move.to.row as i8 - last_move.from.row as i8).abs() == 2
    }

    /// Get the adjacent column indices for en passant checking
    fn get_adjacent_columns(x: u8) -> Vec<i8> {
        let mut cols = Vec::new();
        if x > 0 {
            cols.push(x as i8 - 1);
        }
        if x < 7 {
            cols.push(x as i8 + 1);
        }
        cols
    }

    /// Check if an en passant capture is possible at the given adjacent column
    fn check_en_passant_capture(
        coordinates: &Coord,
        adj_col: i8,
        color: PieceColor,
        last_move: &PieceMove,
        game_board: &GameBoard,
        perspective: Option<&PerspectiveManager>,
    ) -> Option<Coord> {
        let y = coordinates.row;
        let adjacent_pos = Coord::new(y, adj_col as u8);

        // Check if there's an enemy pawn at the adjacent position
        let piece_color = game_board.get_piece_color(&adjacent_pos)?;

        // Must be an enemy pawn that matches the color of the pawn that just moved
        if piece_color == color || piece_color != last_move.piece_color {
            return None;
        }

        // verify that this is the last piece that was moved to those adjacent columns
        if let Some(last_move) = game_board.move_history.last() {
            if last_move.to.col != adj_col as u8 {
                return None;
            }
        }

        // Calculate the en passant capture position
        let capture_row = Self::calculate_en_passant_capture_row(y, perspective);
        let en_passant_position = Coord::new(capture_row, adj_col as u8);

        // Verify the position is valid
        if en_passant_position.is_valid() {
            Some(en_passant_position)
        } else {
            None
        }
    }

    /// Calculate the row where the en passant capture occurs
    /// In this coordinate system, en passant always captures one row up (toward row 0)
    fn calculate_en_passant_capture_row(
        current_row: u8,
        perspective: Option<&PerspectiveManager>,
    ) -> u8 {
        if let Some(perspective_manager) = perspective {
            if perspective_manager.needs_transformation() {
                current_row + 1
            } else {
                current_row - 1
            }
        } else {
            current_row - 1
        }
    }
}
