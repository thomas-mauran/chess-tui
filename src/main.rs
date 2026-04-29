#[cfg(feature = "chess-tui")]
extern crate chess_tui;

use chess_tui::app::{App, AppResult};
use chess_tui::config::{Args, Config};
use chess_tui::constants::{config_dir, DisplayMode, Pages, SKIN_NAME_ASCII, SKIN_NAME_DEFAULT};
use chess_tui::event::{Event, EventHandler};
use chess_tui::game_logic::opponent::wait_for_game_start;
use chess_tui::logging;
use chess_tui::pgn_viewer::PgnViewer;
use chess_tui::skin::{PieceStyle, Skin};
use chess_tui::ui::tui::Tui;
use clap::Parser;
use log::LevelFilter;
use std::fs;
use std::panic;
use std::path::Path;
use chess_tui::handlers::handler::{handle_key_events, handle_mouse_events};
use chess_tui::lichess::models::LichessClient;

fn main() -> AppResult<()> {
    // Parse the cli arguments first (this will handle --version and exit early if needed)
    let args = Args::parse();

    // Handle --update-skins: update config from default, then exit (no TUI)
    if args.update_skins {
        return Skin::run_update_skins();
    }

    // Used to enable mouse capture (only after we know we're running the TUI)
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::EnableMouseCapture
    )?;

    let config_dir = config_dir()?;
    let folder_path = config_dir.join("chess-tui");
    let config_path = config_dir.join("chess-tui/config.toml");

    // Create the configuration file
    Config::config_create(&args, &folder_path, &config_path)?;

    // Create an application.
    let mut app = App::default();

    // Check audio availability and disable sound if not available (e.g., in Docker)
    let audio_available = chess_tui::sound::check_audio_availability();
    if !audio_available {
        // Automatically disable sound if audio is not available
        app.sound_enabled = false;
    }
    // Initialize global sound state from app default
    chess_tui::sound::set_sound_enabled(app.sound_enabled);

    // We store the chess engine path if there is one
    if let Ok(content) = fs::read_to_string(config_path) {
        if content.trim().is_empty() {
            app.bot_state.chess_engine_path = None;
        } else {
            let config: Config = toml::from_str(&content).unwrap_or_default();

            if let Some(engine_path) = config.engine_path {
                app.bot_state.chess_engine_path = Some(engine_path);
            }
            // Set the display mode based on the configuration file
            if let Some(display_mode) = config.display_mode {
                app.game.ui.display_mode = match display_mode.as_str() {
                    "ASCII" => DisplayMode::ASCII,
                    "CUSTOM" => DisplayMode::CUSTOM,
                    _ => DisplayMode::DEFAULT,
                };
            }
            // Add log level handling
            if let Some(log_level) = config.log_level {
                app.log_level = log_level.parse().unwrap_or(LevelFilter::Off);
            }
            // Add bot depth handling
            if let Some(bot_depth) = config.bot_depth {
                app.bot_state.bot_depth = bot_depth;
            }
            // Bot difficulty
            app.bot_state.bot_difficulty = config.bot_difficulty;
            // Add selected skin name handling
            if let Some(selected_skin_name) = config.selected_skin_name {
                app.theme_state.selected_skin_name = selected_skin_name;
            }
            // Add lichess token handling
            if let Some(lichess_token) = config.lichess_token {
                app.lichess_state.token = Some(lichess_token.clone());
                app.lichess_state.client = Some(LichessClient::new(lichess_token));

            }
            // Add sound enabled handling
            if let Some(sound_enabled) = config.sound_enabled {
                app.sound_enabled = sound_enabled;
                chess_tui::sound::set_sound_enabled(sound_enabled);
            }
        }
    } else {
        println!("Error reading the file or the file does not exist");
    }

    // Always start with Default and ASCII display modes at the beginning
    app.theme_state.available_skins.push(Skin::default());
    app.theme_state.available_skins.push(Skin::ascii_display_mode());

    // Load all available skins from skins.json
    let skins_path = config_dir.join("chess-tui/skins.json");

    // Create skins.json if it doesn't exist
    if !skins_path.exists() {
        if let Err(e) = Skin::create_default_skins_file(&skins_path) {
            eprintln!("Failed to create default skins.json: {}", e);
        }
    }
    if skins_path.exists() {
        match Skin::load_all_skins(&skins_path) {
            Ok(skins) => {
                // Filter out any "Default" or "ASCII" skins from JSON to avoid duplicates
                let custom_skins: Vec<Skin> = skins
                    .into_iter()
                    .filter(|s| s.name != SKIN_NAME_DEFAULT && s.name != SKIN_NAME_ASCII)
                    .collect();
                app.theme_state.available_skins.extend(custom_skins);
            }
            Err(e) => {
                eprintln!("Failed to load skins: {}", e);
            }
        }

        match Skin::load_all_piece_styles(&skins_path) {
            Ok(piece_styles) => {
                // Filter out any "Default" or "ASCII" skins from JSON to avoid duplicates
                let custom_piece_styles: Vec<PieceStyle> = piece_styles
                    .into_iter()
                    .filter(|ps| ps.name != SKIN_NAME_DEFAULT && ps.name != SKIN_NAME_ASCII)
                    .collect();
                app.theme_state.available_piece_styles.extend(custom_piece_styles);
            }
            Err(e) => {
                eprintln!("Failed to load custom piece styles: {}", e);
            }
        }
        // Sync loaded piece styles to the game UI so rendering can use them
        app.game.ui.available_piece_styles = app.theme_state.available_piece_styles.clone();
    }

    // Apply selected skin
    if let Some(skin) = Skin::get_skin_by_name(&app.theme_state.available_skins, &app.theme_state.selected_skin_name) {
        app.theme_state.loaded_skin = Some(skin.clone());
        app.game.ui.skin = skin.clone();
        // Set display mode based on skin name
        match app.theme_state.selected_skin_name.as_str() {
            SKIN_NAME_DEFAULT => app.game.ui.display_mode = DisplayMode::DEFAULT,
            SKIN_NAME_ASCII => app.game.ui.display_mode = DisplayMode::ASCII,
            _ => {
                // For custom skins, set to CUSTOM if not already set
                if app.game.ui.display_mode == DisplayMode::DEFAULT {
                    app.game.ui.display_mode = DisplayMode::CUSTOM;
                }
            }
        }
    } else {
        // Fallback: use the first available skin if selected skin not found
        if let Some(first_skin) = app.theme_state.available_skins.first() {
            app.theme_state.selected_skin_name = first_skin.name.clone();
            app.theme_state.loaded_skin = Some(first_skin.clone());
            app.game.ui.skin = first_skin.clone();
            // Set display mode based on skin name
            match app.theme_state.selected_skin_name.as_str() {
                "Default" => app.game.ui.display_mode = DisplayMode::DEFAULT,
                "ASCII" => app.game.ui.display_mode = DisplayMode::ASCII,
                _ => app.game.ui.display_mode = DisplayMode::CUSTOM,
            }
        }
    }

    // Command line arguments take precedence over configuration file
    if !args.engine_path.is_empty() {
        app.bot_state.chess_engine_path = Some(args.engine_path.clone());
    }

    // Command line depth argument takes precedence over configuration file
    if let Some(depth) = args.depth {
        app.bot_state.bot_depth = depth;
    }

    // Command line difficulty argument takes precedence over configuration file
    if let Some(ref d) = args.difficulty {
        let idx = match d.to_lowercase().as_str() {
            "easy" => Some(0),
            "medium" => Some(1),
            "hard" => Some(2),
            "magnus" => Some(3),
            _ => None,
        };
        if let Some(i) = idx {
            app.bot_state.bot_difficulty = Some(i);
        }
    }

    // Command line lichess token takes precedence over configuration file
    if let Some(token) = &args.lichess_token {
        app.lichess_state.token = Some(token.clone());
    }

    // Command line no-sound flag takes precedence over configuration file
    if args.no_sound {
        app.sound_enabled = false;
        chess_tui::sound::set_sound_enabled(false);
    }

    // Command line skin takes precedence over configuration file (reproducible theme)
    if let Some(ref skin_name) = args.skin {
        app.theme_state.selected_skin_name = skin_name.clone();
        if let Some(skin) = Skin::get_skin_by_name(&app.theme_state.available_skins, skin_name) {
            app.theme_state.loaded_skin = Some(skin.clone());
            app.game.ui.skin = skin.clone();
            match skin_name.as_str() {
                "Default" => app.game.ui.display_mode = DisplayMode::DEFAULT,
                "ASCII" => app.game.ui.display_mode = DisplayMode::ASCII,
                _ => app.game.ui.display_mode = DisplayMode::CUSTOM,
            }
        }
    }

    // Setup logging
    if let Err(e) = logging::setup_logging(&folder_path, &app.log_level) {
        eprintln!("Failed to initialize logging: {}", e);
    }

    // Load PGN file(s) if --pgn was provided
    if let Some(ref pgn_path) = args.pgn {
        let path = Path::new(pgn_path);
        let load_result: Result<Vec<PgnViewer>, String> = if path.is_dir() {
            // Load all .pgn files from the directory
            let mut all_games: Vec<PgnViewer> = Vec::new();
            let mut entries: Vec<_> = std::fs::read_dir(path)
                .map_err(|e| e.to_string())?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|x| x == "pgn").unwrap_or(false))
                .collect();
            entries.sort_by_key(|e| e.path());
            for entry in entries {
                match PgnViewer::from_file(entry.path().to_str().unwrap_or("")) {
                    Ok(mut games) => all_games.append(&mut games),
                    Err(e) => eprintln!("Skipping {:?}: {}", entry.path(), e),
                }
            }
            if all_games.is_empty() {
                Err(format!("No valid .pgn files found in '{}'", pgn_path))
            } else {
                Ok(all_games)
            }
        } else {
            PgnViewer::from_file(pgn_path)
        };

        match load_result {
            Ok(games) => {
                app.pgn_viewer_state = Some(games);
                app.pgn_viewer_game_idx = 0;
                app.ui_state.current_page = Pages::PgnViewer;
            }
            Err(e) => {
                eprintln!("Failed to load PGN '{}': {}", pgn_path, e);
                std::process::exit(1);
            }
        }
    }

    // Initialize the terminal user interface.
    let terminal = ratatui::try_init()?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    let default_panic = std::panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        let _ = ratatui::crossterm::execute!(
            std::io::stdout(),
            ratatui::crossterm::event::DisableMouseCapture
        );
        default_panic(info);
    }));

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(_, _) => {}
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
                app.bot_state.start_bot_thinking(app.game.logic.game_board.fen_position(), bot.depth, bot.difficulty);
            }
            if let Some(bot) = app.game.logic.bot.as_mut() {
                bot.bot_will_move = false;
            }
        }

        // Check if bot move is ready
        if app.check_and_apply_bot_move() {
            app.check_and_show_game_end();
        }
        // Check if Lichess seek is done
        app.check_lichess_seek();

        // Check if game ended
        app.check_game_end_status();

        // Check if hosting player received game start signal from background thread
        if let Some(ref game_start_rx) = app.multiplayer_state.game_start_rx {
            if let Ok(()) = game_start_rx.try_recv() {
                if let Some(opponent) = app.game.logic.opponent.as_mut() {
                    log::info!("Host received game start signal, starting game");
                    opponent.game_started = true;
                    app.ui_state.close_popup();
                }
            }
        }

        // For non-hosting players, check directly on the stream
        if let Some(opponent) = app.game.logic.opponent.as_mut() {
            if !opponent.game_started && app.multiplayer_state.game_start_rx.is_none() {
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
