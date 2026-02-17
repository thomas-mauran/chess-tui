use crate::{pieces::PieceSize, skin::PieceStyle};
use shakmaty::Color;

pub struct King;

impl King {
    #[must_use]
    pub fn to_string(
        piece_style: String,
        size: PieceSize,
        color: Option<Color>,
        piece_styles: &[PieceStyle],
    ) -> String {
        match piece_style.as_str() {
            "DEFAULT" => match size {
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
                    "▗▂╋▂▖\n ▀█▀ \n ▀▀▀ ".to_string()
                }
                PieceSize::Extended => {
                    // Extended 3-4 line design - more solid and consistent
                    " ▂╋▂ \n▜███▛\n ▜█▛ \n▝▀▀▀▘".to_string()
                }
                PieceSize::Large => {
                    // Current multi-line art
                    r#"  ▂▃╋▃▂  
 ▐█████▋ 
  ▜███▛  
   ▟█▙   
  ▀▀▀▀▀  
"#
                    .to_string()
                }
            },
            "ASCII" => "K".to_string(),
            _ => piece_styles
                .iter()
                .find(|ps| ps.name.eq_ignore_ascii_case(&piece_style))
                .map(|ps| {
                    let block = match size {
                        PieceSize::Small => &ps.small,
                        PieceSize::Compact => &ps.compact,
                        PieceSize::Extended => &ps.extended,
                        PieceSize::Large => &ps.large,
                    };
                    block.king.clone()
                })
                .unwrap_or_else(|| format!("({}) piece style not found", piece_style)),
        }
    }
}
