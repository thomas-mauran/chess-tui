use core::fmt;
use std::path::PathBuf;

use ratatui::style::Color;

pub const UNDEFINED_POSITION: i8 = -1;
pub const WHITE: Color = Color::Rgb(160, 160, 160);
pub const BLACK: Color = Color::Rgb(128, 95, 69);

pub const TITLE: &str = r#"
 ██████╗██╗  ██╗███████╗███████╗███████╗   ████████╗██╗   ██╗██╗
██╔════╝██║  ██║██╔════╝██╔════╝██╔════╝   ╚══██╔══╝██║   ██║██║
██║     ███████║█████╗  ███████╗███████╗█████╗██║   ██║   ██║██║
██║     ██╔══██║██╔══╝  ╚════██║╚════██║╚════╝██║   ██║   ██║██║
╚██████╗██║  ██║███████╗███████║███████║      ██║   ╚██████╔╝██║
 ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝      ╚═╝    ╚═════╝ ╚═╝
"#;

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

#[derive(Debug, PartialEq)]
pub enum Pages {
    Home,
    Solo,
    Bot,
    Credit,
}
