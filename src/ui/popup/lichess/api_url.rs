//! Lichess API base URL entry popup.

use crate::ui::prompt::Prompt;
use crate::{
    constants::{
        DEFAULT_LICHESS_API_URL, LICHESS_API_URL_ENV, WHITE, lichess_api_url_is_env_pinned,
    },
    ui::components::centered_rect::centered_rect,
};
use ratatui::{
    Frame,
    layout::{Alignment, Position},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders a text input for changing the Lichess API base URL.
///
/// The prompt is pre-filled with the URL currently in effect, so submitting an
/// unchanged input is a no-op. Clearing the input restores the default endpoint.
pub fn render_enter_lichess_api_url_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Lichess API URL")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(70, 60, frame.area());

    let mut text = vec![
        Line::from("Enter the Lichess API base URL:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(prompt.input.as_str()),
        Line::from(""),
    ];

    if let Some(error) = &prompt.error {
        text.push(Line::from(Span::styled(
            error.clone(),
            Style::default().fg(Color::Red),
        )));
        text.push(Line::from(""));
    }

    text.push(Line::from(format!("Default: {}", DEFAULT_LICHESS_API_URL)));
    text.push(Line::from("Leave empty to restore the default."));

    if lichess_api_url_is_env_pinned() {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            format!(
                "Note: {} is set and wins on next start.",
                LICHESS_API_URL_ENV
            ),
            Style::default().fg(Color::Yellow),
        )));
    }

    text.push(Line::from(""));
    text.push(Line::from("Press `Enter` to save, `Esc` to cancel.").alignment(Alignment::Center));

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
