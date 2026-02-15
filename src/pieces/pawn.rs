use crate::pieces::PieceSize;
use crate::skin::PieceStyle;
use shakmaty::Color;

pub struct Pawn;

impl Pawn {
    #[must_use]
    pub fn to_string(
        piece_style: String,
        size: PieceSize,
        color: Option<Color>,
        piece_styles: &[PieceStyle],
    ) -> String {
        match piece_style.as_str() {
            "DEFAULT" => match size {
                PieceSize::Small => match color {
                    Some(Color::White) => "♙".to_string(),
                    Some(Color::Black) => "♟".to_string(),
                    None => " ".to_string(),
                },
                PieceSize::Compact => "\n ▟▙\n ██".to_string(),
                PieceSize::Extended => "   \n   \n ▟▙\n ██".to_string(),
                PieceSize::Large => "\
        \n\
        \n\
      ▟█▙\n\
      ▜█▛\n\
     ▟███▙\n\
    "
                .to_string(),
            },
            "ASCII" => "P".to_string(),
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
                    block.pawn.clone()
                })
                .unwrap_or_else(|| format!("({}) piece style not found", piece_style)),
        }
    }
}
