use crate::constants::DisplayMode;
use crate::pieces::PieceSize;
use shakmaty::Color;

pub struct King;

impl King {
    pub fn to_string(display_mode: &DisplayMode, size: PieceSize, color: Option<Color>) -> String {
        match display_mode {
            DisplayMode::DEFAULT | DisplayMode::CUSTOM => match size {
                PieceSize::Small => {
                    // Use standard Unicode chess symbols for 1x1
                    match color {
                        Some(Color::White) => "♔".to_string(),
                        Some(Color::Black) => "♚".to_string(),
                        None => " ".to_string(),
                    }
                }
                PieceSize::Compact => {
                    // Simple 2-line design for medium-sized cells
                    "\n ✚\n ███".to_string()
                }
                PieceSize::Extended => {
                    // Extended 3-4 line design - more solid and consistent
                    " ✚\n █▄█\n ███\n ███".to_string()
                }
                PieceSize::Large => {
                    // Current multi-line art
                    "\
      ✚\n\
    ▞▀▄▀▚\n\
    ▙▄█▄▟\n\
    ▐███▌\n\
   ▗█████▖\n\
    "
                    .to_string()
                }
            },
            DisplayMode::ASCII => "K".to_string(),
        }
    }
}
