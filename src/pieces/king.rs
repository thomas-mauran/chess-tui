use super::{Movable, PieceColor, PieceType, Position};
use crate::constants::DisplayMode;
use crate::game_logic::coord::Coord;
use crate::game_logic::game_board::GameBoard;
use crate::game_logic::perspective::PerspectiveManager;
use crate::utils::{cleaned_positions, is_cell_color_ally};
pub struct King;

impl Movable for King {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
    ) -> Vec<Coord> {
        let mut positions: Vec<Coord> = vec![];
        let y = coordinates.row;
        let x = coordinates.col;

        // can move on a complete row
        // Generate positions in all eight possible directions
        for &dy in &[-1i8, 0, 1] {
            for &dx in &[-1i8, 0, 1] {
                // Skip the case where both dx and dy are zero (the current position)
                let new_x = x as i8 + dx;
                let new_y = y as i8 + dy;

                let new_coordinates = Coord::new(new_y as u8, new_x as u8);
                if new_coordinates.is_valid()
                    && (!is_cell_color_ally(game_board, &new_coordinates, color)
                        || allow_move_on_ally_positions)
                {
                    positions.push(new_coordinates);
                }
            }
        }

        cleaned_positions(&positions)
    }
}

impl Position for King {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        is_king_checked: bool,
    ) -> Vec<Coord> {
        let mut positions: Vec<Coord> = vec![];
        let checked_cells = game_board.get_all_protected_cells(color);

        let rook_big_castle_x = 0;
        let rook_small_castle_x = 7;
        let king_row = 7;
        let king_col = if color == PieceColor::White { 4 } else { 3 };

        // We check the condition for small and big castling
        if !game_board.did_piece_already_move((
            Some(PieceType::King),
            Some(color),
            Coord::new(king_row, king_col),
        )) && !is_king_checked
        {
            // We check if there is no pieces between tower and king
            // Big castle check
            if !game_board.did_piece_already_move((
                Some(PieceType::Rook),
                Some(color),
                Coord::new(king_row, rook_big_castle_x),
            )) && King::check_castling_condition(
                game_board,
                color,
                0,
                king_col as i8 - 1,
                &checked_cells,
            ) {
                positions.push(Coord::new(king_row, 0));
            }
            // Small castle check
            if !game_board.did_piece_already_move((
                Some(PieceType::Rook),
                Some(color),
                Coord::new(king_row, rook_small_castle_x),
            )) && King::check_castling_condition(
                game_board,
                color,
                king_col as i8 + 1,
                7,
                &checked_cells,
            ) {
                positions.push(Coord::new(king_row, 7));
            }
        }

        // Here we only want king positions that are not in impossible (already checked)
        let king_cells = King::piece_move(coordinates, color, game_board, false);

        // Add castling positions to the list
        for king_position in king_cells {
            positions.push(king_position);
        }

        // Filter out positions that would leave the king in check
        game_board.impossible_positions_king_checked(coordinates, positions, color)
    }

    // This method is used to calculated the cells the king is actually covering and is used when the other king authorized position is called
    fn authorized_positions_with_perspective(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        is_king_checked: bool,
        perspective: Option<&PerspectiveManager>,
    ) -> Vec<Coord> {
        let mut positions: Vec<Coord> = vec![];
        let checked_cells = game_board.get_all_protected_cells_with_perspective(color, perspective);

        // Get perspective-aware castling positions
        let castling_positions = Self::get_castling_positions_with_perspective(
            coordinates,
            color,
            game_board,
            is_king_checked,
            &checked_cells,
            perspective,
        );
        positions.extend(castling_positions);

        // Get perspective-aware king movement positions
        let king_cells =
            Self::piece_move_with_perspective(coordinates, color, game_board, false, perspective);

        // Add king movement positions to the list
        for king_position in king_cells {
            positions.push(king_position);
        }

        // Filter out positions that would leave the king in check
        game_board.impossible_positions_king_checked(coordinates, positions, color)
    }

    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, game_board, true)
    }
}

impl King {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
      ✚\n\
    ▞▀▄▀▚\n\
    ▙▄█▄▟\n\
    ▐███▌\n\
   ▗█████▖\n\
    "
            }
            DisplayMode::ASCII => "K",
        }
    }

    // Check if nothing is in between the king and a rook and if none of those cells are getting checked
    pub fn check_castling_condition(
        game_board: &GameBoard,
        color: PieceColor,
        start: i8,
        end: i8,
        checked_cells: &[Coord],
    ) -> bool {
        let king_row = 7;

        let mut valid_for_castling = true;

        for i in start..=end {
            let new_coordinates = Coord::new(king_row, i as u8);

            if checked_cells.contains(&new_coordinates) {
                valid_for_castling = false;
            }
            if (i == 7 || i == 0)
                && (game_board.get_piece_type(&new_coordinates) != Some(PieceType::Rook)
                    || !is_cell_color_ally(game_board, &new_coordinates, color))
                || (i != 7 && i != 0 && game_board.get_piece_type(&new_coordinates).is_some())
            {
                valid_for_castling = false;
            }
        }

        valid_for_castling
    }

    /// Perspective-aware piece movement for kings
    pub fn piece_move_with_perspective(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
        _perspective: Option<&PerspectiveManager>,
    ) -> Vec<Coord> {
        // For now, king movement doesn't need perspective transformation
        // as it moves in all 8 directions regardless of perspective
        // The perspective mainly affects castling positions
        let mut positions: Vec<Coord> = vec![];
        let y = coordinates.row;
        let x = coordinates.col;

        // Generate positions in all eight possible directions
        for &dy in &[-1i8, 0, 1] {
            for &dx in &[-1i8, 0, 1] {
                // Skip the case where both dx and dy are zero (the current position)
                let new_x = x as i8 + dx;
                let new_y = y as i8 + dy;

                let new_coordinates = Coord::new(new_y as u8, new_x as u8);
                if new_coordinates.is_valid()
                    && (!is_cell_color_ally(game_board, &new_coordinates, color)
                        || allow_move_on_ally_positions)
                {
                    positions.push(new_coordinates);
                }
            }
        }

        cleaned_positions(&positions)
    }

    /// Get castling positions with perspective awareness
    fn get_castling_positions_with_perspective(
        _coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        is_king_checked: bool,
        checked_cells: &[Coord],
        perspective: Option<&PerspectiveManager>,
    ) -> Vec<Coord> {
        let mut positions: Vec<Coord> = vec![];

        // Determine the correct row and column based on perspective
        let (king_row, king_col) = Self::get_king_position_for_color(color, perspective);
        let (rook_big_castle_x, rook_small_castle_x) =
            Self::get_rook_positions_for_color(color, perspective);

        // We check the condition for small and big castling
        if !game_board.did_piece_already_move((
            Some(PieceType::King),
            Some(color),
            Coord::new(king_row, king_col),
        )) && !is_king_checked
        {
            // We check if there is no pieces between tower and king
            // Big castle check
            if !game_board.did_piece_already_move((
                Some(PieceType::Rook),
                Some(color),
                Coord::new(king_row, rook_big_castle_x),
            )) && King::check_castling_condition(
                game_board,
                color,
                rook_big_castle_x as i8,
                king_col as i8 - 1,
                checked_cells,
            ) {
                positions.push(Coord::new(king_row, rook_big_castle_x));
            }
            // Small castle check
            if !game_board.did_piece_already_move((
                Some(PieceType::Rook),
                Some(color),
                Coord::new(king_row, rook_small_castle_x),
            )) && King::check_castling_condition(
                game_board,
                color,
                king_col as i8 + 1,
                rook_small_castle_x as i8,
                checked_cells,
            ) {
                positions.push(Coord::new(king_row, rook_small_castle_x));
            }
        }

        positions
    }

    /// Get the correct king position based on color and perspective
    fn get_king_position_for_color(
        color: PieceColor,
        perspective: Option<&PerspectiveManager>,
    ) -> (u8, u8) {
        match color {
            PieceColor::White => {
                if let Some(perspective_manager) = perspective {
                    if perspective_manager.needs_transformation() {
                        // Black perspective: white king appears at row 0, col 3
                        (0, 3)
                    } else {
                        // White perspective: white king at row 7, col 4
                        (7, 4)
                    }
                } else {
                    // Default: white king at row 7, col 4
                    (7, 4)
                }
            }
            PieceColor::Black => {
                if let Some(perspective_manager) = perspective {
                    if perspective_manager.needs_transformation() {
                        // Black perspective: black king at row 7, col 4
                        (7, 4)
                    } else {
                        // White perspective: black king at row 0, col 3
                        (0, 3)
                    }
                } else {
                    // Default: black king at row 0, col 3
                    (0, 3)
                }
            }
        }
    }

    /// Get the correct rook positions based on color and perspective
    fn get_rook_positions_for_color(
        color: PieceColor,
        perspective: Option<&PerspectiveManager>,
    ) -> (u8, u8) {
        match color {
            PieceColor::White => {
                if let Some(perspective_manager) = perspective {
                    if perspective_manager.needs_transformation() {
                        // Black perspective: white rooks at row 0, cols 0 and 7
                        (0, 7)
                    } else {
                        // White perspective: white rooks at row 7, cols 0 and 7
                        (0, 7)
                    }
                } else {
                    // Default: white rooks at row 7, cols 0 and 7
                    (0, 7)
                }
            }
            PieceColor::Black => {
                if let Some(perspective_manager) = perspective {
                    if perspective_manager.needs_transformation() {
                        // Black perspective: black rooks at row 7, cols 0 and 7
                        (0, 7)
                    } else {
                        // White perspective: black rooks at row 0, cols 0 and 7
                        (0, 7)
                    }
                } else {
                    // Default: black rooks at row 0, cols 0 and 7
                    (0, 7)
                }
            }
        }
    }
}
