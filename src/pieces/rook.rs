use super::{Movable, PieceColor, Position};
use crate::constants::DisplayMode;
use crate::game_logic::coord::Coord;
use crate::game_logic::game_board::GameBoard;
use crate::utils::{cleaned_positions, is_cell_color_ally, is_piece_opposite_king};
pub struct Rook;

impl Movable for Rook {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
    ) -> Vec<Coord> {
        // Pawns can only move in one direction depending on their color
        let mut positions = vec![];

        let (y, x) = (coordinates.row, coordinates.col);

        // RIGHT ROW
        for i in 1..8u8 {
            let new_x = x + i;
            let new_y = y;
            let new_coordinates = Coord::new(new_y, new_x);

            // Invalid coords
            if !new_coordinates.is_valid() {
                break;
            }

            // Empty cell
            if game_board.get_piece_color(&new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(game_board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(game_board.board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // LEFT ROW
        for i in 1..=8 {
            let new_x: i8 = x as i8 - i as i8;
            let new_y: i8 = y as i8;
            let Some(new_coordinates) = Coord::opt_new(new_y, new_x) else {
                break;
            };

            // Empty piece
            if game_board.get_piece_color(&new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }

            // Ally piece
            if is_cell_color_ally(game_board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(game_board.board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // BOTTOM ROW
        for i in 1..8u8 {
            let new_x = x;
            let new_y = y + i;
            let new_coordinates = Coord::new(new_y, new_x);

            // Invalid coords
            if !new_coordinates.is_valid() {
                break;
            }

            // Empty cell
            if game_board.get_piece_color(&new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(game_board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }

            // Enemy cell
            positions.push(new_coordinates);

            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(game_board.board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // UP ROW
        for i in 1..8u8 {
            let new_x = x as i8;
            let new_y = y as i8 - i as i8;
            let Some(new_coordinates) = Coord::opt_new(new_y, new_x) else {
                break;
            };

            // Empty cell
            if game_board.get_piece_color(&new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(game_board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }
            // Enemy cell
            positions.push(new_coordinates);

            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(game_board.board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        cleaned_positions(&positions)
    }
}

impl Position for Rook {
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

    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, game_board, true)
    }
}

impl Rook {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
    \n\
    █▟█▙█\n\
    ▜███▛\n\
    ▐███▌\n\
   ▗█████▖\n\
    "
            }
            DisplayMode::ASCII => "R",
        }
    }
}
