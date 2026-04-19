use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use crate::ui::components::centered_rect::centered_rect;

// This renders a popup for a promotion
pub fn render_end_popup(frame: &mut Frame, sentence: &str, is_lichess: bool) {
    let block = Block::default()
        .title("Game Over")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::horizontal(2))
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::DarkGray));
    let area = centered_rect(50, 50, frame.area());

    // Create styled text with better formatting
    let mut text = vec![Line::from(""), Line::from("")];

    // Split sentence by newlines to handle multi-line messages
    for line in sentence.lines() {
        text.push(
            Line::from(line).alignment(Alignment::Center).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        );
    }

    text.extend(vec![
        Line::from(""),
        Line::from(""),
        Line::from("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
            .style(Style::default().fg(Color::Gray)),
        Line::from(""),
        Line::from("Press `H` to hide this screen")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightBlue)),
    ]);

    // Only show restart option for non-Lichess games (Lichess games can't be restarted)
    if !is_lichess {
        text.push(Line::from(""));
        text.push(
            Line::from("Press `R` to restart a new game")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::LightGreen)),
        );
    }

    text.push(Line::from(""));
    text.push(
        Line::from("Press `B` to go back to the menu")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightCyan)),
    );
    text.push(Line::from(""));
    text.push(Line::from(""));

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}