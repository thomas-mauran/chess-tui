#[cfg(feature = "chess-tui")]
extern crate chess_tui;

use chess_tui::app::{App, AppResult};
use chess_tui::constants::{home_dir, DisplayMode};
use chess_tui::event::{Event, EventHandler};
use chess_tui::game_logic::game::GameState;
use chess_tui::game_logic::opponent::wait_for_game_start;
use chess_tui::handler::{handle_key_events, handle_mouse_events};
use chess_tui::logging;
use chess_tui::ui::tui::Tui;
use chess_tui::config::Config;
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
}

fn main() -> AppResult<()> {
    // Used to enable mouse capture
    ratatui::crossterm::execute!(
        std::io::stdout(),
        ratatui::crossterm::event::EnableMouseCapture
    )?;
    // Parse the cli arguments
    let args = Args::parse();

    let home_dir = home_dir()?;
    let folder_path = home_dir.join(".config/chess-tui");
    let config_path = home_dir.join(".config/chess-tui/config.toml");

    // Create the configuration file
    config_create(&args, &folder_path, &config_path)?;

    // Create an application.
    let mut app = App::default();

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
                    _ => DisplayMode::DEFAULT,
                };
            }
            // Add log level handling
            if let Some(log_level) = config.log_level {
                app.log_level = log_level
                    .parse()
                    .unwrap_or(LevelFilter::Off);
            }
            // Add bot depth handling
            if let Some(bot_depth) = config.bot_depth {
                app.bot_depth = bot_depth;
            }
        }
    } else {
        println!("Error reading the file or the file does not exist");
    }

    // Command line arguments take precedence over configuration file
    if !args.engine_path.is_empty() {
        app.chess_engine_path = Some(args.engine_path.clone());
    }

    // Command line depth argument takes precedence over configuration file
    app.bot_depth = args.depth;

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
        if app.game.logic.bot.is_some() && app.game.logic.bot.as_ref().is_some_and(|bot| bot.bot_will_move) {
            app.game.logic.execute_bot_move();
            app.game.switch_player_turn();
            if let Some(bot) = app.game.logic.bot.as_mut() {
                bot.bot_will_move = false;
            }
            // need to be centralised
            if app.game.logic.game_board.is_checkmate() {
                app.game.logic.game_state = GameState::Checkmate;
                app.show_end_screen();
            } else if app.game.logic.game_board.is_draw(app.game.logic.player_turn) {
                app.game.logic.game_state = GameState::Draw;
                app.show_end_screen();
            }
            tui.draw(&mut app)?;
        }

        if app.game.logic.opponent.is_some()
            && app
                .game
                .logic
                .opponent
                .as_ref()
                .is_some_and(|opponent| !opponent.game_started)
        {
            let opponent = app.game.logic.opponent.as_mut().unwrap();
            wait_for_game_start(opponent.stream.as_ref().unwrap());
            opponent.game_started = true;
            app.current_popup = None;
        }

        // If it's the opponent turn, wait for the opponent to move
        if app.game.logic.opponent.is_some()
            && app
                .game
                .logic
                .opponent
                .as_ref()
                .is_some_and(|opponent| opponent.opponent_will_move)
        {
            tui.draw(&mut app)?;

            if !app.game.logic.game_board.is_checkmate() && !app.game.logic.game_board.is_draw(app.game.logic.player_turn) && app.game.logic.execute_opponent_move() {
                app.game.switch_player_turn();
            }

            // need to be centralised
            if app.game.logic.game_board.is_checkmate() {
                app.game.logic.game_state = GameState::Checkmate;
                app.show_end_screen();
            } else if app.game.logic.game_board.is_draw(app.game.logic.player_turn) {
                app.game.logic.game_state = GameState::Draw;
                app.show_end_screen();
            }
            tui.draw(&mut app)?;
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
    if config.engine_path.is_none() || (config.engine_path.is_some() && config.engine_path.as_ref().unwrap().is_empty()) {
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

    // Update bot_depth if provided via command line
    if args.depth != 10 {
        config.bot_depth = Some(args.depth);
    }

    let toml_string = toml::to_string(&config)
        .expect("Failed to serialize config to TOML. This is a bug, please report it.");
    let mut file = File::create(config_path)?;
    file.write_all(toml_string.as_bytes())?;

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
        };

        let home_dir = home_dir().expect("Failed to get home directory");
        let folder_path = home_dir.join(".test/chess-tui");
        let config_path = home_dir.join(".test/chess-tui/config.toml");

        let result = config_create(&args, &folder_path, &config_path);

        assert!(result.is_ok());
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        assert_eq!(
            config.engine_path.unwrap(),
            "test_engine_path"
        );
        assert_eq!(
            config.display_mode.unwrap(),
            "DEFAULT"
        );
        assert_eq!(config.bot_depth.unwrap(), 10);
        let removed = fs::remove_file(config_path);
        assert!(removed.is_ok());
    }
}
