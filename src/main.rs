#[cfg(feature = "chess-tui")]
extern crate chess_tui;

use chess_tui::app::{App, AppResult};
use chess_tui::config::Config;
use chess_tui::constants::{config_dir, DisplayMode};
use chess_tui::event::{Event, EventHandler};
use chess_tui::game_logic::opponent::wait_for_game_start;
use chess_tui::handler::{handle_key_events, handle_mouse_events};
use chess_tui::logging;
use chess_tui::skin::Skin;
use chess_tui::ui::tui::Tui;
use clap::Parser;
use log::LevelFilter;
use std::fs::{self, File};
use std::io::Write;
use std::panic;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for the chess engine
    #[arg(short, long, default_value = "")]
    engine_path: String,
    /// Bot thinking depth for chess engine (1-255)
    #[arg(short, long, default_value = "10")]
    depth: u8,
    /// Lichess API token
    #[arg(short, long)]
    lichess_token: Option<String>,
    /// Disable sound effects
    #[arg(long)]
    no_sound: bool,
}

fn main() -> AppResult<()> {
    // Parse the cli arguments first (this will handle --version and exit early if needed)
    let args = Args::parse();

    // Used to enable mouse capture (only after we know we're running the TUI)
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::EnableMouseCapture
    )?;

    let config_dir = config_dir()?;
    let folder_path = config_dir.join("chess-tui");
    let config_path = config_dir.join("chess-tui/config.toml");

    // Create the configuration file
    config_create(&args, &folder_path, &config_path)?;

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
            app.chess_engine_path = None;
        } else {
            let config: Config = toml::from_str(&content).unwrap_or_default();

            if let Some(engine_path) = config.engine_path {
                app.chess_engine_path = Some(engine_path);
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
                app.bot_depth = bot_depth;
            }
            // Add selected skin name handling
            if let Some(selected_skin_name) = config.selected_skin_name {
                app.selected_skin_name = selected_skin_name;
            }
            // Add lichess token handling
            if let Some(lichess_token) = config.lichess_token {
                app.lichess_token = Some(lichess_token);
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
    app.available_skins.push(Skin::default_display_mode());
    app.available_skins.push(Skin::ascii_display_mode());

    // Load all available skins from skins.json
    let skins_path = config_dir.join("chess-tui/skins.json");

    // Create skins.json if it doesn't exist
    if !skins_path.exists() {
        if let Err(e) = create_default_skins_file(&skins_path) {
            eprintln!("Failed to create default skins.json: {}", e);
        }
    }

    if skins_path.exists() {
        match Skin::load_all_skins(&skins_path) {
            Ok(skins) => {
                // Filter out any "Default" or "ASCII" skins from JSON to avoid duplicates
                let custom_skins: Vec<Skin> = skins
                    .into_iter()
                    .filter(|s| s.name != "Default" && s.name != "ASCII")
                    .collect();
                app.available_skins.extend(custom_skins);
            }
            Err(e) => {
                eprintln!("Failed to load skins: {}", e);
            }
        }
    }

    // Apply selected skin
    if let Some(skin) = Skin::get_skin_by_name(&app.available_skins, &app.selected_skin_name) {
        app.loaded_skin = Some(skin.clone());
        app.game.ui.skin = skin.clone();
        // Set display mode based on skin name
        match app.selected_skin_name.as_str() {
            "Default" => app.game.ui.display_mode = DisplayMode::DEFAULT,
            "ASCII" => app.game.ui.display_mode = DisplayMode::ASCII,
            _ => {
                // For custom skins, set to CUSTOM if not already set
                if app.game.ui.display_mode == DisplayMode::DEFAULT {
                    app.game.ui.display_mode = DisplayMode::CUSTOM;
                }
            }
        }
    } else {
        // Fallback: use the first available skin if selected skin not found
        if let Some(first_skin) = app.available_skins.first() {
            app.selected_skin_name = first_skin.name.clone();
            app.loaded_skin = Some(first_skin.clone());
            app.game.ui.skin = first_skin.clone();
            // Set display mode based on skin name
            match app.selected_skin_name.as_str() {
                "Default" => app.game.ui.display_mode = DisplayMode::DEFAULT,
                "ASCII" => app.game.ui.display_mode = DisplayMode::ASCII,
                _ => app.game.ui.display_mode = DisplayMode::CUSTOM,
            }
        }
    }

    // Command line arguments take precedence over configuration file
    if !args.engine_path.is_empty() {
        app.chess_engine_path = Some(args.engine_path.clone());
    }

    // Command line depth argument takes precedence over configuration file
    app.bot_depth = args.depth;

    // Command line lichess token takes precedence over configuration file
    if let Some(token) = &args.lichess_token {
        app.lichess_token = Some(token.clone());
    }

    // Command line no-sound flag takes precedence over configuration file
    if args.no_sound {
        app.sound_enabled = false;
        chess_tui::sound::set_sound_enabled(false);
    }

    // Setup logging
    if let Err(e) = logging::setup_logging(&folder_path, &app.log_level) {
        eprintln!("Failed to initialize logging: {}", e);
    }

    // Initialize the terminal user interface.
    let terminal = ratatui::try_init()?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    let default_panic = std::panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        ratatui::crossterm::execute!(
            std::io::stdout(),
            ratatui::crossterm::event::DisableMouseCapture
        )
        .unwrap();
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
        if !app.is_bot_thinking()
            && app
                .game
                .logic
                .bot
                .as_ref()
                .is_some_and(|bot| bot.bot_will_move)
        {
            app.start_bot_thinking();
            if let Some(bot) = app.game.logic.bot.as_mut() {
                bot.bot_will_move = false;
            }
        }

        // Check if bot move is ready
        if app.check_bot_move() {
            app.check_and_show_game_end();
        }
        // Check if Lichess seek is done
        app.check_lichess_seek();

        // Check if game ended
        app.check_game_end_status();

        // Check if hosting player received game start signal from background thread
        if let Some(ref game_start_rx) = app.game_start_rx {
            if let Ok(()) = game_start_rx.try_recv() {
                if let Some(opponent) = app.game.logic.opponent.as_mut() {
                    log::info!("Host received game start signal, starting game");
                    opponent.game_started = true;
                    app.current_popup = None;
                }
            }
        }

        // For non-hosting players, check directly on the stream
        if let Some(opponent) = app.game.logic.opponent.as_mut() {
            if !opponent.game_started && app.game_start_rx.is_none() {
                match wait_for_game_start(opponent) {
                    Ok(true) => {
                        opponent.game_started = true;
                        app.current_popup = None;
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

fn config_create(args: &Args, folder_path: &Path, config_path: &Path) -> AppResult<()> {
    std::fs::create_dir_all(folder_path)?;

    if !config_path.exists() {
        //write to console
        File::create(config_path)?;
    }

    // Attempt to read the configuration file and parse it as a TOML Value.
    // If we encounter any issues (like the file not being readable or not being valid TOML), we start with a new, empty TOML table instead.
    let mut config: Config = match fs::read_to_string(config_path) {
        Ok(content) => toml::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    };

    // We update the configuration with the engine_path and display_mode.
    // If these keys are already in the configuration, we leave them as they are.
    // If they're not, we add them with default values.
    if config.engine_path.as_ref().is_none_or(|s| s.is_empty()) {
        if args.engine_path.is_empty() {
            config.engine_path = Some(String::new());
        } else {
            config.engine_path = Some(args.engine_path.clone());
        }
    }

    if config.display_mode.is_none() {
        config.display_mode = Some("DEFAULT".to_string());
    }
    if config.log_level.is_none() {
        config.log_level = Some(LevelFilter::Off.to_string());
    }
    if config.bot_depth.is_none() {
        config.bot_depth = Some(10);
    }
    if config.selected_skin_name.is_none() {
        config.selected_skin_name = Some("Default".to_string());
    }
    if config.sound_enabled.is_none() {
        config.sound_enabled = Some(true);
    }

    // Always update engine_path if provided via command line (command line takes precedence)
    if !args.engine_path.is_empty() {
        config.engine_path = Some(args.engine_path.clone());
    }

    // Always update Lichess token if provided via command line
    if let Some(token) = &args.lichess_token {
        config.lichess_token = Some(token.clone());
    }

    // Update bot_depth if provided via command line
    if args.depth != 10 {
        config.bot_depth = Some(args.depth);
    }

    // Always update sound_enabled if --no-sound flag is provided via command line (command line takes precedence)
    if args.no_sound {
        config.sound_enabled = Some(false);
    }

    let toml_string = toml::to_string(&config)
        .expect("Failed to serialize config to TOML. This is a bug, please report it.");
    let mut file = File::create(config_path)?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

fn create_default_skins_file(skins_path: &Path) -> AppResult<()> {
    // Ensure the directory exists
    if let Some(parent) = skins_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Default skins.json content (embedded at compile time)
    const DEFAULT_SKINS: &str = include_str!("default_skins.json");

    let mut file = File::create(skins_path)?;
    file.write_all(DEFAULT_SKINS.as_bytes())?;

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
            depth: 10,
            lichess_token: None,
            no_sound: false,
        };

        let config_dir = config_dir().unwrap();
        let folder_path = config_dir.join(".test/chess-tui");
        let config_path = config_dir.join(".test/chess-tui/config.toml");

        let result = config_create(&args, &folder_path, &config_path);

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
