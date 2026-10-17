//! Confirmation before a new game replaces a saved one.

use crate::{
    app::App, constants::WHITE, state::ui_state::NewGame,
    ui::components::centered_rect::centered_rect,
};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

/// Renders the "discard the saved game?" dialog for `app.ui_state.pending_new_game`.
pub fn render_new_game_confirmation_popup(frame: &mut Frame, app: &App) {
    let restart = app.ui_state.pending_new_game == Some(NewGame::Restart);
    let block = Block::default()
        .title(if restart { "Restart Game" } else { "New Game" })
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(50, 30, frame.area());

    let key = |k: &'static str, color: Color| {
        Span::styled(k, Style::default().fg(color).add_modifier(Modifier::BOLD))
    };
    let mut choices = vec![
        key("Y", Color::Green),
        Span::raw(if restart {
            ": restart   "
        } else {
            ": new game   "
        }),
    ];
    if !restart {
        choices.extend([key("R", Color::Yellow), Span::raw(": resume saved game   ")]);
    }
    choices.extend([
        key("N", Color::Red),
        Span::raw(if restart {
            ": keep playing"
        } else {
            ": cancel"
        }),
    ]);

    let text = vec![
        Line::from(""),
        Line::from(if restart {
            "Restart this game from the beginning?"
        } else {
            "You have a saved game in this mode."
        })
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(if restart {
            "Its saved position will be lost."
        } else {
            "Starting a new game discards it."
        })
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from(choices).alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
