//! Merges CLI args onto the persisted TOML config; exposes `Args` and `Config` to the rest of the crate.

use crate::app::AppResult;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self},
    path::Path,
};

use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path for the chess engine
    #[arg(short, long, default_value = "")]
    pub engine_path: String,
    /// Bot thinking depth for chess engine (1-255)
    #[arg(short, long)]
    pub depth: Option<u8>,
    /// Bot difficulty: easy, medium, hard, or magnus. Omit for full strength (Off).
    #[arg(long)]
    pub difficulty: Option<String>,
    /// Lichess API token
    #[arg(short, long)]
    pub lichess_token: Option<String>,
    /// Disable sound effects
    #[arg(long)]
    pub no_sound: bool,
    /// Skin/theme name (e.g. Default, ASCII). Overrides config for reproducible demos.
    #[arg(long)]
    pub skin: Option<String>,
    /// Update skin config with built-in default (prompts for confirmation, archives current file)
    #[arg(long)]
    pub update_skins: bool,
    /// Open a PGN file or directory of .pgn files directly in the viewer.
    #[arg(short = 'p', long)]
    pub pgn: Option<String>,
}

/// Persisted settings loaded from and written to `config.toml`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub engine_path: Option<String>,
    pub display_mode: Option<String>,
    pub log_level: Option<String>,
    pub bot_depth: Option<u8>,
    /// Bot difficulty preset: None = Off, Some(0..=3) = Easy/Medium/Hard/Magnus.
    pub bot_difficulty: Option<u8>,
    pub selected_skin_name: Option<String>,
    pub lichess_token: Option<String>,
    pub sound_enabled: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            engine_path: None,
            display_mode: Some("DEFAULT".to_string()),
            log_level: Some("OFF".to_string()),
            bot_depth: Some(10),
            bot_difficulty: None,
            selected_skin_name: Some("Default".to_string()),
            lichess_token: None,
            sound_enabled: Some(true),
        }
    }
}

impl Config {
    /// Checks if an IO error indicates a read-only filesystem or permission issue.
    /// This handles both EROFS (error code 30) and permission denied errors.
    pub fn is_readonly_error(e: &io::Error) -> bool {
        // EROFS = 30 on Unix systems (Read-only file system)
        e.raw_os_error() == Some(30) || e.kind() == io::ErrorKind::PermissionDenied
    }

    /// Handles config file write errors gracefully, logging appropriate warnings.
    /// This allows the application to work with read-only config files (e.g., from NixOS/home-manager).
    pub fn handle_config_write_error(e: io::Error, config_path: &Path) {
        if Self::is_readonly_error(&e) {
            log::warn!(
                "Config file at {:?} is read-only. Settings changes will not be persisted.",
                config_path
            );
        } else {
            log::warn!(
                "Could not write to config file at {:?}: {}. Settings changes will not be persisted.",
                config_path,
                e
            );
        }
    }

    /// Creates or updates the config file, merging CLI `args` on top of any existing values.
    /// Tolerates a read-only config: write errors are logged but never propagated.
    pub fn config_create(args: &Args, folder_path: &Path, config_path: &Path) -> AppResult<()> {
        std::fs::create_dir_all(folder_path)?;

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

        // Priority order: command line > environment variable > config file
        // Always update Lichess token if provided via command line
        if let Some(token) = &args.lichess_token {
            config.lichess_token = Some(token.clone());
        } else if config.lichess_token.is_none() {
            // If no token in config and not provided via CLI, check environment variable
            if let Ok(env_token) = env::var("LICHESS_TOKEN") {
                if !env_token.is_empty() {
                    config.lichess_token = Some(env_token);
                }
            }
        }

        // Update bot_depth if provided via command line
        if let Some(depth) = args.depth {
            config.bot_depth = Some(depth);
        }

        // Update bot_difficulty if provided via command line
        if let Some(ref d) = args.difficulty {
            let idx = match d.to_lowercase().as_str() {
                "easy" => Some(0),
                "medium" => Some(1),
                "hard" => Some(2),
                "magnus" => Some(3),
                _ => None,
            };
            if let Some(i) = idx {
                config.bot_difficulty = Some(i);
            }
        }

        // Always update sound_enabled if --no-sound flag is provided via command line (command line takes precedence)
        if args.no_sound {
            config.sound_enabled = Some(false);
        }

        // Try to write the config file, but don't fail if it's read-only
        // This allows the application to work with read-only config files (e.g., from NixOS/home-manager)
        let toml_string = toml::to_string(&config).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize config to TOML: {e}"),
            )
        })?;

        match File::create(config_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(toml_string.as_bytes()) {
                    Self::handle_config_write_error(e, config_path);
                }
            }
            Err(e) => {
                Self::handle_config_write_error(e, config_path);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::constants::config_dir;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_lichess_token_from_env_var() {
        // Set environment variable
        unsafe {
            env::set_var("LICHESS_TOKEN", "test_token_from_env");
        }

        let args = Args {
            engine_path: String::new(),
            depth: None,
            difficulty: None,
            lichess_token: None,
            no_sound: false,
            skin: None,
            update_skins: false,
            pgn: None,
        };

        let config_dir = config_dir().unwrap();
        let folder_path = config_dir.join(".test-env/chess-tui");
        let config_path = config_dir.join(".test-env/chess-tui/config.toml");

        // Clean up any existing test config
        let _ = fs::remove_dir_all(&folder_path);

        let result = Config::config_create(&args, &folder_path, &config_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        // Should have token from environment variable
        assert_eq!(config.lichess_token, Some("test_token_from_env".to_string()));

        // Clean up
        let _ = fs::remove_dir_all(&folder_path);
        unsafe {
            env::remove_var("LICHESS_TOKEN");
        }
    }

    #[test]
    #[serial]
    fn test_lichess_token_cli_takes_precedence_over_env() {
        // Set environment variable
        unsafe {
            env::set_var("LICHESS_TOKEN", "token_from_env");
        }

        let args = Args {
            engine_path: String::new(),
            depth: None,
            difficulty: None,
            lichess_token: Some("token_from_cli".to_string()),
            no_sound: false,
            skin: None,
            update_skins: false,
            pgn: None,
        };

        let config_dir = config_dir().unwrap();
        let folder_path = config_dir.join(".test-cli-precedence/chess-tui");
        let config_path = config_dir.join(".test-cli-precedence/chess-tui/config.toml");

        // Clean up any existing test config
        let _ = fs::remove_dir_all(&folder_path);

        let result = Config::config_create(&args, &folder_path, &config_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        // Should have token from CLI, not environment
        assert_eq!(config.lichess_token, Some("token_from_cli".to_string()));

        // Clean up
        let _ = fs::remove_dir_all(&folder_path);
        unsafe {
            env::remove_var("LICHESS_TOKEN");
        }
    }

    #[test]
    #[serial]
    fn test_lichess_token_config_preserved_when_no_env_or_cli() {
        // No environment variable set
        unsafe {
            env::remove_var("LICHESS_TOKEN");
        }

        let config_dir = config_dir().unwrap();
        let folder_path = config_dir.join(".test-preserve/chess-tui");
        let config_path = config_dir.join(".test-preserve/chess-tui/config.toml");

        // Clean up and create test directory
        let _ = fs::remove_dir_all(&folder_path);
        fs::create_dir_all(&folder_path).unwrap();

        // Create an existing config with a token
        let existing_config = Config {
            engine_path: None,
            display_mode: Some("DEFAULT".to_string()),
            log_level: Some("OFF".to_string()),
            bot_depth: Some(10),
            bot_difficulty: None,
            selected_skin_name: Some("Default".to_string()),
            lichess_token: Some("existing_token".to_string()),
            sound_enabled: Some(true),
        };

        let toml_string = toml::to_string(&existing_config).unwrap();
        fs::write(&config_path, toml_string).unwrap();

        // Create args with no token
        let args = Args {
            engine_path: String::new(),
            depth: None,
            difficulty: None,
            lichess_token: None,
            no_sound: false,
            skin: None,
            update_skins: false,
            pgn: None,
        };

        let result = Config::config_create(&args, &folder_path, &config_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        // Should still have the existing token
        assert_eq!(config.lichess_token, Some("existing_token".to_string()));

        // Clean up
        let _ = fs::remove_dir_all(&folder_path);
    }

    #[test]
    #[serial]
    fn test_empty_env_var_ignored() {
        // Set empty environment variable
        unsafe {
            env::set_var("LICHESS_TOKEN", "");
        }

        let args = Args {
            engine_path: String::new(),
            depth: None,
            difficulty: None,
            lichess_token: None,
            no_sound: false,
            skin: None,
            update_skins: false,
            pgn: None,
        };

        let config_dir = config_dir().unwrap();
        let folder_path = config_dir.join(".test-empty-env/chess-tui");
        let config_path = config_dir.join(".test-empty-env/chess-tui/config.toml");

        // Clean up any existing test config
        let _ = fs::remove_dir_all(&folder_path);

        let result = Config::config_create(&args, &folder_path, &config_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        // Should not have a token (empty env var is ignored)
        assert_eq!(config.lichess_token, None);

        // Clean up
        let _ = fs::remove_dir_all(&folder_path);
        unsafe {
            env::remove_var("LICHESS_TOKEN");
        }
    }
}
