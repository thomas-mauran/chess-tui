#[cfg(feature = "chess-tui")]
extern crate chess_tui;

use chess_tui::app::{App, AppResult};
use chess_tui::board::Board;
use chess_tui::event::{Event, EventHandler};
use chess_tui::handler::handle_key_events;
use chess_tui::tui::Tui;
use clap::Parser;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs::{self, File};
use std::io::{self, Write};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for the chess engine
    #[arg(short, long, default_value = "")]
    engine_path: String,

    /// Fen to start the game from
    #[arg(short, long, default_value = "")]
    fen_position: String,
    // /// Pgn to start the game from
    // #[arg(short, long, default_value = "")]
    // pgn_position: String,
}

fn main() -> AppResult<()> {
    // Parse the cli arguments
    let args = Args::parse();

    let config_path = dirs::config_dir().unwrap().join("chess-tui").join("config");

    if !args.engine_path.is_empty() {
        if !config_path.exists() {
            if !config_path.parent().unwrap().exists() {
                fs::create_dir(config_path.parent().unwrap())?;
            }
            File::create(&config_path)?;
        }

        let mut file = File::create(&config_path)?;
        file.write_all(args.engine_path.as_bytes())?;
    }

    // Create an application.
    let mut app = App::default();
    if !args.fen_position.is_empty() {
        app.board = Board::from_fen(args.fen_position.trim())?;
    }
    // if !args.pgn_position.is_empty() {
    //     app.board = Board::pgn_import(args.fen_position.trim())?;
    // }

    // We store the chess engine path if there is one
    if let Ok(content) = fs::read_to_string(config_path) {
        if content.trim().is_empty() {
            app.chess_engine_path = None
        } else {
            app.chess_engine_path = Some(content)
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
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
