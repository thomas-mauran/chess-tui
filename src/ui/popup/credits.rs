use crate::{
    constants::WHITE, ui::components::centered_rect::centered_rect,
};
use ratatui::{
    layout::Alignment,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};



// This render the credit popup
pub fn render_credit_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("Credits")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let credits_text = vec![
        Line::from(""),
        Line::from("Hi 👋, I'm Thomas, a 23 years old French computer engineer."),
        Line::from("Thank you for playing Chess-tui! This project started as a personal journey to improve my algorithmic skills and learn Rust."),
        Line::from(""),
        Line::from("The entire source code is available on GitHub at https://github.com/thomas-mauran/chess-tui"),
        Line::from(""),
        Line::from("Feel free to contribute by picking an issue or creating a new one and star the repo if you wanna support the project !"),
        Line::from(""),
        Line::from("Special thanks to my classmates for their support and inspiration!"),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
    ];

    // Assuming Alignment is an enum or struct you have defined

    let paragraph = Paragraph::new(credits_text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}