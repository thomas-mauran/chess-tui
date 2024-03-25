use super::{Movable, PieceColor, PieceType, Position};
use crate::{
    board::HistRec,
    utils::{
        cleaned_positions, did_piece_already_move, get_all_protected_cells, get_piece_type,
        is_cell_color_ally, is_valid, is_vec_in_array,
    },
};

pub struct King;

impl Movable for King {
    fn piece_move(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        _move_history: &[HistRec],
    ) -> Vec<Vec<i8>> {
        let mut positions: Vec<Vec<i8>> = vec![];
        let y = coordinates[0];
        let x = coordinates[1];

        // can move on a complete row
        // Generate positions in all eight possible directions
        for &dy in &[-1, 0, 1] {
            for &dx in &[-1, 0, 1] {
                // Skip the case where both dx and dy are zero (the current position)
                let new_x = x + dx;
                let new_y = y + dy;

                let new_coordinates = [new_y, new_x];
                if is_valid(new_coordinates)
                    && (!is_cell_color_ally(board, [new_y, new_x], color)
                        || allow_move_on_ally_positions)
                {
                    positions.push(vec![y + dy, x + dx]);
                }
            }
        }

        cleaned_positions(positions)
    }
}

impl Position for King {
    fn authorized_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[HistRec],
        is_king_checked: bool,
    ) -> Vec<Vec<i8>> {
        let mut positions: Vec<Vec<i8>> = vec![];
        let checked_cells = get_all_protected_cells(board, color, move_history);

        let rook_big_castle_x = 0;
        let rook_small_castle_x = 7;
        let king_x = 4;
        let king_line = if color == PieceColor::White { 7 } else { 0 };

        // We check the condition for small and big castling
        if !did_piece_already_move(move_history, (PieceType::King, [king_line, king_x]))
            && !is_king_checked
        {
            // We check if there is no pieces between tower and king

            // Big castle check
            if !did_piece_already_move(
                move_history,
                (PieceType::Rook, [king_line, rook_big_castle_x]),
            ) && King::check_castling_condition(board, color, 0, 3, &checked_cells)
            {
                positions.push(vec![king_line, 0]);
            }
            // Small castle check
            if !did_piece_already_move(
                move_history,
                (PieceType::Rook, [king_line, rook_small_castle_x]),
            ) && King::check_castling_condition(board, color, 5, 7, &checked_cells)
            {
                positions.push(vec![king_line, 7]);
            }
        }

        // Here we only want king positions that are not in impossible (already checked)
        let king_cells = King::piece_move(coordinates, color, board, false, move_history);

        for king_position in king_cells.clone() {
            if !is_vec_in_array(checked_cells.clone(), [king_position[0], king_position[1]]) {
                positions.push(king_position);
            }
        }

        positions
    }

    // This method is used to calculated the cells the king is actually covering and is used when the other king authorized position is called
    fn protected_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[HistRec],
    ) -> Vec<Vec<i8>> {
        Self::piece_move(coordinates, color, board, true, move_history)
    }
}

impl King {
    pub fn to_string() -> &'static str {
        "\
      ✚\n\
    ▞▀▄▀▚\n\
    ▙▄█▄▟\n\
    ▐███▌\n\
   ▗█████▖\n\
    "
    }

    // Check if nothing is in between the king and a rook and if none of those cells are getting checked
    pub fn check_castling_condition(
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        color: PieceColor,
        start: i8,
        end: i8,
        checked_cells: &[Vec<i8>],
    ) -> bool {
        let king_line = if color == PieceColor::White { 7 } else { 0 };

        let mut valid_for_castling = true;

        for i in start..=end {
            let new_coordinates = [king_line, i];

            if is_vec_in_array(checked_cells.to_owned().clone(), new_coordinates) {
                valid_for_castling = false;
            }
            if (i == 7 || i == 0)
                && (get_piece_type(board, new_coordinates) != Some(PieceType::Rook)
                    || !is_cell_color_ally(board, new_coordinates, color))
                || (i != 7 && i != 0 && get_piece_type(board, new_coordinates).is_some())
            {
                valid_for_castling = false;
            }
        }

        valid_for_castling
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{king::King, PieceColor, PieceType, Position},
        utils::is_getting_checked,
    };

    #[test]
    fn multiple_enemies_1() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![4, 5], vec![5, 4]];
        right_positions.sort();

        let mut positions =
            King::authorized_positions([4, 4], PieceColor::White, board.board, &[], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn multiple_enemies_2() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![3, 4]];
        right_positions.sort();

        let mut positions =
            King::authorized_positions([4, 4], PieceColor::White, board.board, &[], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn multiple_enemies_3() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![4, 5]];
        right_positions.sort();

        let mut positions =
            King::authorized_positions([4, 4], PieceColor::White, board.board, &[], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_white() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![7, 3], vec![7, 0]];
        right_positions.sort();

        let mut positions =
            King::authorized_positions([7, 4], PieceColor::White, board.board, &[], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn small_castle_black() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![0, 5], vec![0, 7]];
        right_positions.sort();

        let mut positions =
            King::authorized_positions([0, 4], PieceColor::Black, board.board, &[], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_check_blocking() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
                None,
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![0, 5]];
        right_positions.sort();

        let mut positions =
            King::authorized_positions([0, 4], PieceColor::Black, board.board, &[], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_king_checked() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions = vec![vec![0, 5]];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            [0, 4],
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_rook_already_moved() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![0, 5]];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            [0, 4],
            PieceColor::Black,
            board.board,
            &[
                (PieceType::Rook, "0747".to_string()),
                (PieceType::Pawn, "6252".to_string()),
                (PieceType::Rook, "4707".to_string()),
            ],
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
