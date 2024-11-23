use super::{Movable, PieceColor, PieceMove, Position};
use crate::constants::DisplayMode;
use crate::game::board::Board;
use crate::game::coord::Coord;
use crate::utils::{cleaned_positions, impossible_positions_king_checked, is_cell_color_ally};
pub struct Knight;

impl Movable for Knight {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        board: Board,
        allow_move_on_ally_positions: bool,
        move_history: &[PieceMove],
    ) -> Vec<Coord> {
        let _ = move_history;
        let mut positions: Vec<Coord> = Vec::new();

        // Generate knight positions in all eight possible L-shaped moves
        let piece_move: [(i8, i8); 8] = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        for &(dy, dx) in &piece_move {
            let Some(new_coordinates) =
                Coord::opt_new(coordinates.row as i8 + dy, coordinates.col as i8 + dx)
            else {
                continue;
            };

            if is_cell_color_ally(board, &new_coordinates, color) && !allow_move_on_ally_positions {
                continue;
            }

            positions.push(new_coordinates);
        }

        cleaned_positions(&positions)
    }
}

impl Position for Knight {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: Board,
        move_history: &[PieceMove],
        _is_king_checked: bool,
    ) -> Vec<Coord> {
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
        _move_history: &[PieceMove],
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, board, true, _move_history)
    }
}

impl Knight {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
    \n\
    ▟▛██▙\n\
   ▟█████\n\
   ▀▀▟██▌\n\
    ▟████\n\
    "
            }
            DisplayMode::ASCII => "N",
        }
    }
}
