#[cfg(test)]
mod tests {
    use chess_tui::game_logic::coord::Coord;
    use chess_tui::game_logic::game::Game;
    use chess_tui::game_logic::game_board::GameBoard;
    use chess_tui::pieces::{PieceColor, PieceMove, PieceType};
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
        let game_board = GameBoard::new(
            custom_board,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    from: Coord::new(1, 4),
                    to: Coord::new(0, 4),
                }),
            ],
            vec![],
        );
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        assert!(game.game_board.is_latest_move_promotion());
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
        let game_board = GameBoard::new(
            custom_board,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::White,
                    from: Coord::new(7, 3),
                    to: Coord::new(6, 3),
                }),
            ],
            vec![],
        );
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        assert!(!game.game_board.is_latest_move_promotion());
    }

    #[tokio::test]
    async fn promote_and_checkmate() {
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
        // We setup the game
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::White);
        game.game_board.board = custom_board;

        // Move the pawn to a promote cell
        game.execute_move(&Coord::new(1, 4), &Coord::new(0, 4)).await;
        assert!(game.game_board.is_latest_move_promotion());

        // Promote the pawn
        game.promote_piece().await;

        // The black king gets checkmated
        game.player_turn = PieceColor::Black;
        assert!(game.game_board.is_checkmate(game.player_turn).await);
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
        let game_board = GameBoard::new(
            custom_board,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: PieceColor::Black,
                    from: Coord::new(1, 4),
                    to: Coord::new(0, 4),
                }),
            ],
            vec![],
        );
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        assert!(game.game_board.is_latest_move_promotion());
    }

    #[tokio::test]
    async fn promote_and_draw() {
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
        // We setup the game
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, PieceColor::Black);
        game.game_board.board = custom_board;

        // Move the pawn to a promote cell
        game.execute_move(&Coord::new(1, 5), &Coord::new(0, 5)).await;
        assert!(game.game_board.is_latest_move_promotion());

        // Promote the pawn
        game.promote_piece().await;

        // The black king gets checkmated
        game.player_turn = PieceColor::White;
        assert!(game.game_board.is_draw(game.player_turn).await);
    }
}
