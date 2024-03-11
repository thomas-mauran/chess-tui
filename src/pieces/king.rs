use super::{Movable, PieceColor, PieceKind, Position};
use crate::{
    board::GameBoard,
    notations::Coords,
    utils::{
        cleaned_positions, did_piece_already_move, get_all_protected_cells, get_piece_kind,
        is_cell_color_ally,
    },
};

pub struct King;

impl Movable for King {
    fn piece_move(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        _move_history: &[(Option<PieceKind>, String)],
    ) -> Vec<Coords> {
        let mut positions: Vec<Coords> = vec![];
        let y = coordinates.row;
        let x = coordinates.col;

        // can move on a complete row
        // Generate positions in all eight possible directions
        for &dy in &[-1, 0, 1] {
            for &dx in &[-1, 0, 1] {
                // Skip the case where both dx and dy are zero (the current position)
                let new_x = x + dx;
                let new_y = y + dy;

                let new_coordinates = Coords::new(new_y, new_x);
                if new_coordinates.is_valid()
                    && (!is_cell_color_ally(board, Coords::new(new_y, new_x), color)
                        || allow_move_on_ally_positions)
                {
                    positions.push(Coords::new(y + dy, x + dx));
                }
            }
        }

        cleaned_positions(positions)
    }
}

impl Position for King {
    fn authorized_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        move_history: &[(Option<PieceKind>, String)],
        is_king_checked: bool,
    ) -> Vec<Coords> {
        let mut positions: Vec<Coords> = vec![];
        let checked_cells = get_all_protected_cells(board, color, move_history);

        let rook_big_castle_x = 0;
        let rook_small_castle_x = 7;
        let king_x = 4;
        let king_line = if color == PieceColor::White { 7 } else { 0 };

        // We check the condition for small and big castling
        if !did_piece_already_move(
            move_history,
            (Some(PieceKind::King), &Coords::new(king_line, king_x)),
        ) && !is_king_checked
        {
            // We check if there is no pieces between tower and king

            // Big castle check
            if !did_piece_already_move(
                move_history,
                (
                    Some(PieceKind::Rook),
                    &Coords::new(king_line, rook_big_castle_x),
                ),
            ) && King::check_castling_condition(board, color, 0, 3, &checked_cells)
            {
                positions.push(Coords::new(king_line, 0));
            }
            // Small castle check
            if !did_piece_already_move(
                move_history,
                (
                    Some(PieceKind::Rook),
                    &Coords::new(king_line, rook_small_castle_x),
                ),
            ) && King::check_castling_condition(board, color, 5, 7, &checked_cells)
            {
                positions.push(Coords::new(king_line, 7));
            }
        }

        // Here we only want king positions that are not in impossible (already checked)
        let king_cells = King::piece_move(coordinates, color, board, false, move_history);

        for king_position in king_cells.clone() {
            if checked_cells.contains(&king_position) {
                positions.push(king_position);
            }
        }

        positions
    }

    // This method is used to calculated the cells the king is actually covering and is used when the other king authorized position is called
    fn protected_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        move_history: &[(Option<PieceKind>, String)],
    ) -> Vec<Coords> {
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
        board: GameBoard,
        color: PieceColor,
        start: i8,
        end: i8,
        checked_cells: &[Coords],
    ) -> bool {
        let king_line = if color == PieceColor::White { 7 } else { 0 };

        let mut valid_for_castling = true;

        for i in start..=end {
            let new_coordinates = Coords::new(king_line, i);

            if checked_cells.contains(&new_coordinates) {
                valid_for_castling = false;
            }
            if (i == 7 || i == 0)
                && (get_piece_kind(&board, &new_coordinates) != Some(PieceKind::Rook)
                    || !is_cell_color_ally(board, new_coordinates.clone(), color))
                || (i != 7 && i != 0 && get_piece_kind(&board, &new_coordinates).is_some())
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
        board::BoardState,
        notations::Coords,
        pieces::{king::King, Piece, PieceColor, PieceKind, Position},
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
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some(Piece::new(PieceKind::King, PieceColor::White)),
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
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = BoardState::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coords::new(4, 5), Coords::new(5, 4)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(4, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
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
                Some(Piece::new(PieceKind::Queen, PieceColor::Black)),
                None,
                None,
                None,
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some(Piece::new(PieceKind::King, PieceColor::White)),
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
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = BoardState::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coords::new(3, 4)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(4, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
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
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
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
                Some(Piece::new(PieceKind::King, PieceColor::White)),
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [
                None,
                None,
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = BoardState::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coords::new(4, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(4, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_white() {
        let custom_board = [
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
                Some(Piece::new(PieceKind::Knight, PieceColor::Black)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                Some(Piece::new(PieceKind::Queen, PieceColor::Black)),
                Some(Piece::new(PieceKind::King, PieceColor::Black)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                Some(Piece::new(PieceKind::Knight, PieceColor::Black)),
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
            ],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
            ],
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
                None,
                None,
                None,
                Some(Piece::new(PieceKind::King, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Knight, PieceColor::White)),
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
            ],
        ];
        let mut board = BoardState::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coords::new(7, 3), Coords::new(7, 0)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(7, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn small_castle_black() {
        let custom_board = [
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
                Some(Piece::new(PieceKind::Knight, PieceColor::Black)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                Some(Piece::new(PieceKind::Queen, PieceColor::Black)),
                Some(Piece::new(PieceKind::King, PieceColor::Black)),
                None,
                None,
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
            ],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
            ],
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
                Some(Piece::new(PieceKind::Knight, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Queen, PieceColor::White)),
                Some(Piece::new(PieceKind::King, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Knight, PieceColor::White)),
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
            ],
        ];
        let mut board = BoardState::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coords::new(0, 5), Coords::new(0, 7)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(0, 4),
            PieceColor::Black,
            board.board,
            &[],
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_check_blocking() {
        let custom_board = [
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
                Some(Piece::new(PieceKind::Knight, PieceColor::Black)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                Some(Piece::new(PieceKind::Queen, PieceColor::Black)),
                Some(Piece::new(PieceKind::King, PieceColor::Black)),
                None,
                None,
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
            ],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                None,
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                None,
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
            ],
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
                Some(Piece::new(PieceKind::Knight, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Queen, PieceColor::White)),
                Some(Piece::new(PieceKind::King, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
                None,
            ],
        ];
        let mut board = BoardState::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coords::new(0, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(0, 4),
            PieceColor::Black,
            board.board,
            &[],
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_king_checked() {
        let custom_board = [
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
                Some(Piece::new(PieceKind::Knight, PieceColor::Black)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                Some(Piece::new(PieceKind::Queen, PieceColor::Black)),
                Some(Piece::new(PieceKind::King, PieceColor::Black)),
                None,
                None,
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
            ],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                None,
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                None,
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
            ],
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
                Some(Piece::new(PieceKind::Knight, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Queen, PieceColor::White)),
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                None,
                Some(Piece::new(PieceKind::King, PieceColor::White)),
            ],
        ];
        let mut board = BoardState::new(&custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions = vec![Coords::new(0, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(0, 4),
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
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
                Some(Piece::new(PieceKind::Knight, PieceColor::Black)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::Black)),
                Some(Piece::new(PieceKind::Queen, PieceColor::Black)),
                Some(Piece::new(PieceKind::King, PieceColor::Black)),
                None,
                None,
                Some(Piece::new(PieceKind::Rook, PieceColor::Black)),
            ],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
                Some(Piece::new(PieceKind::Pawn, PieceColor::White)),
            ],
            [
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
                Some(Piece::new(PieceKind::Knight, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Queen, PieceColor::White)),
                Some(Piece::new(PieceKind::King, PieceColor::White)),
                Some(Piece::new(PieceKind::Bishop, PieceColor::White)),
                Some(Piece::new(PieceKind::Knight, PieceColor::White)),
                Some(Piece::new(PieceKind::Rook, PieceColor::White)),
            ],
        ];
        let mut board = BoardState::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coords::new(0, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coords::new(0, 4),
            PieceColor::Black,
            board.board,
            &[
                (Some(PieceKind::Rook), "0747".to_string()),
                (Some(PieceKind::Pawn), "6252".to_string()),
                (Some(PieceKind::Rook), "4707".to_string()),
            ],
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
