#[cfg(test)]
mod tests {
    use chess_tui::game_logic::coord::Coord;
    use chess_tui::game_logic::game::Game;
    use chess_tui::game_logic::game_board::GameBoard;
    use chess_tui::pieces::{PieceMove, PieceType};
    use shakmaty::Color;

    #[test]
    fn fen_converter_1() {
        let custom_board = [
            [
                None,
                None,
                Some((PieceType::King, Color::Black)),
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, Color::White)),
            ],
            [None, None, None, None, None, None, None, None],
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
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the game
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, Color::White);
        game.game_board.board = custom_board;

        // Move the king to replicate a third time the same position
        assert_eq!(
            game.game_board.fen_position(false, game.player_turn),
            "2k4R/8/4K3/8/8/8/8/8 b - - 0 1"
        );
    }

    #[test]
    fn fen_converter_en_passant() {
        let custom_board = [
            [
                None,
                None,
                Some((PieceType::King, Color::Black)),
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, Color::White)),
            ],
            [None, None, None, None, None, None, None, None],
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
            [
                None,
                None,
                Some((PieceType::Pawn, Color::White)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        // We setup the game
        let game_board = GameBoard::new(
            custom_board,
            vec![
                (PieceMove {
                    piece_type: PieceType::Pawn,
                    piece_color: Color::White,
                    from: Coord::new(6, 2),
                    to: Coord::new(4, 2),
                }),
            ],
            vec![],
        );
        let mut game = Game::new(game_board, Color::White);
        game.game_board.board = custom_board;

        // Move the king to replicate a third time the same position
        assert_eq!(
            game.game_board.fen_position(false, game.player_turn),
            "2k4R/8/4K3/8/2P5/8/8/8 b - c3 0 1"
        );
    }
    #[test]
    fn fen_converter_castling() {
        let custom_board = [
            [
                Some((PieceType::Rook, Color::Black)),
                Some((PieceType::Knight, Color::Black)),
                Some((PieceType::Bishop, Color::Black)),
                Some((PieceType::Queen, Color::Black)),
                Some((PieceType::King, Color::Black)),
                Some((PieceType::Bishop, Color::Black)),
                Some((PieceType::Knight, Color::Black)),
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
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
                Some((PieceType::Pawn, Color::White)),
            ],
            [
                Some((PieceType::Rook, Color::White)),
                Some((PieceType::Knight, Color::White)),
                Some((PieceType::Bishop, Color::White)),
                Some((PieceType::Queen, Color::White)),
                Some((PieceType::King, Color::White)),
                Some((PieceType::Bishop, Color::White)),
                Some((PieceType::Knight, Color::White)),
                Some((PieceType::Rook, Color::White)),
            ],
        ];
        // We setup the game
        let game_board = GameBoard::new(custom_board, vec![], vec![]);
        let mut game = Game::new(game_board, Color::White);
        game.game_board.board = custom_board;

        // Move the king to replicate a third time the same position
        assert_eq!(
            game.game_board.fen_position(false, game.player_turn),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1"
        );
    }
}
