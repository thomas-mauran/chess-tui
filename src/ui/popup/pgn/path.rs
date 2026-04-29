//! PGN file path entry popup.

use crate::ui::prompt::Prompt;
use crate::{constants::WHITE, ui::components::centered_rect::centered_rect};
use ratatui::{
    layout::{Alignment, Position},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

/// Renders a text input for typing an absolute path to a `.pgn` file.
pub fn render_load_pgn_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Load PGN")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(60, 35, frame.area());

    let current_input = prompt.input.as_str();

    let text = vec![
        Line::from("Enter the absolute path to a .pgn file:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(""),
        Line::from("Multi-game PGN files are supported."),
        Line::from(""),
        Line::from("Press `Enter` to load, `Esc` to cancel.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.set_cursor_position(Position::new(
        area.x + prompt.character_index as u16 + 2,
        area.y + 3,
    ));

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
