//! Waiting-for-opponent popup shown while the host listens for a connection.

use crate::{
    constants::{NETWORK_PORT, WHITE},
    ui::components::centered_rect::centered_rect,
};
use ratatui::{
    layout::Alignment,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use std::net::IpAddr;

/// Renders a "Waiting for other player" popup displaying the host's IP and port.
pub fn render_wait_for_other_player(frame: &mut Frame, ip: Option<IpAddr>) {
    let block = Block::default()
        .title("Waiting ...")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let ip_str = ip
        .map(|i| i.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let text = vec![
        Line::from(""),
        Line::from(""),
        Line::from("Waiting for other player").alignment(Alignment::Center),
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

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
