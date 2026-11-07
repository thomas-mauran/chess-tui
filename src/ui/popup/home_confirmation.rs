//! Confirmation before returning to the home page.

use crate::{constants::WHITE, ui::components::centered_rect::centered_rect};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders a confirmation dialog before leaving the current game.
pub fn render_home_confirmation_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("Return to home?")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(50, 30, frame.area());

    let key = |k: &'static str, color: Color| {
        Span::styled(k, Style::default().fg(color).add_modifier(Modifier::BOLD))
    };
    let choices = vec![
        key("Y", Color::Green),
        Span::raw(": return home   "),
        key("N", Color::Red),
        Span::raw(": keep playing"),
    ];
    let text = vec![
        Line::from(""),
        Line::from("Are you sure you want to leave this game?").alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from(choices).alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
