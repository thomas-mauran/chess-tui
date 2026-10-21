//! Project credits overlay.

use crate::{constants::WHITE, ui::components::centered_rect::centered_rect};
use ratatui::{
    Frame,
    layout::Alignment,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders a centered popup listing contributors and project information.
pub fn render_credit_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("Credits")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let credits_text = vec![
        Line::from(""),
        Line::from("Thank you for playing chess-tui!"),
        Line::from(""),
        Line::from(
            "chess-tui is an open source chess game for the terminal, written in Rust and built by its community of contributors.",
        ),
        Line::from(""),
        Line::from("Source code: https://github.com/thomas-mauran/chess-tui"),
        Line::from(""),
        Line::from(
            "Contributions are welcome: open an issue, submit a pull request, or star the repository to support the project.",
        ),
        Line::from(""),
        Line::from("Thanks to everyone who has contributed, reported bugs, or shared feedback."),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(credits_text)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}
