use super::{Movable, PieceColor, PieceMove, PieceType, Position};
use crate::board::DisplayMode;
use crate::utils::{
    cleaned_positions, get_latest_move, get_piece_color, impossible_positions_king_checked,
    is_cell_color_ally, is_valid,
};

pub struct Pawn;

impl Movable for Pawn {
    fn piece_move(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        move_history: &[PieceMove],
    ) -> Vec<Vec<i8>> {
        // Pawns can only move in one direction depending of their color
        // -1 if they are white (go up) +1 if they are black (go down)
        let direction = if color == PieceColor::White { -1 } else { 1 };

        let mut positions: Vec<Vec<i8>> = vec![];

        let (y, x) = (coordinates[0], coordinates[1]);

        // move one in front
        let new_x_front_one = x;
        let new_y_front_one = y + direction;
        let new_coordinates_front_one = [new_y_front_one, new_x_front_one];

        if is_valid(new_coordinates_front_one)
            && !allow_move_on_ally_positions
            && get_piece_color(board, new_coordinates_front_one).is_none()
        {
            // Empty cell
            positions.push(new_coordinates_front_one.to_vec());

            // move front a second cell
            let new_x_front_two = x;
            let new_y_front_two = y + direction * 2;
            let new_coordinates_front_two = [new_y_front_two, new_x_front_two];

            if is_valid(new_coordinates_front_two)
                && get_piece_color(board, new_coordinates_front_two).is_none()
                && ((color == PieceColor::White && y == 6)
                    || (color == PieceColor::Black && y == 1))
            {
                positions.push(new_coordinates_front_two.to_vec());
            }
        }

        // check for enemy piece on the right
        let new_x_right = x + 1;
        let new_y_right = y + direction;
        let new_coordinates_right = [new_y_right, new_x_right];

        // check for enemy piece on the left
        let new_x_left = x - 1;
        let new_y_left = y + direction;
        let new_coordinates_left = [new_y_left, new_x_left];

        // If we allow on ally position we push it anyway

        if allow_move_on_ally_positions {
            if is_valid(new_coordinates_right) {
                positions.push(new_coordinates_right.to_vec())
            };
            if is_valid(new_coordinates_left) {
                positions.push(new_coordinates_left.to_vec())
            };
        } else {
            // else we check if it's an ally piece
            if is_valid(new_coordinates_right)
                && get_piece_color(board, new_coordinates_right).is_some()
                && !is_cell_color_ally(board, new_coordinates_right, color)
            {
                positions.push(new_coordinates_right.to_vec());
            }
            if is_valid(new_coordinates_left)
                && get_piece_color(board, new_coordinates_left).is_some()
                && !is_cell_color_ally(board, new_coordinates_left, color)
            {
                positions.push(new_coordinates_left.to_vec());
            }
        }

        // We check for en passant
        if let Some(latest_move) = get_latest_move(move_history) {
            let valid_y_start: i8;
            let number_of_cells_move: i8;

            if color == PieceColor::White {
                valid_y_start = 1;
                number_of_cells_move = latest_move.to_y - latest_move.from_y;
            } else {
                valid_y_start = 6;
                number_of_cells_move = latest_move.from_y - latest_move.to_y;
            };

            // We check if the latest move was on the right start cell
            // if it moved 2 cells
            // and if the current pawn is next to this pawn latest position
            if latest_move.from_y == valid_y_start
                && number_of_cells_move == 2
                && y == latest_move.to_y
                && (x == latest_move.to_x - 1 || x == latest_move.to_x + 1)
            {
                let new_y = latest_move.from_y + -direction;
                let new_x = latest_move.from_x;
                positions.push([new_y, new_x].to_vec());
            }
        }

        cleaned_positions(positions)
    }
}

impl Position for Pawn {
    fn authorized_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[PieceMove],
        _is_king_checked: bool,
    ) -> Vec<Vec<i8>> {
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
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[PieceMove],
    ) -> Vec<Vec<i8>> {
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

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{pawn::Pawn, PieceColor, PieceMove, PieceType, Position},
        utils::is_getting_checked,
    };

    #[test]
    fn piece_move_one_cell_forward() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![3, 4]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([4, 4], PieceColor::White, board.board, &[], false);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_one_cell_forward_two() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![5, 4], vec![4, 4]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([6, 4], PieceColor::White, board.board, &[], false);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_one_cell_enemy_left_right() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 3], vec![3, 3], vec![2, 4], vec![2, 2]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([1, 3], PieceColor::Black, board.board, &[], false);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_one_cell_3_enemies() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 4], vec![2, 2]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([1, 3], PieceColor::Black, board.board, &[], false);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_en_passant_white() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 2], vec![2, 3]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [3, 3],
            PieceColor::White,
            board.board,
            &[(PieceMove {
                piece_type: PieceType::Pawn,
                from_y: 1,
                from_x: 2,
                to_y: 3,
                to_x: 2,
            })],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_en_passant_black() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![5, 2], vec![5, 3]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [4, 2],
            PieceColor::Black,
            board.board,
            &[(PieceMove {
                piece_type: PieceType::Pawn,
                from_y: 6,
                from_x: 3,
                to_y: 4,
                to_x: 3,
            })],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_not_en_passant() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 1], vec![3, 1]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [1, 1],
            PieceColor::Black,
            board.board,
            &[(PieceMove {
                piece_type: PieceType::Pawn,
                from_y: 6,
                from_x: 3,
                to_y: 4,
                to_x: 3,
            })],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn king_checked_can_resolve() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Queen, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions = vec![vec![3, 2]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [2, 3],
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn king_checked_cant_resolve() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Queen, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions: Vec<Vec<i8>> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [2, 4],
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn nailing() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
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
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Queen, PieceColor::White)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions: Vec<Vec<i8>> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [1, 5],
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
