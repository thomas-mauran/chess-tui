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

#[derive(Debug, PartialEq)]
pub enum Pages {
    Home,
    Solo,
    Bot,
    Help,
    Credit,
}
