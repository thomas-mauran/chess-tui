#[cfg(test)]
mod tests {
    use chess_tui::game::coord::Coord;
    use chess_tui::game::game::Game;
    use chess_tui::game::game_board::GameBoard;
    use chess_tui::pieces::knight::Knight;
    use chess_tui::pieces::{PieceColor, PieceType, Position};

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
        let mut game = Game::default();
        game.game_board.board = custom_board;

        let mut right_positions = vec![
            Coord::new(2, 3),
            Coord::new(2, 5),
            Coord::new(3, 2),
            Coord::new(3, 6),
            Coord::new(5, 2),
            Coord::new(5, 6),
            Coord::new(6, 3),
            Coord::new(6, 5),
        ];
        right_positions.sort();

        let mut positions = Knight::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            &game.game_board,
            false,
        );
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
        let mut game = Game::default();
        game.game_board.board = custom_board;

        let mut right_positions = vec![Coord::new(6, 5)];
        right_positions.sort();

        let mut positions = Knight::authorized_positions(
            &Coord::new(7, 7),
            PieceColor::White,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut right_positions = vec![Coord::new(7, 7)];
        right_positions.sort();

        let mut positions = Knight::authorized_positions(
            &Coord::new(6, 5),
            PieceColor::White,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Knight::authorized_positions(
            &Coord::new(6, 4),
            PieceColor::White,
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

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = game
            .game_board
            .is_getting_checked(game.game_board.board, game.player_turn);

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Knight::authorized_positions(
            &Coord::new(1, 4),
            PieceColor::Black,
            &game.game_board,
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
