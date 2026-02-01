use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub engine_path: Option<String>,
    pub display_mode: Option<String>,
    pub log_level: Option<String>,
    pub bot_depth: Option<u8>,
    /// Optional ELO limit for UCI engine None = full strength.
    pub bot_elo: Option<u16>,
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
            bot_elo: None,
            selected_skin_name: Some("Default".to_string()),
            lichess_token: None,
            sound_enabled: Some(true),
        }
    }
}
