use crate::constants::DisplayMode;
use crate::pieces::PieceSize;
use shakmaty::Color;

pub struct Bishop;

impl Bishop {
    #[must_use]
    pub fn to_string(display_mode: &DisplayMode, size: PieceSize, color: Option<Color>) -> String {
        match display_mode {
            DisplayMode::DEFAULT | DisplayMode::CUSTOM => match size {
                PieceSize::Small => match color {
                    Some(Color::White) => "♗".to_string(),
                    Some(Color::Black) => "♝".to_string(),
                    None => " ".to_string(),
                },
                PieceSize::Compact => {
                    // Simple 2-line design for medium-sized cells
                    "\n █x█\n ███".to_string()
                }
                PieceSize::Extended => {
                    // Extended design - bishop with X cross on top
                    " \n █x█\n ███\n ▗███▖".to_string()
                }
                PieceSize::Large => "\
    \n\
       ⭘\n\
      █x█\n\
      ███\n\
    ▗█████▖\n\
    "
                .to_string(),
            },
            DisplayMode::ASCII => "B".to_string(),
        }
    }
}
