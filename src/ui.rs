use ratatui::{
    layout::Alignment,
    style::{Color, Style, Modifier},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};


use crate::{app::App, pieces::{PAWN, ROOK, QUEEN, KING, BISHOP, KNIGHT}};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "{}\n{}\n{}\n{}\n{}\n{}", PAWN, ROOK, QUEEN, KING, BISHOP, KNIGHT
        ))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Chess-tui")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Black).bg(Color::Rgb(220, 210, 175)).add_modifier(Modifier::BOLD)),
        frame.size(),
    )
}
