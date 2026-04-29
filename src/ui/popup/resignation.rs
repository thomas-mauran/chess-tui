//! Resignation confirmation popup.

use crate::{app::App, constants::WHITE, ui::components::centered_rect::centered_rect};
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

/// Renders a "Yes / No" confirmation dialog before resigning the current game.
pub fn render_resign_confirmation_popup(frame: &mut Frame, app: &App) {
    let block = Block::default()
        .title("Resign Game")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(50, 30, frame.area());

    let opponent_name = if let Some(game) = app
        .lichess_state
        .ongoing_games
        .get(app.ui_state.menu_cursor as usize)
    {
        format!("vs {}", game.opponent.username)
    } else {
        "this game".to_string()
    };

    let text = vec![
        Line::from(""),
        Line::from(format!(
            "Are you sure you want to resign {}?",
            opponent_name
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from("This action cannot be undone.").alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Y",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("es / "),
            Span::styled(
                "N",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw("o"),
        ])
        .alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
