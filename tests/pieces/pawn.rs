#[cfg(test)]
mod tests {
    use chess_tui::game_logic::coord::Coord;
    use chess_tui::game_logic::game::Game;
    use chess_tui::game_logic::game_board::GameBoard;
    use chess_tui::pieces::pawn::Pawn;
    use chess_tui::pieces::{PieceColor, PieceMove, PieceType, Position};

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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        let mut right_positions = vec![Coord::new(3, 4)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            &game.game_board,
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        let mut right_positions = vec![Coord::new(5, 4), Coord::new(4, 4)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(6, 4),
            PieceColor::White,
            &game.game_board,
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
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
            &game.game_board,
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        let mut right_positions = vec![Coord::new(5, 4), Coord::new(5, 2)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(6, 3),
            PieceColor::Black,
            &game.game_board,
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        // First pawn on the left
        let mut right_positions_first_pawn = vec![Coord::new(1, 2), Coord::new(1, 3)];
        right_positions_first_pawn.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(2, 2),
            PieceColor::White,
            &game.game_board,
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
            &game.game_board,
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
            &game.game_board,
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        game.game_board.move_history = [(PieceMove {
            piece_type: PieceType::Pawn,
            piece_color: PieceColor::Black,
            from: Coord::new(6, 4),
            to: Coord::new(4, 4),
        })]
        .to_vec();

        let mut right_positions = vec![Coord::new(2, 2), Coord::new(2, 3)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(3, 2),
            PieceColor::White,
            &game.game_board,
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);

        // Perform the en passant move
        game.move_piece_on_the_board(&Coord { row: 3, col: 2 }, &Coord { row: 2, col: 3 });

        let board_after_move = [
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        assert_eq!(board_after_move, game.game_board.board);
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        game.game_board.move_history = [(PieceMove {
            piece_type: PieceType::Pawn,
            piece_color: PieceColor::White,
            from: Coord::new(6, 4),
            to: Coord::new(4, 4),
        })]
        .to_vec();

        let mut right_positions = vec![Coord::new(2, 2), Coord::new(2, 3)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(3, 2),
            PieceColor::Black,
            &game.game_board,
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);

        game.move_piece_on_the_board(&Coord { row: 3, col: 2 }, &Coord { row: 2, col: 3 });

        let board_after_move = [
            [None, None, None, None, None, None, None, None],
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        assert_eq!(board_after_move, game.game_board.board);
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        game.game_board.move_history = [(PieceMove {
            piece_type: PieceType::Pawn,
            piece_color: PieceColor::White,
            from: Coord::new(6, 4),
            to: Coord::new(4, 4),
        })]
        .to_vec();

        let mut right_positions = vec![Coord::new(5, 1), Coord::new(4, 1)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(6, 1),
            PieceColor::Black,
            &game.game_board,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut right_positions = vec![Coord::new(2, 2)];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(3, 3),
            PieceColor::Black,
            &game.game_board,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(2, 4),
            PieceColor::Black,
            &game.game_board,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            &Coord::new(1, 5),
            PieceColor::Black,
            &game.game_board,
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
