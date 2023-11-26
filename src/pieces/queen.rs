use super::rook::Rook;
use super::{PieceColor, PieceType};
use crate::pieces::bishop::Bishop;
use crate::utils::cleaned_positions;

pub struct Queen {}

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

    pub fn queen_moves(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
    ) -> Vec<Vec<i32>> {
        let mut positions: Vec<Vec<i32>> = vec![];

        // Queen = bishop concat rook
        positions.extend(Bishop::bishop_moves(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
        ));
        positions.extend(Rook::rook_moves(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
        ));

        cleaned_positions(positions)
    }

    pub fn authorized_positions(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    ) -> Vec<Vec<i32>> {
        Self::queen_moves(coordinates, color, board, false)
    }

    pub fn protecting_positions(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    ) -> Vec<Vec<i32>> {
        Self::queen_moves(coordinates, color, board, true)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{queen::Queen, PieceColor, PieceType},
    };

    #[test]
    fn queen_moves_no_enemies() {
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

        let mut positions = Queen::authorized_positions([4, 4], PieceColor::White, board.board);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn queen_moves_one_enemies_top_right() {
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

        let mut positions = Queen::authorized_positions([4, 4], PieceColor::White, board.board);
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn queen_moves_enemies_and_allies() {
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

        let mut positions = Queen::authorized_positions([4, 4], PieceColor::White, board.board);
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
