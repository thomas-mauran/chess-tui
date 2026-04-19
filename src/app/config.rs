use crate::app::App;
use crate::config::Config;
use crate::constants::config_dir;
use std::fs::{self, File};
use std::io::Write;

impl App {

    pub fn update_config(&self) {
        let Ok(config_dir) = config_dir() else {
            log::error!("Failed to get config directory");
            return;
        };
        let config_path = config_dir.join("chess-tui/config.toml");
        let mut config: Config = match fs::read_to_string(&config_path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        };

        config.display_mode = Some(self.game.ui.display_mode.to_string());
        config.log_level = Some(self.log_level.to_string());
        config.bot_depth = Some(self.bot_state.bot_depth);
        config.bot_difficulty = self.bot_state.bot_difficulty;
        config.selected_skin_name = Some(self.theme_state.selected_skin_name.clone());
        config.lichess_token = self.lichess_state.token.clone();
        config.sound_enabled = Some(self.sound_enabled);

        // Try to write the config file, but don't fail if it's read-only
        // This allows the application to work with read-only config files (e.g., from NixOS/home-manager)
        let toml_string = toml::to_string(&config).unwrap_or_default();
        match File::create(&config_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(toml_string.as_bytes()) {
                    Config::handle_config_write_error(e, &config_path);
                }
            }
            Err(e) => {
                Config::handle_config_write_error(e, &config_path);
            }
        }
    }
}