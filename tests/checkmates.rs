#[cfg(test)]
mod tests {
    use chess_tui::game_logic::game::Game;
    use chess_tui::game_logic::game_board::GameBoard;
    use chess_tui::pieces::PieceType;
    use shakmaty::Color;

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
                Some((PieceType::Rook, Color::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, Color::White)),
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
        assert!(game
            .game_board
            .is_getting_checked(custom_board, Color::White));
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
                Some((PieceType::Rook, Color::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, Color::White)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, Color::White)),
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
        assert!(!game
            .game_board
            .is_getting_checked(custom_board, Color::White));
    }

    #[test]
    fn is_getting_checked_piece_in_front_false() {
        let custom_board = [
            [
                Some((PieceType::Rook, Color::Black)),
                Some((PieceType::Knight, Color::Black)),
                Some((PieceType::Bishop, Color::Black)),
                Some((PieceType::Queen, Color::Black)),
                Some((PieceType::King, Color::Black)),
                None,
                None,
                Some((PieceType::Rook, Color::Black)),
            ],
            [
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                None,
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
            ],
            [
                Some((PieceType::Rook, Color::White)),
                Some((PieceType::Knight, Color::White)),
                Some((PieceType::Bishop, Color::White)),
                Some((PieceType::Queen, Color::White)),
                Some((PieceType::Rook, Color::White)),
                Some((PieceType::Bishop, Color::White)),
                None,
                Some((PieceType::King, Color::White)),
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;
        assert!(!game
            .game_board
            .is_getting_checked(custom_board, Color::Black));
    }

    #[test]
    fn is_getting_checked_piece_in_with_gap_false() {
        let custom_board = [
            [
                Some((PieceType::Rook, Color::Black)),
                Some((PieceType::Knight, Color::Black)),
                Some((PieceType::Bishop, Color::Black)),
                Some((PieceType::Queen, Color::Black)),
                Some((PieceType::King, Color::Black)),
                None,
                None,
                Some((PieceType::Rook, Color::Black)),
            ],
            [
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                None,
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
                Some((PieceType::Pawn, Color::Black)),
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, Color::Black)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                None,
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
            ],
            [
                Some((PieceType::Rook, Color::White)),
                Some((PieceType::Knight, Color::White)),
                Some((PieceType::Bishop, Color::White)),
                Some((PieceType::Queen, Color::White)),
                Some((PieceType::Rook, Color::White)),
                Some((PieceType::Bishop, Color::White)),
                None,
                Some((PieceType::King, Color::White)),
            ],
        ];
        let mut game = Game::default();
        game.game_board.board = custom_board;
        assert!(!game
            .game_board
            .is_getting_checked(custom_board, Color::Black));
    }

    #[test]
    fn is_checkmate_true() {
        let custom_board = [
            [
                Some((PieceType::King, Color::White)),
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
                Some((PieceType::Rook, Color::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                Some((PieceType::Queen, Color::Black)),
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

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, Color::White);
        game.game_board.board = custom_board;

        assert!(game.game_board.is_checkmate(game.player_turn));
    }

    #[test]
    fn is_checkmate_false() {
        let custom_board = [
            [
                Some((PieceType::King, Color::White)),
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
                Some((PieceType::Rook, Color::Black)),
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
                Some((PieceType::Queen, Color::Black)),
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
        let mut game = Game::new(game_board, Color::White);
        game.game_board.board = custom_board;

        assert!(!game.game_board.is_checkmate(game.player_turn));
    }

    #[test]
    fn is_checkmate_false_2() {
        let custom_board = [
            [
                Some((PieceType::King, Color::White)),
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
                Some((PieceType::Queen, Color::White)),
                None,
            ],
            [
                None,
                Some((PieceType::Rook, Color::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                Some((PieceType::Queen, Color::Black)),
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

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, Color::White);
        game.game_board.board = custom_board;

        assert!(!game.game_board.is_checkmate(game.player_turn));
    }
}
