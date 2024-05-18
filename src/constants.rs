use core::fmt;
use std::path::PathBuf;

use crate::pieces::{PieceColor, PieceType};
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Players {
    Local,
    Enemy,
}

pub const WHITE_PLAYER_INIT_BOARD: [[Option<(PieceType, PieceColor)>; 8]; 8] = [
    [
        Some((PieceType::Rook, PieceColor::Black)),
        Some((PieceType::Knight, PieceColor::Black)),
        Some((PieceType::Bishop, PieceColor::Black)),
        Some((PieceType::Queen, PieceColor::Black)),
        Some((PieceType::King, PieceColor::Black)),
        Some((PieceType::Bishop, PieceColor::Black)),
        Some((PieceType::Knight, PieceColor::Black)),
        Some((PieceType::Rook, PieceColor::Black)),
    ],
    [
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
    ],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
    ],
    [
        Some((PieceType::Rook, PieceColor::White)),
        Some((PieceType::Knight, PieceColor::White)),
        Some((PieceType::Bishop, PieceColor::White)),
        Some((PieceType::Queen, PieceColor::White)),
        Some((PieceType::King, PieceColor::White)),
        Some((PieceType::Bishop, PieceColor::White)),
        Some((PieceType::Knight, PieceColor::White)),
        Some((PieceType::Rook, PieceColor::White)),
    ],
];

pub const BLACK_PLAYER_INIT_BOARD: [[Option<(PieceType, PieceColor)>; 8]; 8] = [
    [
        Some((PieceType::Rook, PieceColor::White)),
        Some((PieceType::Knight, PieceColor::White)),
        Some((PieceType::Bishop, PieceColor::White)),
        Some((PieceType::Queen, PieceColor::White)),
        Some((PieceType::King, PieceColor::White)),
        Some((PieceType::Bishop, PieceColor::White)),
        Some((PieceType::Knight, PieceColor::White)),
        Some((PieceType::Rook, PieceColor::White)),
    ],
    [
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
        Some((PieceType::Pawn, PieceColor::White)),
    ],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
        Some((PieceType::Pawn, PieceColor::Black)),
    ],
    [
        Some((PieceType::Rook, PieceColor::Black)),
        Some((PieceType::Knight, PieceColor::Black)),
        Some((PieceType::Bishop, PieceColor::Black)),
        Some((PieceType::King, PieceColor::Black)),
        Some((PieceType::Queen, PieceColor::Black)),
        Some((PieceType::Bishop, PieceColor::Black)),
        Some((PieceType::Knight, PieceColor::Black)),
        Some((PieceType::Rook, PieceColor::Black)),
    ],
];
