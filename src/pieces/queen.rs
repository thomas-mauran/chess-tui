use super::rook::Rook;
use super::{Movable, PieceColor, PieceType, Position};
use crate::pieces::bishop::Bishop;
use crate::utils::{cleaned_positions, impossible_positions_king_checked};

pub struct Queen;

impl Movable for Queen {
    fn piece_move(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        _move_history: Vec<(Option<PieceType>, String)>,
    ) -> Vec<Vec<i8>> {
        let mut positions: Vec<Vec<i8>> = vec![];

        // Queen = bishop concat rook
        positions.extend(Bishop::piece_move(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
            _move_history.clone(),
        ));
        positions.extend(Rook::piece_move(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
            _move_history,
        ));

        cleaned_positions(positions)
    }
}

impl Position for Queen {
    fn authorized_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: Vec<(Option<PieceType>, String)>,
        _is_king_checked: bool,
    ) -> Vec<Vec<i8>> {
        impossible_positions_king_checked(
            coordinates,
            Self::piece_move(coordinates, color, board, false, move_history.clone()),
            board,
            color,
            move_history,
        )
    }
    fn protected_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        _move_history: Vec<(Option<PieceType>, String)>,
    ) -> Vec<Vec<i8>> {
        Self::piece_move(coordinates, color, board, true, _move_history)
    }
}

impl Queen {
    pub fn to_string() -> &'static str {
        "\
    \n\
    ○○○○○\n\
    ▙▙█▟▟\n\
    ▐▄▄▄▌\n\
    ▐███▌\n\
    "
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{queen::Queen, PieceColor, PieceType, Position},
        utils::is_getting_checked,
    };

    #[test]
    fn piece_move_no_enemies() {
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
                Some((PieceType::Queen, PieceColor::White)),
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
            vec![0, 0],
            vec![0, 4],
            vec![1, 1],
            vec![1, 4],
            vec![1, 7],
            vec![2, 2],
            vec![2, 4],
            vec![2, 6],
            vec![3, 3],
            vec![3, 4],
            vec![3, 5],
            vec![4, 0],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
            vec![4, 5],
            vec![4, 6],
            vec![4, 7],
            vec![5, 3],
            vec![5, 4],
            vec![5, 5],
            vec![6, 2],
            vec![6, 4],
            vec![6, 6],
            vec![7, 1],
            vec![7, 4],
            vec![7, 7],
        ];
        right_positions.sort();

        let mut positions =
            Queen::authorized_positions([4, 4], PieceColor::White, board.board, vec![], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_one_enemies_top_right() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
                Some((PieceType::Queen, PieceColor::White)),
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
            vec![0, 0],
            vec![0, 4],
            vec![1, 1],
            vec![1, 4],
            vec![2, 2],
            vec![2, 4],
            vec![3, 3],
            vec![3, 4],
            vec![3, 5],
            vec![4, 0],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
            vec![4, 5],
            vec![4, 6],
            vec![4, 7],
            vec![5, 3],
            vec![5, 4],
            vec![5, 5],
            vec![6, 2],
            vec![6, 4],
            vec![6, 6],
            vec![7, 1],
            vec![7, 4],
            vec![7, 7],
        ];
        right_positions.sort();

        let mut positions =
            Queen::authorized_positions([4, 4], PieceColor::White, board.board, vec![], false);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_enemies_and_allies() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Queen, PieceColor::White)),
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
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
            vec![1, 7],
            vec![2, 6],
            vec![3, 3],
            vec![3, 5],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
            vec![4, 5],
            vec![4, 6],
            vec![4, 7],
            vec![5, 3],
            vec![5, 4],
            vec![5, 5],
            vec![6, 2],
            vec![6, 4],
            vec![7, 4],
        ];

        right_positions.sort();

        let mut positions =
            Queen::authorized_positions([4, 4], PieceColor::White, board.board, vec![], false);
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
                None,
                None,
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
                Some((PieceType::Bishop, PieceColor::White)),
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, board.moves_history);

        let mut right_positions = vec![vec![4, 4]];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            [5, 5],
            PieceColor::Black,
            board.board,
            vec![],
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
                None,
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
                Some((PieceType::Bishop, PieceColor::White)),
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
                Some((PieceType::Queen, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, board.moves_history);

        let mut right_positions: Vec<Vec<i8>> = vec![];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            [5, 6],
            PieceColor::Black,
            board.board,
            vec![],
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
                Some((PieceType::Queen, PieceColor::Black)),
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
            is_getting_checked(board.board, board.player_turn, board.moves_history);

        let mut right_positions: Vec<Vec<i8>> = vec![vec![2, 6], vec![3, 7]];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            [1, 5],
            PieceColor::Black,
            board.board,
            vec![],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
