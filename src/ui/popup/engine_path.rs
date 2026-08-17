//! Chess engine path entry popup.

use crate::ui::prompt::Prompt;
use crate::{constants::WHITE, ui::components::centered_rect::centered_rect};
use ratatui::{
    Frame,
    layout::{Alignment, Position},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders a text input for entering and saving a chess engine path.
pub fn render_enter_engine_path_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Enter Chess Engine Path")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(70, 20, frame.area());

    let current_input = prompt.input.as_str();

    let text = vec![
        Line::from("Enter the absolute path of chess engine:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Enter` to save, `Esc` to cancel.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        area.x + prompt.character_index as u16 + 2,
        // Move one line down, from the border to the input line
        area.y + 3,
    ));

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
