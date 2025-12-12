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
}
impl Pages {
    pub fn variant_count() -> usize {
        7
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Popups {
    ColorSelection,
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
}
