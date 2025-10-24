use core::fmt;
use std::path::PathBuf;

use ratatui::style::Color;

pub const UNDEFINED_POSITION: u8 = u8::MAX;
pub const WHITE: Color = Color::Rgb(160, 160, 160);
pub const BLACK: Color = Color::Rgb(128, 95, 69);

pub const TITLE: &str = r"
 ██████╗██╗  ██╗███████╗███████╗███████╗   ████████╗██╗   ██╗██╗
██╔════╝██║  ██║██╔════╝██╔════╝██╔════╝   ╚══██╔══╝██║   ██║██║
██║     ███████║█████╗  ███████╗███████╗█████╗██║   ██║   ██║██║
██║     ██╔══██║██╔══╝  ╚════██║╚════██║╚════╝██║   ██║   ██║██║
╚██████╗██║  ██║███████╗███████║███████║      ██║   ╚██████╔╝██║
 ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝      ╚═╝    ╚═════╝ ╚═╝
";

#[derive(Debug, Clone, Copy)]
pub enum DisplayMode {
    DEFAULT,
    ASCII,
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayMode::ASCII => write!(f, "ASCII"),
            DisplayMode::DEFAULT => write!(f, "DEFAULT"),
        }
    }
}

pub fn home_dir() -> Result<PathBuf, &'static str> {
    match dirs::home_dir() {
        Some(dir) => Ok(dir),
        None => Err("Could not get home directory"),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pages {
    Home,
    Solo,
    Multiplayer,
    Bot,
    Credit,
}
impl Pages {
    pub fn variant_count() -> usize {
        6
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
}
