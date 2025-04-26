use super::{Movable, Position};
use crate::constants::DisplayMode;
use crate::game_logic::coord::Coord;
use crate::game_logic::game_board::GameBoard;
use crate::utils::{cleaned_positions, is_cell_color_ally, is_piece_opposite_king};
pub struct Bishop;
use shakmaty::Color;

impl Movable for Bishop {
    fn piece_move(
        coordinates: &Coord,
        color: Color,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
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

        // for diagonal from piece to top right
        for i in 1..8u8 {
            let new_x = x as i8 + i as i8;
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
                || !is_piece_opposite_king(
                    game_board.board[new_coordinates.row as usize][new_coordinates.col as usize],
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
        color: Color,
        game_board: &GameBoard,
        _is_king_checked: bool,
    ) -> Vec<Coord> {
        // if the king is checked we clean all the position not resolving the check
        game_board.impossible_positions_king_checked(
            coordinates,
            Self::piece_move(coordinates, color, game_board, false),
            color,
        )
    }
    fn protected_positions(
        coordinates: &Coord,
        color: Color,
        game_board: &GameBoard,
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, game_board, true)
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
