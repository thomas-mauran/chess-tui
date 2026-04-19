use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use crate::ui::components::centered_rect::centered_rect;

// This renders a generic success popup with a custom message
pub fn render_success_popup(frame: &mut Frame, success_message: &str) {
    let block = Block::default()
        .title("Success")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(Color::Green));
    let area = centered_rect(50, 30, frame.area());

    // Split the success message by newlines and create a Line for each part
    let mut text = vec![Line::from("")];

    for line in success_message.split('\n') {
        if !line.is_empty() {
            text.push(Line::from(line).alignment(Alignment::Center));
        } else {
            text.push(Line::from(""));
        }
    }

    text.extend(vec![
        Line::from(""),
        Line::from("Press `Esc` or `Enter` to close.").alignment(Alignment::Center),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}