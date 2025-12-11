#[cfg(test)]
mod tests {
    use chess_tui::game_logic::game::{Game, GameState};

    use shakmaty::{Color, Position, Square};

    #[test]
    fn test_new_game() {
        let game = Game::default();
        assert_eq!(game.logic.player_turn, Color::White);
        assert_eq!(game.logic.game_state, GameState::Playing);
        assert!(game.logic.bot.is_none());
        assert!(game.logic.opponent.is_none());
    }

    #[test]
    fn test_switch_player_turn() {
        let mut game = Game::default();
        assert_eq!(game.logic.player_turn, Color::White);

        game.switch_player_turn();
        assert_eq!(game.logic.player_turn, Color::Black);

        game.switch_player_turn();
        assert_eq!(game.logic.player_turn, Color::White);
    }

    #[test]
    fn test_execute_move_updates_state() {
        let mut game = Game::default();

        // Move pawn e2 -> e4
        game.logic.execute_move(Square::E2, Square::E4);

        // Check history in game_board
        assert_eq!(game.logic.game_board.move_history.len(), 1);
        assert_eq!(game.logic.game_board.position_history.len(), 2);
    }

    #[test]
    fn test_checkmate_state() {
        let mut game = Game::default();

        // Fool's Mate sequence
        game.logic.execute_move(Square::F2, Square::F3);
        game.switch_player_turn();

        game.logic.execute_move(Square::E7, Square::E5);
        game.switch_player_turn();

        game.logic.execute_move(Square::G2, Square::G4);
        game.switch_player_turn();

        game.logic.execute_move(Square::D8, Square::H4);

        // Update game state
        game.handle_cell_click(None); // This triggers update_game_state internally if we were clicking, but we can call it directly or simulate the flow
                                      // Actually Game::update_game_state is private, but handle_cell_click calls it.
                                      // However, execute_move doesn't auto-update game_state.
                                      // We need to check if we can access update_game_state or if we need to simulate a click.
                                      // Looking at Game impl, update_game_state is private.
                                      // But we can check the board state directly.

        assert!(game.logic.game_board.is_checkmate());
    }

    #[test]
    fn test_sync_player_turn_with_position() {
        let mut game = Game::default();

        // Initial position: white's turn
        assert_eq!(game.logic.player_turn, Color::White);
        assert_eq!(
            game.logic.game_board.position_history[0].turn(),
            Color::White
        );

        // Make a move (white moves)
        game.logic.execute_move(Square::E2, Square::E4);
        game.logic.switch_player_turn();

        // After white's move, it's black's turn
        assert_eq!(game.logic.player_turn, Color::Black);
        // position_history[1] is after white's move, so it should be black's turn
        assert_eq!(
            game.logic.game_board.position_history[1].turn(),
            Color::Black
        );

        // Navigate to previous position (index 0, which is initial, so white's turn)
        // position_history[0] = initial (white to move)
        // position_history[1] = after move 0 (black to move)
        // When at latest (index 1), navigating previous goes to index 0
        game.logic.game_board.navigate_history_previous(true);
        assert_eq!(game.logic.game_board.history_position_index, Some(0));
        game.logic.sync_player_turn_with_position();

        // position_history[0] is initial position, so it should be white's turn
        assert_eq!(game.logic.player_turn, Color::White);

        // Can't go back further from initial position
        let success = game.logic.game_board.navigate_history_previous(true);
        assert!(!success);
        assert_eq!(game.logic.game_board.history_position_index, Some(0));
        game.logic.sync_player_turn_with_position();
        assert_eq!(game.logic.player_turn, Color::White);

        // Now navigate forward to position 1 (after move 0, black's turn)
        game.logic.game_board.navigate_history_next(true);
        assert_eq!(game.logic.game_board.history_position_index, Some(1));
        game.logic.sync_player_turn_with_position();
        assert_eq!(game.logic.player_turn, Color::Black);
    }

    #[test]
    fn test_truncate_history_when_making_move_from_history() {
        let mut game = Game::default();

        // Make several moves
        game.logic.execute_move(Square::E2, Square::E4);
        game.logic.switch_player_turn();
        game.logic.execute_move(Square::E7, Square::E5);
        game.logic.switch_player_turn();
        game.logic.execute_move(Square::G1, Square::F3);
        game.logic.switch_player_turn();

        // Should have 3 moves
        assert_eq!(game.logic.game_board.move_history.len(), 3);
        assert_eq!(game.logic.game_board.position_history.len(), 4);

        // Navigate to position after first move
        game.logic.game_board.navigate_history_previous(true);
        assert_eq!(game.logic.game_board.history_position_index, Some(2));

        // Simulate making a move from this position (truncate should happen in handle_cell_click)
        game.logic.game_board.truncate_history_at(1);

        // Should have 1 move and 2 positions
        assert_eq!(game.logic.game_board.move_history.len(), 1);
        assert_eq!(game.logic.game_board.position_history.len(), 2);
        assert_eq!(game.logic.game_board.history_position_index, None);
    }
}
