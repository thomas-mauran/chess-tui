use super::{Movable, PieceColor, PieceMove, PieceType, Position};
use crate::board::{Coord, GameBoard};
use crate::constants::DisplayMode;
use crate::utils::{
    cleaned_positions, get_piece_color, impossible_positions_king_checked, invert_position,
    is_cell_color_ally,
};

pub struct Pawn;

impl Movable for Pawn {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        move_history: &[PieceMove],
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
            && get_piece_color(board, &new_coordinates_front_one).is_none()
        {
            // Empty cell
            positions.push(new_coordinates_front_one);

            // move front a second cell
            let new_x_front_two = x;
            let new_y_front_two = y as i8 + direction * 2;
            let new_coordinates_front_two = Coord::new(new_y_front_two as u8, new_x_front_two);

            if new_coordinates_front_two.is_valid()
                && get_piece_color(board, &new_coordinates_front_two).is_none()
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
                && get_piece_color(board, &new_coordinates_right).is_some()
                && !is_cell_color_ally(board, &new_coordinates_right, color)
            {
                positions.push(new_coordinates_right);
            }
            if new_coordinates_left.is_valid()
                && get_piece_color(board, &new_coordinates_left).is_some()
                && !is_cell_color_ally(board, &new_coordinates_left, color)
            {
                positions.push(new_coordinates_left);
            }
        }

        // We check for en passant
        if let Some(latest_move) = move_history.last() {
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
        board: GameBoard,
        move_history: &[PieceMove],
        _is_king_checked: bool,
    ) -> Vec<Coord> {
        // If the king is not checked we get then normal moves
        // if the king is checked we clean all the position not resolving the check
        impossible_positions_king_checked(
            coordinates,
            Self::piece_move(coordinates, color, board, false, move_history),
            board,
            color,
            move_history,
        )
    }

    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, board, true, move_history)
    }
}

impl Pawn {
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
}
