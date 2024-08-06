#[cfg(feature = "chess-tui")]
extern crate chess_tui;

use chess_tui::app::{App, AppResult};
use chess_tui::constants::{home_dir, DisplayMode};
use chess_tui::event::{Event, EventHandler};
use chess_tui::handler::handle_key_events;
use chess_tui::tui::Tui;
use clap::Parser;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use toml::Value;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for the chess engine
    #[arg(short, long, default_value = "")]
    engine_path: String,
}

fn main() -> AppResult<()> {
    // Parse the cli arguments
    let args = Args::parse();

    let home_dir = home_dir()?;
    let folder_path = home_dir.join(".config/chess-tui");
    let config_path = home_dir.join(".config/chess-tui/config.toml");

    // Create the configuration file
    config_create(&args, &folder_path, &config_path)?;

    // Create an application.
    let mut app: App = App::default();

    // We store the chess engine path if there is one
    if let Ok(content) = fs::read_to_string(config_path) {
        if content.trim().is_empty() {
            app.chess_engine_path = None;
        } else {
            let config = content.parse::<toml::Value>().unwrap();
            if let Some(engine_path) = config.get("engine_path") {
                app.chess_engine_path = Some(engine_path.as_str().unwrap().to_string());
            }
            // Set the display mode based on the configuration file
            if let Some(display_mode) = config.get("display_mode") {
                app.board.display_mode = match display_mode.as_str() {
                    Some("ASCII") => DisplayMode::ASCII,
                    _ => DisplayMode::DEFAULT,
                };
            }
        }
    } else {
        println!("Error reading the file or the file does not exist");
    }

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) | Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
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
    let mut config = match fs::read_to_string(config_path) {
        Ok(content) => content
            .parse::<Value>()
            .unwrap_or_else(|_| Value::Table(Default::default())),
        Err(_) => Value::Table(Default::default()),
    };

    // We update the configuration with the engine_path and display_mode.
    // If these keys are already in the configuration, we leave them as they are.
    // If they're not, we add them with default values.
    if let Some(table) = config.as_table_mut() {
        // Only update the engine_path in the configuration if it's not empty
        if args.engine_path.is_empty() {
            table
                .entry("engine_path".to_string())
                .or_insert(Value::String(String::new()));
        } else {
            table.insert(
                "engine_path".to_string(),
                Value::String(args.engine_path.clone()),
            );
        }
        table
            .entry("display_mode".to_string())
            .or_insert(Value::String("DEFAULT".to_string()));
    }

    let mut file = File::create(config_path)?;
    file.write_all(config.to_string().as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use toml::Value;

    #[test]
    fn test_config_create() {
        let args = Args {
            engine_path: "test_engine_path".to_string(),
        };

        let home_dir = home_dir().expect("Failed to get home directory");
        let folder_path = home_dir.join(".test/chess-tui");
        let config_path = home_dir.join(".test/chess-tui/config.toml");

        let result = config_create(&args, &folder_path, &config_path);

        assert!(result.is_ok());
        assert!(config_path.exists());

        let content = fs::read_to_string(config_path).unwrap();
        let config: Value = content.parse().unwrap();
        let table = config.as_table().unwrap();

        assert_eq!(
            table.get("engine_path").unwrap().as_str().unwrap(),
            "test_engine_path"
        );
        assert_eq!(
            table.get("display_mode").unwrap().as_str().unwrap(),
            "DEFAULT"
        );
        let removed = fs::remove_dir_all(home_dir.join(".test"));
        assert!(removed.is_ok());
    }
}
