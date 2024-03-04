use super::rook::Rook;
use super::{Movable, PieceColor, PieceType, Position};
use crate::board::Coords;
use crate::pieces::bishop::Bishop;
use crate::utils::{cleaned_positions, impossible_positions_king_checked};

pub struct Queen;

impl Movable for Queen {
    fn piece_move(
        coordinates: &Coords,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        move_history: &[(Option<PieceType>, String)],
    ) -> Vec<Coords> {
        let mut positions: Vec<Coords> = vec![];

        // Queen = bishop concat rook
        positions.extend(Bishop::piece_move(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
            move_history,
        ));
        positions.extend(Rook::piece_move(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
            move_history,
        ));

        cleaned_positions(positions)
    }
}

impl Position for Queen {
    fn authorized_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[(Option<PieceType>, String)],
        _is_king_checked: bool,
    ) -> Vec<Coords> {
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
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: &[(Option<PieceType>, String)],
    ) -> Vec<Coords> {
        Self::piece_move(coordinates, color, board, true, move_history)
    }
}

impl Queen {
    pub fn to_string() -> &'static str {
        "\
    \n\
◀█▟█▙█▶\n\
  ◥█◈█◤\n\
  ███\n\
▗█████▖\n\
    "
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, Coords},
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
            Coords::new(0, 0),
            Coords::new(0, 4),
            Coords::new(1, 1),
            Coords::new(1, 4),
            Coords::new(1, 7),
            Coords::new(2, 2),
            Coords::new(2, 4),
            Coords::new(2, 6),
            Coords::new(3, 3),
            Coords::new(3, 4),
            Coords::new(3, 5),
            Coords::new(4, 0),
            Coords::new(4, 1),
            Coords::new(4, 2),
            Coords::new(4, 3),
            Coords::new(4, 5),
            Coords::new(4, 6),
            Coords::new(4, 7),
            Coords::new(5, 3),
            Coords::new(5, 4),
            Coords::new(5, 5),
            Coords::new(6, 2),
            Coords::new(6, 4),
            Coords::new(6, 6),
            Coords::new(7, 1),
            Coords::new(7, 4),
            Coords::new(7, 7),
        ];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
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
            Coords::new(0, 0),
            Coords::new(0, 4),
            Coords::new(1, 1),
            Coords::new(1, 4),
            Coords::new(2, 2),
            Coords::new(2, 4),
            Coords::new(3, 3),
            Coords::new(3, 4),
            Coords::new(3, 5),
            Coords::new(4, 0),
            Coords::new(4, 1),
            Coords::new(4, 2),
            Coords::new(4, 3),
            Coords::new(4, 5),
            Coords::new(4, 6),
            Coords::new(4, 7),
            Coords::new(5, 3),
            Coords::new(5, 4),
            Coords::new(5, 5),
            Coords::new(6, 2),
            Coords::new(6, 4),
            Coords::new(6, 6),
            Coords::new(7, 1),
            Coords::new(7, 4),
            Coords::new(7, 7),
        ];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
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
            Coords::new(1, 7),
            Coords::new(2, 6),
            Coords::new(3, 3),
            Coords::new(3, 5),
            Coords::new(4, 1),
            Coords::new(4, 2),
            Coords::new(4, 3),
            Coords::new(4, 5),
            Coords::new(4, 6),
            Coords::new(4, 7),
            Coords::new(5, 3),
            Coords::new(5, 4),
            Coords::new(5, 5),
            Coords::new(6, 2),
            Coords::new(6, 4),
            Coords::new(7, 4),
        ];

        right_positions.sort();

        let mut positions = Queen::authorized_positions(
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
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions = vec![Coords::new(4, 4)];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coords::new(5, 5),
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
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions: Vec<Coords> = vec![];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coords::new(5, 6),
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
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions: Vec<Coords> = vec![Coords::new(2, 6), Coords::new(3, 7)];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
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
