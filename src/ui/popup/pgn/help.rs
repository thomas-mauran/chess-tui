//! PGN viewer key-bindings help popup.

use crate::{
    constants::WHITE, ui::components::centered_rect::centered_rect,
};
use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

/// Renders the read-only PGN viewer controls popup (navigation, auto-play, speed).
pub fn render_pgn_help_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("PGN viewer - controls")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(45, 55, frame.area());

    let text = vec![
        Line::from("Navigation".underlined().bold()),
        Line::from(""),
        Line::from("→ / N / l : Next move"),
        Line::from("← / P / h : Previous move"),
        Line::from("g         : Jump to start"),
        Line::from("G         : Jump to end"),
        Line::from("Tab       : Next game (multi-game PGN)"),
        Line::from(""),
        Line::from("Auto-play".underlined().bold()),
        Line::from(""),
        Line::from("Space     : Play / Pause"),
        Line::from("+  /  -   : Speed up / slow down"),
        Line::from("            (0.5x, 1x, 1.5x, 2x, 2.5x, 3x, 4x)"),
        Line::from(""),
        Line::from("Other".underlined().bold()),
        Line::from(""),
        Line::from("h         : Hide the end-of-game banner"),
        Line::from("?         : Toggle this help"),
        Line::from("Esc / q   : Exit the viewer"),
        Line::from(""),
        Line::from("Press `Esc` or `?` to close.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}