#[cfg(test)]
mod local_game_tests {
    use chess_tui::{
        app::{App, AppResult},
        constants::Popups,
        game_logic::coord::Coord,
        handlers::handler::handle_key_events,
        ui::popup::help::render_help_popup,
    };
    use ratatui::{
        Terminal,
        backend::TestBackend,
        crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    };
    use shakmaty::{Color, Role, Square};

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
    fn m_opens_move_input_popup_and_esc_closes() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        send_keys(&mut app, &[KeyCode::Char('m')])?;
        assert_eq!(
            app.ui_state.current_popup,
            Some(Popups::MoveInputSelection),
            "'m' should open the move-input popup"
        );

        send_keys(&mut app, &[KeyCode::Esc])?;
        assert_eq!(
            app.ui_state.current_popup, None,
            "Esc should close the move-input popup"
        );

        // Cursor navigation must still work after closing the popup.
        let before = app.game.ui.cursor_coordinates;
        send_keys(&mut app, &[KeyCode::Down])?;
        assert_ne!(
            app.game.ui.cursor_coordinates, before,
            "Cursor should move normally after the popup is dismissed"
        );

        Ok(())
    }

    #[test]
    fn m_then_e4_enter_makes_move_and_closes_popup() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        send_keys(
            &mut app,
            &[
                KeyCode::Char('m'),
                KeyCode::Char('e'),
                KeyCode::Char('4'),
                KeyCode::Enter,
            ],
        )?;

        assert_eq!(
            app.ui_state.current_popup, None,
            "Popup should close after a successful move"
        );
        assert_eq!(
            app.game.logic.player_turn,
            Color::Black,
            "Turn should swap to Black"
        );
        assert_eq!(
            app.game.logic.game_board.get_role_at_square(&Square::E4),
            Some(Role::Pawn),
            "A pawn should now sit on e4"
        );
        assert!(
            app.game.ui.prompt.error.is_none(),
            "No error should be set on success"
        );

        Ok(())
    }

    #[test]
    fn invalid_san_keeps_popup_open_and_sets_error() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        send_keys(
            &mut app,
            &[
                KeyCode::Char('m'),
                KeyCode::Char('z'),
                KeyCode::Char('z'),
                KeyCode::Char('z'),
                KeyCode::Enter,
            ],
        )?;

        assert_eq!(
            app.ui_state.current_popup,
            Some(Popups::MoveInputSelection),
            "Invalid SAN should not close the popup"
        );
        assert!(
            app.game.ui.prompt.error.is_some(),
            "Invalid SAN should set an error"
        );
        assert_eq!(
            app.game.ui.prompt.input, "zzz",
            "Input must be preserved so the user can edit it"
        );
        assert_eq!(
            app.game.logic.game_board.move_history.len(),
            0,
            "No move should be applied"
        );

        Ok(())
    }

    #[test]
    fn illegal_san_keeps_popup_open_and_sets_error() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        // "e5" parses as SAN but no white pawn can reach e5 from the starting position.
        send_keys(
            &mut app,
            &[
                KeyCode::Char('m'),
                KeyCode::Char('e'),
                KeyCode::Char('5'),
                KeyCode::Enter,
            ],
        )?;

        assert_eq!(
            app.ui_state.current_popup,
            Some(Popups::MoveInputSelection),
            "Illegal SAN should not close the popup"
        );
        assert!(
            app.game.ui.prompt.error.is_some(),
            "Illegal SAN should set an error"
        );
        assert_eq!(
            app.game.logic.player_turn,
            Color::White,
            "Turn should not change on illegal input"
        );
        assert!(
            app.game
                .logic
                .game_board
                .get_role_at_square(&Square::E5)
                .is_none(),
            "e5 must remain empty"
        );

        Ok(())
    }

    #[test]
    fn typing_after_error_clears_it() -> AppResult<()> {
        let mut app = setup_local_game_start()?;

        send_keys(
            &mut app,
            &[
                KeyCode::Char('m'),
                KeyCode::Char('z'),
                KeyCode::Char('z'),
                KeyCode::Char('z'),
                KeyCode::Enter,
            ],
        )?;
        assert!(app.game.ui.prompt.error.is_some());

        send_keys(&mut app, &[KeyCode::Backspace])?;
        assert!(
            app.game.ui.prompt.error.is_none(),
            "Backspace should clear the stale error"
        );

        Ok(())
    }

    #[test]
    fn help_popup_lists_m_shortcut() -> AppResult<()> {
        let app = setup_local_game_start()?;

        // Help popup is laid out as 40% width, 65% height of the frame, with
        // wrapping. Give it ample room so the 'm' line is in the buffer.
        let backend = TestBackend::new(120, 80);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|frame| render_help_popup(frame, &app))?;

        let buffer = terminal.backend().buffer().clone();
        let mut rendered = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                rendered.push_str(buffer[(x, y)].symbol());
            }
            rendered.push('\n');
        }

        assert!(
            rendered.contains("Type a move in chess notation"),
            "Help popup should mention the 'm' shortcut.\nRendered:\n{rendered}"
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
