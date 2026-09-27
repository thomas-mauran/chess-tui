use chess_tui::app::AppResult;
use chess_tui::config::Args;
use chess_tui::skin::Skin;
use clap::Parser;

fn main() -> AppResult<()> {
    // Parse the cli arguments first (this will handle --version and exit early if needed)
    let args = Args::parse();

    // Handle --update-skins: update config from default, then exit (no TUI)
    if args.update_skins {
        return Skin::run_update_skins();
    }

    let mut app = chess_tui::setup::setup_app(&args)?;

    let _ = app.run();

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
