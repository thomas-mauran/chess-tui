use super::{Movable, PieceColor, PieceType, Position};
use crate::{
    board::{Coords, GameBoard},
    utils::{
        cleaned_positions, get_int_from_char, get_latest_move, get_piece_color,
        impossible_positions_king_checked, is_cell_color_ally,
    },
};

pub struct Pawn;

impl Movable for Pawn {
    fn piece_move(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        move_history: &[(Option<PieceType>, String)],
    ) -> Vec<Coords> {
        // Pawns can only move in one direction depending of their color
        // -1 if they are white (go up) +1 if they are black (go down)
        let direction = if color == PieceColor::White { -1 } else { 1 };

        let mut positions: Vec<Coords> = vec![];

        let (y, x) = (coordinates.row, coordinates.col);

        // move one in front
        let new_x_front_one = x;
        let new_y_front_one = y + direction;
        let new_coordinates_front_one = Coords::new(new_y_front_one, new_x_front_one);

        if new_coordinates_front_one.is_valid()
            && !allow_move_on_ally_positions
            && get_piece_color(board, &new_coordinates_front_one).is_none()
        {
            // Empty cell
            positions.push(new_coordinates_front_one);

            // move front a second cell
            let new_x_front_two = x;
            let new_y_front_two = y + direction * 2;
            let new_coordinates_front_two = Coords::new(new_y_front_two, new_x_front_two);

            if new_coordinates_front_two.is_valid()
                && get_piece_color(board, &new_coordinates_front_two).is_none()
                && ((color == PieceColor::White && y == 6)
                    || (color == PieceColor::Black && y == 1))
            {
                positions.push(new_coordinates_front_two);
            }
        }

        // check for enemy piece on the right
        let new_x_right = x + 1;
        let new_y_right = y + direction;
        let new_coordinates_right = Coords::new(new_y_right, new_x_right);

        // check for enemy piece on the left
        let new_x_left = x - 1;
        let new_y_left = y + direction;
        let new_coordinates_left = Coords::new(new_y_left, new_x_left);

        // If we allow on ally position we push it anyway

        if allow_move_on_ally_positions {
            if new_coordinates_right.is_valid() {
                positions.push(new_coordinates_right.clone())
            };
            if new_coordinates_left.is_valid() {
                positions.push(new_coordinates_left)
            };
        } else {
            // else we check if it's an ally piece
            if new_coordinates_right.is_valid()
                && get_piece_color(board, &new_coordinates_right).is_some()
                && !is_cell_color_ally(board, new_coordinates_right.clone(), color)
            {
                positions.push(new_coordinates_right);
            }
            if new_coordinates_left.is_valid()
                && get_piece_color(board, &new_coordinates_left).is_some()
                && !is_cell_color_ally(board, new_coordinates_left.clone(), color)
            {
                positions.push(new_coordinates_left);
            }
        }

        // We check for en passant
        let latest_move = get_latest_move(move_history);

        if let (Some(PieceType::Pawn), piece_move) = latest_move {
            let from_y = get_int_from_char(piece_move.chars().nth(0));
            let from_x = get_int_from_char(piece_move.chars().nth(1));
            let to_y = get_int_from_char(piece_move.chars().nth(2));
            let to_x = get_int_from_char(piece_move.chars().nth(3));

            let valid_y_start: i8;
            let number_of_cells_move: i8;

            if color == PieceColor::White {
                valid_y_start = 1;
                number_of_cells_move = to_y - from_y;
            } else {
                valid_y_start = 6;
                number_of_cells_move = from_y - to_y;
            };

            // We check if the latest move was on the right start cell
            // if it moved 2 cells
            // and if the current pawn is next to this pawn latest position
            if from_y == valid_y_start
                && number_of_cells_move == 2
                && y == to_y
                && (x == to_x - 1 || x == to_x + 1)
            {
                let new_y = from_y + -direction;
                let new_x = from_x;
                positions.push(Coords::new(new_y, new_x));
            }
        }

        cleaned_positions(positions)
    }
}

impl Position for Pawn {
    fn authorized_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        move_history: &[(Option<PieceType>, String)],
        _is_king_checked: bool,
    ) -> Vec<Coords> {
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
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        move_history: &[(Option<PieceType>, String)],
    ) -> Vec<Coords> {
        Self::piece_move(coordinates, color, board, true, move_history)
    }
}

impl Pawn {
    pub fn to_string() -> &'static str {
        "\
        \n\
        \n\
      ▟█▙\n\
      ▜█▛\n\
     ▟███▙\n\
    "
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, Coords},
        pieces::{pawn::Pawn, PieceColor, PieceType, Position},
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

        let mut right_positions = vec![Coords::new(3, 4)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(4, 4),
            PieceColor::White,
            board.board,
            &[(None, "0000".to_string())],
            false,
        );
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

        let mut right_positions = vec![Coords::new(5, 4), Coords::new(4, 4)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(6, 4),
            PieceColor::White,
            board.board,
            &[(None, "0000".to_string())],
            false,
        );
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

        let mut right_positions = vec![
            Coords::new(2, 3),
            Coords::new(3, 3),
            Coords::new(2, 4),
            Coords::new(2, 2),
        ];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(1, 3),
            PieceColor::Black,
            board.board,
            &[(None, "0000".to_string())],
            false,
        );
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

        let mut right_positions = vec![Coords::new(2, 4), Coords::new(2, 2)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(1, 3),
            PieceColor::Black,
            board.board,
            &[(None, "0000".to_string())],
            false,
        );
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

        let mut right_positions = vec![Coords::new(2, 2), Coords::new(2, 3)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(3, 3),
            PieceColor::White,
            board.board,
            &[(Some(PieceType::Pawn), "1232".to_string())],
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

        let mut right_positions = vec![Coords::new(5, 2), Coords::new(5, 3)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(4, 2),
            PieceColor::Black,
            board.board,
            &[(Some(PieceType::Pawn), "6343".to_string())],
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

        let mut right_positions = vec![Coords::new(2, 1), Coords::new(3, 1)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(1, 1),
            PieceColor::Black,
            board.board,
            &[(Some(PieceType::Pawn), "6343".to_string())],
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

        let mut right_positions = vec![Coords::new(3, 2)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(2, 3),
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

        let mut right_positions: Vec<Coords> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(2, 4),
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

        let mut right_positions: Vec<Coords> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coords::new(1, 5),
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
