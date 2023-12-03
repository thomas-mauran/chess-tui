use super::{Movable, PieceColor, PieceType, Position};
use crate::utils::{
    cleaned_positions, impossible_positions_king_checked, is_cell_color_ally, is_valid,
};
pub struct Knight;

impl Movable for Knight {
    fn piece_move(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        _move_history: &Vec<(Option<PieceType>, String)>,
    ) -> Vec<Vec<i8>> {
        let mut positions: Vec<Vec<i8>> = Vec::new();

        let (y, x) = (coordinates[0], coordinates[1]);

        // Generate knight positions in all eight possible L-shaped moves
        let piece_move = [
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
            let new_coordinates = [y + dy, x + dx];

            if !is_valid(new_coordinates) {
                continue;
            }

            if is_cell_color_ally(board, new_coordinates, color) && !allow_move_on_ally_positions {
                continue;
            }

            positions.push(new_coordinates.to_vec());
        }

        cleaned_positions(positions)
    }
}

impl Position for Knight {
    fn authorized_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &Vec<(Option<PieceType>, String)>,
        _is_king_checked: bool,
    ) -> Vec<Vec<i8>> {
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
        _move_history: &Vec<(Option<PieceType>, String)>,
    ) -> Vec<Vec<i8>> {
        Self::piece_move(coordinates, color, board, true, _move_history)
    }
}

impl Knight {
    pub fn to_string() -> &'static str {
        "\
    \n\
    ▟▛██▙\n\
   ▟█████\n\
   ▀▀▟██▌\n\
    ▟████\n\
    "
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{knight::Knight, PieceColor, PieceType, Position},
        utils::is_getting_checked,
    };

    #[test]
    fn no_enemies() {
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
                Some((PieceType::Knight, PieceColor::White)),
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

        let mut right_positions = vec![
            vec![2, 3],
            vec![2, 5],
            vec![3, 2],
            vec![3, 6],
            vec![5, 2],
            vec![5, 6],
            vec![6, 3],
            vec![6, 5],
        ];
        right_positions.sort();

        let mut positions =
            Knight::authorized_positions([4, 4], PieceColor::White, board.board, &vec![], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn enemy_and_ally() {
        let custom_board = [
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
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
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
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Knight, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![6, 5]];
        right_positions.sort();

        let mut positions =
            Knight::authorized_positions([7, 7], PieceColor::White, board.board, &vec![], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn king_checked_can_resolve() {
        let custom_board = [
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
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Knight, PieceColor::White)),
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Knight, PieceColor::Black)),
            ],
        ];
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions = vec![vec![7, 7]];
        right_positions.sort();

        let mut positions = Knight::authorized_positions(
            [6, 5],
            PieceColor::White,
            board.board,
            &vec![],
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Knight, PieceColor::White)),
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
                None,
                None,
                Some((PieceType::Knight, PieceColor::Black)),
            ],
        ];
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions: Vec<Vec<i8>> = vec![];
        right_positions.sort();

        let mut positions = Knight::authorized_positions(
            [6, 4],
            PieceColor::White,
            board.board,
            &vec![],
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
                Some((PieceType::Knight, PieceColor::Black)),
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
                Some((PieceType::Queen, PieceColor::White)),
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

        let mut positions = Knight::authorized_positions(
            [1, 4],
            PieceColor::Black,
            board.board,
            &vec![],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
