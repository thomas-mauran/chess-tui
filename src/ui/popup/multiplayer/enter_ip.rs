//! IP address entry popup for joining a hosted multiplayer game.

use crate::ui::prompt::Prompt;
use crate::{
    constants::{NETWORK_PORT, WHITE},
    ui::components::centered_rect::centered_rect,
};
use ratatui::{
    Frame,
    layout::{Alignment, Position},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders a text input for entering the host's IP address and port.
pub fn render_enter_multiplayer_ip(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Join a game")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let current_input = prompt.input.as_str();

    let text = vec![
        Line::from("Enter the ip address and port of the host:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(format!("Example: 10.111.6.50:{};", NETWORK_PORT)),
        Line::from(
            "Documentation: https://thomas-mauran.github.io/chess-tui/docs/Multiplayer/Online%20multiplayer/",
        ),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
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
