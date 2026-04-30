//! SAN move-entry popup anchored to the right side of the screen.

use crate::ui::prompt::Prompt;
use crate::{constants::WHITE, ui::components::right_rect::right_rect};
use ratatui::{
    Frame,
    layout::{Alignment, Position},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders a right-anchored text input box for typing a move in SAN notation.
pub fn render_move_input_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Input a move")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = right_rect(23, 40, frame.area());

    let current_input = prompt.input.as_str();
    let text = vec![
        Line::from("Enter a move in chess notation").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(
            "Visit this website for more information: https://www.chess.com/terms/chess-notation",
        ),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
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
