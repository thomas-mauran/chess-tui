//! Puzzle-completion result popup with optional Elo change.

use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use crate::ui::components::centered_rect::centered_rect;

/// Renders the puzzle-end popup showing the result, Elo delta, and next-action hints.
///
/// `elo_change` is `None` while the rating update is still being fetched from Lichess;
/// `is_calculating` should be `true` in that interval to show a "Calculating..." message.
pub fn render_puzzle_end_popup(
    frame: &mut Frame,
    sentence: &str,
    elo_change: Option<i32>,
    is_calculating: bool,
) {
    let block = Block::default()
        .title("Puzzle Complete")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::horizontal(2))
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::DarkGray));
    let area = centered_rect(50, 50, frame.area());

    // Create styled text with better formatting
    let mut text = vec![
        Line::from(""),
        Line::from(""),
        Line::from(sentence).alignment(Alignment::Center).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    // Add Elo change if available, or show calculating message
    if let Some(change) = elo_change {
        text.push(Line::from(""));
        let (change_text, color) = if change > 0 {
            (format!("+{} Elo", change), Color::Green)
        } else if change < 0 {
            (format!("{} Elo", change), Color::Red)
        } else {
            ("+0 Elo".to_string(), Color::Yellow)
        };
        text.push(
            Line::from(change_text)
                .alignment(Alignment::Center)
                .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
        );
    } else if is_calculating {
        text.push(Line::from(""));
        text.push(
            Line::from("Calculating Elo change...")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Cyan)),
        );
    }

    text.extend(vec![
        Line::from(""),
        Line::from(""),
        Line::from("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
            .style(Style::default().fg(Color::Gray)),
        Line::from(""),
        Line::from("Press `H` or `Esc` to hide this screen")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightBlue)),
        Line::from(""),
        Line::from("Press `N` for a new puzzle")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightGreen)),
        Line::from(""),
        Line::from("Press `B` to go back to the menu")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightCyan)),
        Line::from(""),
        Line::from(""),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}