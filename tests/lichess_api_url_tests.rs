#[cfg(test)]
mod lichess_api_url_tests {
    use chess_tui::{
        app::{App, AppResult},
        config::{Args, Config},
        constants::{DEFAULT_LICHESS_API_URL, Popups},
        handlers::handler::handle_key_events,
        ui::popup::lichess::{
            api_url::render_enter_lichess_api_url_popup, token::render_enter_lichess_token_popup,
        },
    };
    use ratatui::{
        Terminal,
        backend::TestBackend,
        crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    };

    fn key_press(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::empty(),
        }
    }

    fn render<F>(draw: F) -> AppResult<String>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(draw)?;

        let buffer = terminal.backend().buffer().clone();
        let mut rendered = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                rendered.push_str(buffer[(x, y)].symbol());
            }
            rendered.push('\n');
        }
        Ok(rendered)
    }

    #[test]
    fn token_popup_shows_the_api_url_and_the_tab_hint() -> AppResult<()> {
        let app = App::default();
        let rendered =
            render(|frame| render_enter_lichess_token_popup(frame, &app.game.ui.prompt))?;

        assert!(
            rendered.contains(DEFAULT_LICHESS_API_URL),
            "Token popup should show the API URL in effect.\nRendered:\n{rendered}"
        );
        assert!(
            rendered.contains("Tab"),
            "Token popup should mention the `Tab` shortcut.\nRendered:\n{rendered}"
        );
        Ok(())
    }

    #[test]
    fn tab_on_the_token_popup_opens_a_prefilled_api_url_popup() -> AppResult<()> {
        let mut app = App::default();
        app.ui_state.current_popup = Some(Popups::EnterLichessToken);
        app.game.ui.prompt.set_input("half-typed-token");

        handle_key_events(key_press(KeyCode::Tab), &mut app)?;

        assert_eq!(app.ui_state.current_popup, Some(Popups::EnterLichessApiUrl));
        assert_eq!(app.game.ui.prompt.input, DEFAULT_LICHESS_API_URL);

        let rendered =
            render(|frame| render_enter_lichess_api_url_popup(frame, &app.game.ui.prompt))?;
        assert!(
            rendered.contains(DEFAULT_LICHESS_API_URL),
            "API URL popup should be pre-filled with the current URL.\nRendered:\n{rendered}"
        );
        Ok(())
    }

    #[test]
    fn cancelling_restores_the_token_popup_and_its_input() -> AppResult<()> {
        let mut app = App::default();
        app.ui_state.current_popup = Some(Popups::EnterLichessToken);
        app.game.ui.prompt.set_input("half-typed-token");

        handle_key_events(key_press(KeyCode::Tab), &mut app)?;
        handle_key_events(key_press(KeyCode::Esc), &mut app)?;

        assert_eq!(app.ui_state.current_popup, Some(Popups::EnterLichessToken));
        assert_eq!(app.game.ui.prompt.input, "half-typed-token");
        Ok(())
    }

    #[test]
    fn a_url_without_a_scheme_is_rejected_inline() -> AppResult<()> {
        let mut app = App::default();
        app.open_lichess_api_url_popup(None);
        app.game.ui.prompt.set_input("lichess.dev/api");

        handle_key_events(key_press(KeyCode::Enter), &mut app)?;

        assert_eq!(
            app.ui_state.current_popup,
            Some(Popups::EnterLichessApiUrl),
            "Popup should stay open so the user can fix the URL"
        );
        assert!(app.game.ui.prompt.error.is_some());
        Ok(())
    }

    fn args_with_api_url(api_url: Option<&str>) -> Args {
        Args {
            engine_path: String::new(),
            depth: None,
            difficulty: None,
            lichess_token: None,
            lichess_api_url: api_url.map(str::to_string),
            no_sound: false,
            skin: None,
            update_skins: false,
            pgn: None,
        }
    }

    #[test]
    fn the_cli_flag_is_persisted_and_normalized() -> AppResult<()> {
        let folder = std::env::temp_dir().join("chess-tui-api-url-flag-test");
        let config_path = folder.join("config.toml");
        let _ = std::fs::remove_dir_all(&folder);

        Config::config_create(
            &args_with_api_url(Some(" https://lichess.dev/api/ ")),
            &folder,
            &config_path,
        )?;

        let written: Config = toml::from_str(&std::fs::read_to_string(&config_path)?)?;
        assert_eq!(
            written.lichess_api_url.as_deref(),
            Some("https://lichess.dev/api")
        );

        // Omitting the flag must not clobber the value a previous run stored.
        Config::config_create(&args_with_api_url(None), &folder, &config_path)?;
        let written: Config = toml::from_str(&std::fs::read_to_string(&config_path)?)?;
        assert_eq!(
            written.lichess_api_url.as_deref(),
            Some("https://lichess.dev/api")
        );

        std::fs::remove_dir_all(&folder)?;
        Ok(())
    }
}
