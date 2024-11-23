use super::{Movable, PieceColor, PieceMove, PieceType, Position};
use crate::constants::DisplayMode;
use crate::game::coord::Coord;
use crate::game::game::GameBoard;
use crate::utils::{
    cleaned_positions, did_piece_already_move, get_all_protected_cells, get_piece_type,
    is_cell_color_ally,
};
pub struct King;

impl Movable for King {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        _move_history: &[PieceMove],
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
                    && (!is_cell_color_ally(board, &new_coordinates, color)
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
        board: GameBoard,
        move_history: &[PieceMove],
        is_king_checked: bool,
    ) -> Vec<Coord> {
        let mut positions: Vec<Coord> = vec![];
        let checked_cells = get_all_protected_cells(board, color, move_history);

        let rook_big_castle_x = 0;
        let rook_small_castle_x = 7;
        let king_x = 4;
        let king_row = 7;
        let king_col = if color == PieceColor::White { 4 } else { 3 };

        // We check the condition for small and big castling
        if !did_piece_already_move(
            move_history,
            (
                Some(PieceType::King),
                Some(color),
                Coord::new(king_row, king_x),
            ),
        ) && !is_king_checked
        {
            // We check if there is no pieces between tower and king
            // Big castle check
            if !did_piece_already_move(
                move_history,
                (
                    Some(PieceType::Rook),
                    Some(color),
                    Coord::new(king_row, rook_big_castle_x),
                ),
            ) && King::check_castling_condition(board, color, 0, king_col - 1, &checked_cells)
            {
                positions.push(Coord::new(king_row, 0));
            }
            // Small castle check
            if !did_piece_already_move(
                move_history,
                (
                    Some(PieceType::Rook),
                    Some(color),
                    Coord::new(king_row, rook_small_castle_x),
                ),
            ) && King::check_castling_condition(board, color, king_col + 1, 7, &checked_cells)
            {
                positions.push(Coord::new(king_row, 7));
            }
        }

        // Here we only want king positions that are not in impossible (already checked)
        let king_cells = King::piece_move(coordinates, color, board, false, move_history);

        for king_position in king_cells.clone() {
            if !checked_cells.contains(&king_position) {
                positions.push(king_position);
            }
        }

        positions
    }

    // This method is used to calculated the cells the king is actually covering and is used when the other king authorized position is called
    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, board, true, move_history)
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
        board: GameBoard,
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
                && (get_piece_type(board, &new_coordinates) != Some(PieceType::Rook)
                    || !is_cell_color_ally(board, &new_coordinates, color))
                || (i != 7 && i != 0 && get_piece_type(board, &new_coordinates).is_some())
            {
                valid_for_castling = false;
            }
        }

        valid_for_castling
    }
}
