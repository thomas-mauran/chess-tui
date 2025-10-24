use crate::constants::DisplayMode;

pub struct Bishop;

impl Bishop {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
    \n\
       ⭘\n\
      █✝█\n\
      ███\n\
    ▗█████▖\n\
    "
            }
            DisplayMode::ASCII => "B",
        }
    }
}
