use chess_tui::app::AppResult;
use chess_tui::config::Args;
use chess_tui::event::{Event, EventHandler};
use chess_tui::game_logic::opponent::wait_for_game_start;
use chess_tui::handlers::handler::{handle_key_events, handle_mouse_events};
use chess_tui::skin::Skin;
use chess_tui::ui::tui::Tui;
use clap::Parser;

fn main() -> AppResult<()> {
    // Parse the cli arguments first (this will handle --version and exit early if needed)
    let args = Args::parse();

    // Handle --update-skins: update config from default, then exit (no TUI)
    if args.update_skins {
        return Skin::run_update_skins();
    }

    let mut app = chess_tui::setup::setup_app(&args)?;

    // Initialize the terminal user interface.
    let terminal = ratatui::try_init()?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events fast loop during animations, blocking otherwise.
        if app.animations.is_active() {
            match tui.events.try_next() {
                Some(Event::Tick) => app.tick(),
                Some(Event::Key(key_event)) => handle_key_events(key_event, &mut app)?,
                Some(Event::Mouse(mouse_event)) => handle_mouse_events(mouse_event, &mut app)?,
                Some(Event::Resize(_, _)) | None => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        } else {
            match tui.events.next()? {
                Event::Tick => app.tick(),
                Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
                Event::Resize(_, _) => {}
            }
        }

        // Check if bot should start thinking
        if !app.bot_state.is_bot_thinking()
            && app
                .game
                .logic
                .bot
                .as_ref()
                .is_some_and(|bot| bot.bot_will_move)
        {
            if let Some(bot) = &app.game.logic.bot {
                app.bot_state.start_bot_thinking(
                    app.game.logic.game_board.fen_position(),
                    bot.depth,
                    bot.difficulty,
                );
            }
            if let Some(bot) = app.game.logic.bot.as_mut() {
                bot.bot_will_move = false;
            }
        }

        // Check if bot move is ready
        if app.check_and_apply_bot_move() {
            app.check_and_show_game_end();
        }
        app.check_lichess_updates();

        // Check if game ended
        app.check_game_end_status();

        // Persist local/bot games each loop iteration. A move always
        // triggers a write; clock-only changes are throttled to ~1 Hz inside
        // so we don't hit the disk on every animation frame.
        app.tick_resume_state();

        // Check if hosting player received game start signal from background thread
        if let Some(ref game_start_rx) = app.multiplayer_state.game_start_rx
            && let Ok(()) = game_start_rx.try_recv()
            && let Some(opponent) = app.game.logic.opponent.as_mut()
        {
            log::info!("Host received game start signal, starting game");
            opponent.game_started = true;
            app.ui_state.close_popup();
        }

        // For non-hosting players, check directly on the stream
        if let Some(opponent) = app.game.logic.opponent.as_mut()
            && !opponent.game_started
            && app.multiplayer_state.game_start_rx.is_none()
        {
            match wait_for_game_start(opponent) {
                Ok(true) => {
                    opponent.game_started = true;
                    app.ui_state.close_popup();
                }
                Ok(false) => {
                    // Still waiting, do nothing
                }
                Err(e) => {
                    log::error!("Error waiting for game start: {}", e);
                }
            }
        }

        // If it's the opponent turn, wait for the opponent to move
        // Only check when it's actually the opponent's turn to avoid unnecessary work
        if let Some(opponent) = app.game.logic.opponent.as_ref() {
            // Check if it's the opponent's turn: if player_turn == opponent.color, it's opponent's turn
            let is_opponent_turn = app.game.logic.player_turn == opponent.color;

            // Only check for TCP multiplayer moves here (Lichess is handled in tick())
            // Check both the turn and the flag for TCP multiplayer
            if is_opponent_turn && opponent.opponent_will_move {
                // Check if it's TCP (not Lichess) - Lichess is handled in tick()
                if opponent.is_tcp_multiplayer() {
                    tui.draw(&mut app)?;

                    if !app.game.logic.game_board.is_checkmate()
                        && !app.game.logic.game_board.is_draw()
                        && app.game.logic.execute_opponent_move()
                    {
                        app.game.switch_player_turn();
                    }

                    // need to be centralised
                    app.check_game_end_status();
                    tui.draw(&mut app)?;
                }
            }
        }
    }

    // Exit the user interface.
    ratatui::try_restore()?;
    // Free up the mouse, otherwise it will remain linked to the terminal
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::DisableMouseCapture
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use chess_tui::{config::Config, constants::config_dir};

    use super::*;
    use std::fs;

    #[test]
    fn test_config_create() {
        let args = Args {
            engine_path: "test_engine_path".to_string(),
            depth: None,
            difficulty: None,
            lichess_token: None,
            no_sound: false,
            skin: None,
            update_skins: false,
            pgn: None,
        };

        let config_dir = config_dir().unwrap();
        let folder_path = config_dir.join(".test/chess-tui");
        let config_path = config_dir.join(".test/chess-tui/config.toml");

        let result = Config::config_create(&args, &folder_path, &config_path);

        assert!(result.is_ok());
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        assert_eq!(config.engine_path.unwrap(), "test_engine_path");
        assert_eq!(config.display_mode.unwrap(), "DEFAULT");
        assert_eq!(config.bot_depth.unwrap(), 10);
        let removed = fs::remove_file(config_path);
        assert!(removed.is_ok());
    }
}
