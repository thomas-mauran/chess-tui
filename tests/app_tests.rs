#[cfg(test)]
mod local_game_tests {
    use chess_tui::{
        app::{App, AppResult},
        game_logic::coord::Coord,
        handler::handle_key_events,
    };
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
    use shakmaty::{Color, Square};

    fn key_press(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }
    }

    fn send_keys(app: &mut App, keys: &[KeyCode]) -> AppResult<()> {
        for &key in keys {
            handle_key_events(key_press(key), app)?;
        }
        Ok(())
    }

    fn setup_local_game_start() -> AppResult<App> {
        let mut app = App::default();

        send_keys(
            &mut app,
            &[
                KeyCode::Enter, // Play Game -> GameModeMenu
                KeyCode::Enter, // Select 'Local' -> Activates Form
                KeyCode::Enter, // Submit Form -> Starts Game
            ],
        )?;

        Ok(app)
    }

    #[test]
    fn test_valid_first_move_e2_to_e4() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        // Cursor starts at (4, 4) which is e4.
        // Navigate to e2 (4, 6) by going down 2 times.
        let nav_to_e2 = vec![KeyCode::Down, KeyCode::Down];
        send_keys(&mut app, &nav_to_e2)?;

        // Select the e2 pawn
        send_keys(&mut app, &[KeyCode::Enter])?;
        assert_eq!(
            app.game.ui.selected_square,
            Some(Square::E2),
            "The e2 pawn should be selected"
        );

        // Should not change the cursor position when select a new piece.
        assert_eq!(app.game.ui.cursor_coordinates, Coord::new(6, 4));

        // Navigate from e2 to e4 (2 steps up)
        send_keys(&mut app, &[KeyCode::Up, KeyCode::Up])?;

        // Confirm the move
        send_keys(&mut app, &[KeyCode::Enter])?;

        // Verify the game state successfully updated
        assert!(
            app.game.ui.selected_square.is_none(),
            "Selection should clear after making a valid move"
        );
        assert_eq!(
            app.game.logic.game_board.move_history.len(),
            1,
            "One move should be recorded in the history"
        );
        assert_eq!(
            app.game.logic.player_turn,
            Color::Black,
            "The turn should have swapped to Black"
        );

        Ok(())
    }

    #[test]
    fn test_invalid_first_move_is_rejected() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        // Cursor starts at (4, 4). Navigate to e2 pawn (4, 6)
        let nav_to_e2 = vec![KeyCode::Down, KeyCode::Down];
        send_keys(&mut app, &nav_to_e2)?;

        // Select the pawn
        send_keys(&mut app, &[KeyCode::Enter])?;
        assert!(
            app.game.ui.selected_square.is_some(),
            "The e2 pawn should be selected"
        );

        // Attempt an illegal move: e2 to e5 (3 steps up)
        send_keys(&mut app, &[KeyCode::Up, KeyCode::Up, KeyCode::Up])?;
        send_keys(&mut app, &[KeyCode::Enter])?;

        // Verify the move was rejected and state remains untouched
        assert_eq!(
            app.game.logic.game_board.move_history.len(),
            0,
            "No move should be recorded for an illegal destination"
        );
        assert_eq!(
            app.game.logic.player_turn,
            Color::White,
            "It should still be White's turn"
        );

        Ok(())
    }

    #[test]
    fn test_deselect_piece_with_keyboard() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        // Cursor starts at (4, 4). Navigate to e2 and select it
        let nav_to_e2 = vec![KeyCode::Down, KeyCode::Down];
        send_keys(&mut app, &nav_to_e2)?;
        send_keys(&mut app, &[KeyCode::Enter])?;

        assert!(app.game.ui.selected_square.is_some());

        // Press Enter again on the exact same square to deselect
        send_keys(&mut app, &[KeyCode::Enter])?;

        assert!(
            app.game.ui.selected_square.is_none(),
            "Pressing enter on the currently selected piece should deselect it"
        );
        assert_eq!(app.game.logic.game_board.move_history.len(), 0);

        Ok(())
    }
}
