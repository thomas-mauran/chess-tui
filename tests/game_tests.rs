#[cfg(test)]
mod tests {
    use chess_tui::game_logic::game::{Game, GameState};
    use chess_tui::game_logic::game_board::GameBoard;
    use shakmaty::{Color, Square};

    #[test]
    fn test_new_game() {
        let game = Game::default();
        assert_eq!(game.player_turn, Color::White);
        assert_eq!(game.game_state, GameState::Playing);
        assert!(game.bot.is_none());
        assert!(game.opponent.is_none());
    }

    #[test]
    fn test_switch_player_turn() {
        let mut game = Game::default();
        assert_eq!(game.player_turn, Color::White);
        
        game.switch_player_turn();
        assert_eq!(game.player_turn, Color::Black);
        
        game.switch_player_turn();
        assert_eq!(game.player_turn, Color::White);
    }

    #[test]
    fn test_execute_move_updates_state() {
        let mut game = Game::default();
        
        // Move pawn e2 -> e4
        game.execute_move(Square::E2, Square::E4);
        
        // Check history in game_board
        assert_eq!(game.game_board.move_history.len(), 1);
        assert_eq!(game.game_board.position_history.len(), 2);
    }

    #[test]
    fn test_checkmate_state() {
        let mut game = Game::default();
        
        // Fool's Mate sequence
        game.execute_move(Square::F2, Square::F3);
        game.switch_player_turn();
        
        game.execute_move(Square::E7, Square::E5);
        game.switch_player_turn();
        
        game.execute_move(Square::G2, Square::G4);
        game.switch_player_turn();
        
        game.execute_move(Square::D8, Square::H4);
        
        // Update game state
        game.handle_cell_click(); // This triggers update_game_state internally if we were clicking, but we can call it directly or simulate the flow
        // Actually Game::update_game_state is private, but handle_cell_click calls it. 
        // However, execute_move doesn't auto-update game_state. 
        // We need to check if we can access update_game_state or if we need to simulate a click.
        // Looking at Game impl, update_game_state is private.
        // But we can check the board state directly.
        
        assert!(game.game_board.is_checkmate());
    }
}
