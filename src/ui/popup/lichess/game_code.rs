use crate::{
    constants::WHITE, ui::components::centered_rect::centered_rect,
};
use ratatui::{
    layout::{Alignment, Position},
    style::{Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use crate::ui::prompt::Prompt;

// This renders a popup allowing us to enter a game code to join a Lichess game
pub fn render_enter_game_code_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Join Lichess Game")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(50, 30, frame.area());

    let current_input = prompt.input.as_str();

    let text = vec![
        Line::from("Enter game ID or URL:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(""),
        Line::from("You can paste the full URL or just the game ID."),
        Line::from("Note: You must be a participant in the game."),
        Line::from(""),
        Line::from("Press `Esc` to cancel.").alignment(Alignment::Center),
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