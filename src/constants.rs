use core::fmt;
use std::path::PathBuf;

use ratatui::style::Color;

pub const UNDEFINED_POSITION: u8 = u8::MAX;
pub const WHITE: Color = Color::Rgb(160, 160, 160);
pub const BLACK: Color = Color::Rgb(128, 95, 69);

// Network constants
pub const NETWORK_PORT: u16 = 2308;
pub const NETWORK_BUFFER_SIZE: usize = 5;
pub const SLEEP_DURATION_SHORT_MS: u64 = 50;
pub const SLEEP_DURATION_LONG_MS: u64 = 100;

// Time control constants
// Time control indices: 0: UltraBullet, 1: Bullet, 2: Blitz, 3: Rapid, 4: Classical, 5: No clock, 6: Custom
pub const TIME_CONTROL_CUSTOM_INDEX: u32 = 6;

/// Time control options displayed in the UI
pub const TIME_CONTROL_OPTIONS: &[&str] = &[
    "UltraBullet",
    "Bullet",
    "Blitz",
    "Rapid",
    "Classical",
    "No clock",
    "Custom",
];

pub const TITLE: &str = r"
 ██████╗██╗  ██╗███████╗███████╗███████╗   ████████╗██╗   ██╗██╗
██╔════╝██║  ██║██╔════╝██╔════╝██╔════╝   ╚══██╔══╝██║   ██║██║
██║     ███████║█████╗  ███████╗███████╗█████╗██║   ██║   ██║██║
██║     ██╔══██║██╔══╝  ╚════██║╚════██║╚════╝██║   ██║   ██║██║
╚██████╗██║  ██║███████╗███████║███████║      ██║   ╚██████╔╝██║
 ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝      ╚═╝    ╚═════╝ ╚═╝
";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    DEFAULT,
    ASCII,
    CUSTOM,
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayMode::ASCII => write!(f, "ASCII"),
            DisplayMode::DEFAULT => write!(f, "DEFAULT"),
            DisplayMode::CUSTOM => write!(f, "CUSTOM"),
        }
    }
}

/// Returns the user's config directory path.
///
/// # Errors
///
/// Returns an error if the config directory cannot be determined.
pub fn config_dir() -> Result<PathBuf, &'static str> {
    match dirs::config_dir() {
        Some(dir) => Ok(dir),
        None => Err("Could not get config directory"),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pages {
    Home,
    Solo,
    Multiplayer,
    Lichess,
    LichessMenu,
    OngoingGames,
    Bot,
    Credit,
    GameModeMenu,
}
impl Pages {
    #[must_use]
    pub fn variant_count() -> usize {
        8
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Popups {
    MultiplayerSelection,
    EnterHostIP,
    WaitingForOpponentToJoin,
    EnginePathError,
    Help,
    EndScreen,
    PuzzleEndScreen,
    Error,
    Success,
    SeekingLichessGame,
    EnterGameCode,
    EnterLichessToken,
    ResignConfirmation,
    MoveInputSelection,
}
