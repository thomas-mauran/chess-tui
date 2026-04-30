//! Lichess API token entry popup (input masked for security).

use crate::ui::prompt::Prompt;
use crate::{constants::WHITE, ui::components::centered_rect::centered_rect};
use ratatui::{
    layout::{Alignment, Position},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

/// Renders a masked text input for entering and saving a Lichess API token.
pub fn render_enter_lichess_token_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Enter Lichess API Token")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(70, 40, frame.area());

    let current_input = prompt.input.as_str();
    // Mask the token for security (show only last 4 characters)
    let masked_input = if current_input.len() > 4 {
        format!(
            "{}...{}",
            "*".repeat(current_input.len().saturating_sub(4)),
            &current_input[current_input.len().saturating_sub(4)..]
        )
    } else if !current_input.is_empty() {
        "*".repeat(current_input.len())
    } else {
        String::new()
    };

    let text = vec![
        Line::from("Enter your Lichess API token:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(masked_input),
        Line::from(""),
        Line::from(""),
        Line::from("To get a token, follow the documentation:"),
        Line::from("Documentation: https://thomas-mauran.github.io/chess-tui/docs/Lichess/setup"),
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
