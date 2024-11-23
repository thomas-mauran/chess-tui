use super::{Movable, PieceColor, PieceMove, Position};
use crate::constants::DisplayMode;
use crate::game::board::Board;
use crate::game::coord::Coord;
use crate::utils::{
    cleaned_positions, get_piece_color, impossible_positions_king_checked, is_cell_color_ally,
    is_piece_opposite_king,
};
pub struct Bishop;

impl Movable for Bishop {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        board: Board,
        allow_move_on_ally_positions: bool,
        _move_history: &[PieceMove],
    ) -> Vec<Coord> {
        let mut positions: Vec<Coord> = vec![];

        let y = coordinates.row;
        let x = coordinates.col;

        // for diagonal from piece to top left
        for i in 1..8u8 {
            let new_x: i8 = x as i8 - i as i8;
            let new_y: i8 = y as i8 - i as i8;
            let Some(new_coordinates) = Coord::opt_new(new_y, new_x) else {
                break;
            };

            // Empty cell
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // for diagonal from piece to bottom right
        for i in 1..8u8 {
            let new_x = x + i;
            let new_y = y + i;

            let new_coordinates = Coord::new(new_y, new_x);

            // Invalid coords
            if !new_coordinates.is_valid() {
                break;
            }

            // Empty cell
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // for diagonal from piece to bottom left
        for i in 1..8u8 {
            let new_x: i8 = x as i8 - i as i8;
            let new_y: i8 = y as i8 + i as i8;
            let Some(new_coordinates) = Coord::opt_new(new_y, new_x) else {
                break;
            };

            // Invalid coords
            if !new_coordinates.is_valid() {
                break;
            }

            // Empty cell
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // for diagonal from piece to top right
        for i in 1..8u8 {
            let new_x = x as i8 + i as i8;
            let new_y = y as i8 - i as i8;
            let Some(new_coordinates) = Coord::opt_new(new_y, new_x) else {
                break;
            };

            // Empty cell
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                }
                positions.push(new_coordinates);
                break;
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(
                    board[new_coordinates.row as usize][new_coordinates.col as usize],
                    color,
                )
            {
                break;
            }
        }
        cleaned_positions(&positions)
    }
}

impl Position for Bishop {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: Board,
        move_history: &[PieceMove],
        _is_king_checked: bool,
    ) -> Vec<Coord> {
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
        board: Board,
        move_history: &[PieceMove],
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, board, true, move_history)
    }
}

impl Bishop {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
    \n\
       ⭘\n\
      █✝█\n\
      ███\n\
    ▗█████▖\n\
    "
            }
            DisplayMode::ASCII => "B",
        }
    }
}
