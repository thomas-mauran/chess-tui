#[cfg(test)]
mod tests {
    use chess_tui::board::{Board, Coord};
    use chess_tui::pieces::pawn::Pawn;
    use chess_tui::pieces::{PieceColor, PieceMove, PieceType, Position};
    use chess_tui::utils::is_getting_checked;

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

        let mut right_positions = vec![Coord::new(3, 4)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            board.board,
            &[],
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

        let mut right_positions = vec![Coord::new(5, 4), Coord::new(4, 4)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(6, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_one_cell_enemy_left_right() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
            Coord::new(5, 3),
            Coord::new(4, 3),
            Coord::new(5, 4),
            Coord::new(5, 2),
        ];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(6, 3),
            PieceColor::Black,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_one_pawn_3_enemies() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coord::new(5, 4), Coord::new(5, 2)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(6, 3),
            PieceColor::Black,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_3_enemies_one_pawn() {
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

        // First pawn on the left
        let mut right_positions_first_pawn = vec![Coord::new(1, 2), Coord::new(1, 3)];
        right_positions_first_pawn.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(2, 2),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions_first_pawn, positions);

        // Middle pawn
        let mut right_positions_second_pawn: Vec<Coord> = vec![];
        right_positions_second_pawn.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(2, 3),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions_second_pawn, positions);

        // Third pawn on the right
        let mut right_positions_third_pawn: Vec<Coord> = vec![Coord::new(1, 3), Coord::new(1, 4)];
        right_positions_third_pawn.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(2, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions_third_pawn, positions);
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
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::Black)),
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

        let mut right_positions = vec![Coord::new(2, 2), Coord::new(2, 3)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(3, 2),
            PieceColor::White,
            board.board,
            &[(PieceMove {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::Black,
                from: Coord::new(6, 4),
                to: Coord::new(4, 4),
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

        let mut right_positions = vec![Coord::new(2, 2), Coord::new(2, 3)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(3, 2),
            PieceColor::Black,
            board.board,
            &[(PieceMove {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::White,
                from: Coord::new(6, 4),
                to: Coord::new(4, 4),
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
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![Coord::new(5, 1), Coord::new(4, 1)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(6, 1),
            PieceColor::Black,
            board.board,
            &[(PieceMove {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::White,
                from: Coord::new(6, 4),
                to: Coord::new(4, 4),
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
                Some((PieceType::Queen, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
            ],
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions = vec![Coord::new(2, 2)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(3, 3),
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

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(2, 4),
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

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(1, 5),
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
