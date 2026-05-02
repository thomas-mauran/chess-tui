//! Waiting-for-opponent popup shown while the host listens for a connection.

use crate::{
    constants::{CHESS_SET, NETWORK_PORT, WHITE},
    ui::components::centered_rect::centered_rect,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};
use std::net::IpAddr;

/// Renders a "Waiting for other player" popup displaying the host's IP and port.
pub fn render_wait_for_other_player(frame: &mut Frame, ip: Option<IpAddr>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());
    let inner_area = block.inner(area);

    let ip_str = ip
        .map(|i| i.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let text = vec![
        Line::from(""),
        Line::from(""),
        Line::from("").alignment(Alignment::Center),
        Line::from(format!(
            "Host IP address and port: {}:{}",
            ip_str, NETWORK_PORT
        ))
        .alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let content_width = "Waiting for other player ...".len() as u16 + 3;

    let throbber_row = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner_area)[1];

    let throbber_col = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(content_width),
            Constraint::Fill(1),
        ])
        .split(throbber_row)[1];

    let full = throbber_widgets_tui::Throbber::default()
        .label("Waiting for other player ...")
        .throbber_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .throbber_set(CHESS_SET)
        .use_type(throbber_widgets_tui::WhichUse::Spin);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
    frame.render_widget(full, throbber_col);
}
