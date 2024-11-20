#[cfg(test)]
mod tests {
    use chess_tui::board::{Board, Coord};
    use chess_tui::pieces::{PieceColor, PieceMove, PieceType};
    use chess_tui::utils::is_getting_checked;

    #[test]
    fn is_getting_checked_true() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(is_getting_checked(custom_board, PieceColor::White, &[]));
    }

    #[test]
    fn is_getting_checked_false() {
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::White, &[]));
    }

    #[test]
    fn is_getting_checked_piece_in_front_false() {
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
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::Black, &[]));
    }

    #[test]
    fn is_getting_checked_piece_in_with_gap_false() {
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
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
            ],
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
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::Black, &[]));
    }

    #[test]
    fn is_checkmate_true() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
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
        let board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(board.is_checkmate());
    }

    #[test]
    fn is_checkmate_false() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Queen, PieceColor::Black)),
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
        let board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(!board.is_checkmate());
    }

    #[test]
    fn is_checkmate_false_2() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
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
                Some((PieceType::Queen, PieceColor::White)),
                None,
            ],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
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
        let board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(!board.is_checkmate());
    }

    #[test]
    fn is_draw_true() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
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
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(board.is_draw());
    }

    #[test]
    fn is_draw_false() {
        let custom_board = [
            [
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
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
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);

        assert!(!board.is_draw());
    }

    #[test]
    fn is_promote_false() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
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
            [
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];
        let board = Board::new(
            custom_board,
            PieceColor::Black,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    from: Coord::new(7, 3),
                    to: Coord::new(6, 3),
                }),
            ],
        );

        assert!(!board.is_latest_move_promotion());
    }
    #[test]
    fn is_promote_true() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];
        let board = Board::new(
            custom_board,
            PieceColor::Black,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    from: Coord::new(1, 4),
                    to: Coord::new(0, 4),
                }),
            ],
        );

        assert!(board.is_latest_move_promotion());
    }

    #[test]
    fn promote_and_checkmate() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
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
            [
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        ];
        // We setup the board
        let mut board = Board::new(custom_board, PieceColor::White, vec![]);
        assert!(!board.is_latest_move_promotion());

        // Move the pawn to a promote cell
        board.move_piece_on_the_board(&Coord::new(1, 4), &Coord::new(0, 4));
        assert!(board.is_latest_move_promotion());

        // Promote the pawn
        board.promote_piece();

        // The black king gets checkmated
        board.player_turn = PieceColor::Black;
        assert!(board.is_checkmate());
    }

    #[test]
    fn is_promote_true_black() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
            [
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
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
            [None, None, None, None, None, None, None, None],
        ];
        let board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::Black,
                    from: Coord::new(1, 4),
                    to: Coord::new(0, 4),
                }),
            ],
        );

        assert!(board.is_latest_move_promotion());
    }

    #[test]
    fn promote_and_draw() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
            [
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
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
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the board
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        assert!(!board.is_latest_move_promotion());

        // Move the pawn to a promote cell
        board.move_piece_on_the_board(&Coord::new(1, 5), &Coord::new(0, 5));
        assert!(board.is_latest_move_promotion());

        // Promote the pawn
        board.promote_piece();

        // The black king gets checkmated
        board.player_turn = PieceColor::White;
        assert!(board.is_draw());
    }
    #[test]
    fn fifty_moves_draw() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the board
        let mut board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                // We don't use the history for a fifty draw
            ],
        );

        board.consecutive_non_pawn_or_capture = 49;
        assert!(!board.is_draw());

        // Move the pawn to a make the 50th move
        board.move_piece_on_the_board(&Coord::new(1, 6), &Coord::new(1, 5));
        assert!(board.is_draw());
    }

    #[test]
    fn consecutive_position_draw() {
        let custom_board = [
            [
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        // We setup the board
        let mut board = Board::new(
            custom_board,
            PieceColor::White,
            vec![
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 2),
                    to: Coord::new(0, 1),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 6),
                    to: Coord::new(0, 5),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 1),
                    to: Coord::new(0, 2),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 5),
                    to: Coord::new(0, 6),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 2),
                    to: Coord::new(0, 1),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 6),
                    to: Coord::new(0, 5),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::White,
                    from: Coord::new(0, 1),
                    to: Coord::new(0, 2),
                }),
                (PieceMove {
                    piece_type: PieceType::King,
                    piece_color: PieceColor::Black,
                    from: Coord::new(0, 5),
                    to: Coord::new(0, 6),
                }),
            ],
        );

        let mut copy_move_history = board.move_history.clone();

        for piece_move in copy_move_history.iter_mut() {
            board.move_piece_on_the_board(&piece_move.from, &piece_move.to);

            // In a chess game, board.is_draw() is called after every move
            assert!(!board.is_draw());
        }

        // Move the king to replicate a third time the same position
        board.move_piece_on_the_board(&Coord::new(0, 2), &Coord::new(0, 1));
        assert!(board.is_draw());
    }
}
