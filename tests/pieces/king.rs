#[cfg(test)]
mod tests {
    use chess_tui::game_logic::coord::Coord;
    use chess_tui::game_logic::game::Game;
    use chess_tui::game_logic::game_board::GameBoard;
    use chess_tui::pieces::king::King;
    use chess_tui::pieces::{PieceColor, PieceMove, PieceType, Position};

    // Test with multiple enemies around the king
    #[test]
    fn multiple_enemies_1() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
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
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
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

        let mut right_positions = vec![Coord::new(4, 5), Coord::new(5, 4)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            &game.game_board,
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
                Some((PieceType::Queen, PieceColor::Black)),
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
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
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
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

        let mut right_positions = vec![Coord::new(3, 4)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            &game.game_board,
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
                Some((PieceType::Rook, PieceColor::Black)),
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
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::Black)),
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
            [
                None,
                None,
                Some((PieceType::Bishop, PieceColor::Black)),
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

        let mut right_positions = vec![Coord::new(4, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            &game.game_board,
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    // Test for the castling
    #[test]
    fn big_castle_white() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
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
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;

        let mut right_positions = vec![Coord::new(7, 3), Coord::new(7, 0)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(7, 4),
            PieceColor::White,
            &game.game_board,
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn small_castle_white() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
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
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;

        let mut right_positions = vec![Coord::new(7, 5), Coord::new(7, 7)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(7, 4),
            PieceColor::White,
            &game.game_board,
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;

        let mut right_positions = vec![Coord::new(7, 4), Coord::new(7, 7)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(7, 3),
            PieceColor::Black,
            &game.game_board,
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
    #[test]
    fn small_castle_black() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
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
            [
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Rook, PieceColor::Black)),
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;

        let mut right_positions = vec![Coord::new(7, 0), Coord::new(7, 2)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(7, 3),
            PieceColor::Black,
            &game.game_board,
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_check_blocking() {
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
                None,
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
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
                None,
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;

        let mut right_positions = vec![Coord::new(0, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(0, 4),
            PieceColor::Black,
            &game.game_board,
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_king_checked() {
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

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut right_positions = vec![Coord::new(0, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(0, 4),
            PieceColor::Black,
            &game.game_board,
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn big_castle_black_rook_already_moved() {
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
                None,
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
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;
        game.game_board.move_history = [
            (PieceMove {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::White,
                from: Coord::new(0, 7),
                to: Coord::new(4, 7),
            }),
            (PieceMove {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::Black,
                from: Coord::new(6, 2),
                to: Coord::new(5, 2),
            }),
            (PieceMove {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::White,
                from: Coord::new(4, 7),
                to: Coord::new(0, 7),
            }),
        ]
        .to_vec();

        let mut right_positions = vec![Coord::new(0, 5)];
        right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(0, 4),
            PieceColor::Black,
            &game.game_board,
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn castle_both_sides() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
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
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
            ],
        ];

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut white_right_positions = vec![Coord::new(7, 5), Coord::new(7, 7)];
        white_right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(7, 4),
            PieceColor::White,
            &game.game_board,
            is_king_checked,
        );
        positions.sort();

        assert_eq!(white_right_positions, positions);

        game.move_piece_on_the_board(&Coord::new(7, 4), &Coord::new(7, 7));
        game.game_board.flip_the_board();

        let mut black_right_positions = vec![Coord::new(7, 4), Coord::new(7, 7)];
        black_right_positions.sort();

        let mut positions = King::authorized_positions(
            &Coord::new(7, 3),
            PieceColor::Black,
            &game.game_board,
            is_king_checked,
        );
        positions.sort();

        assert_eq!(black_right_positions, positions);
    }
}
