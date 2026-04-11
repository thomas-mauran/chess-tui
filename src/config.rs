use serde::{Deserialize, Serialize};
use std::{io::{self}, path::Path};

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
                config_path, e
            );
        }
    }
}
