#[cfg(test)]
mod tests {
    use chess_tui::game::coord::Coord;
    use chess_tui::game::game::Game;
    use chess_tui::game::game_board::GameBoard;
    use chess_tui::pieces::queen::Queen;
    use chess_tui::pieces::{PieceColor, PieceType, Position};
    use chess_tui::utils::is_getting_checked;

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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        let mut right_positions = vec![
            Coord::new(0, 0),
            Coord::new(0, 4),
            Coord::new(1, 1),
            Coord::new(1, 4),
            Coord::new(1, 7),
            Coord::new(2, 2),
            Coord::new(2, 4),
            Coord::new(2, 6),
            Coord::new(3, 3),
            Coord::new(3, 4),
            Coord::new(3, 5),
            Coord::new(4, 0),
            Coord::new(4, 1),
            Coord::new(4, 2),
            Coord::new(4, 3),
            Coord::new(4, 5),
            Coord::new(4, 6),
            Coord::new(4, 7),
            Coord::new(5, 3),
            Coord::new(5, 4),
            Coord::new(5, 5),
            Coord::new(6, 2),
            Coord::new(6, 4),
            Coord::new(6, 6),
            Coord::new(7, 1),
            Coord::new(7, 4),
            Coord::new(7, 7),
        ];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            game.game_board.board,
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        let mut right_positions = vec![
            Coord::new(0, 0),
            Coord::new(0, 4),
            Coord::new(1, 1),
            Coord::new(1, 4),
            Coord::new(2, 2),
            Coord::new(2, 4),
            Coord::new(3, 3),
            Coord::new(3, 4),
            Coord::new(3, 5),
            Coord::new(4, 0),
            Coord::new(4, 1),
            Coord::new(4, 2),
            Coord::new(4, 3),
            Coord::new(4, 5),
            Coord::new(4, 6),
            Coord::new(4, 7),
            Coord::new(5, 3),
            Coord::new(5, 4),
            Coord::new(5, 5),
            Coord::new(6, 2),
            Coord::new(6, 4),
            Coord::new(6, 6),
            Coord::new(7, 1),
            Coord::new(7, 4),
            Coord::new(7, 7),
        ];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            game.game_board.board,
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
        let mut game = Game::default();
        game.game_board.board = custom_board;
        let mut right_positions = vec![
            Coord::new(1, 7),
            Coord::new(2, 6),
            Coord::new(3, 3),
            Coord::new(3, 5),
            Coord::new(4, 1),
            Coord::new(4, 2),
            Coord::new(4, 3),
            Coord::new(4, 5),
            Coord::new(4, 6),
            Coord::new(4, 7),
            Coord::new(5, 3),
            Coord::new(5, 4),
            Coord::new(5, 5),
            Coord::new(6, 2),
            Coord::new(6, 4),
            Coord::new(7, 4),
        ];

        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            game.game_board.board,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = is_getting_checked(
            game.game_board.board,
            game.player_turn,
            &game.game_board.move_history,
        );

        let mut right_positions = vec![Coord::new(4, 4)];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coord::new(5, 5),
            PieceColor::Black,
            game.game_board.board,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = is_getting_checked(
            game.game_board.board,
            game.player_turn,
            &game.game_board.move_history,
        );

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coord::new(5, 6),
            PieceColor::Black,
            game.game_board.board,
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
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        let is_king_checked = is_getting_checked(
            game.game_board.board,
            game.player_turn,
            &game.game_board.move_history,
        );

        let mut right_positions = vec![Coord::new(2, 6), Coord::new(3, 7)];
        right_positions.sort();

        let mut positions = Queen::authorized_positions(
            &Coord::new(1, 5),
            PieceColor::Black,
            game.game_board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
