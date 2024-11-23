#[cfg(test)]
mod tests {
    use chess_tui::game::coord::Coord;
    use chess_tui::game::game::Game;
    use chess_tui::game::game_board::GameBoard;
    use chess_tui::pieces::{PieceColor, PieceMove, PieceType};
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

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        assert!(game.is_draw());
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

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        assert!(!game.is_draw());
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
        // We setup the game

        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        game.consecutive_non_pawn_or_capture = 49;
        assert!(!game.is_draw());

        // Move the pawn to a make the 50th move
        game.move_piece_on_the_board(&Coord::new(1, 6), &Coord::new(1, 5));
        assert!(game.is_draw());
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

        // We setup the game

        let game_board = GameBoard::new(
            custom_board,
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
            vec![],
        );
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        let mut copy_move_history = game.game_board.move_history.clone();

        for piece_move in copy_move_history.iter_mut() {
            game.move_piece_on_the_board(&piece_move.from, &piece_move.to);

            // In a chess game, board.is_draw() is called after every move
            assert!(!game.is_draw());
        }

        // Move the king to replicate a third time the same position
        game.move_piece_on_the_board(&Coord::new(0, 2), &Coord::new(0, 1));
        assert!(game.is_draw());
    }
}
