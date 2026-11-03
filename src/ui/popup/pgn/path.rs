//! PGN source selection and input popups.

use crate::ui::prompt::Prompt;
use crate::{constants::WHITE, ui::components::centered_rect::centered_rect};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Position},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
};

pub fn render_pgn_source_popup(frame: &mut Frame, selection: usize) {
    let choices = ["File path", "Paste PGN"];
    let lines = choices.into_iter().enumerate().map(|(index, label)| {
        if index == selection {
            Line::styled(
                format!("> {label}"),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Line::from(format!("  {label}"))
        }
    });
    let mut text: Vec<_> = lines.collect();
    text.push(Line::from("↑/↓ Choose · Enter Select · Esc Cancel"));

    let area = centered_rect(60, 40, frame.area());
    frame.render_widget(Clear, area);
    frame.render_widget(Paragraph::new(text).block(load_block("Load PGN")), area);
}

pub fn render_pgn_paste_popup(frame: &mut Frame, prompt: &Prompt) {
    let area = centered_rect(80, 70, frame.area());
    let block = load_block("Paste PGN");
    let [instructions, preview, status] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(block.inner(area));

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new("Paste using your terminal's paste shortcut.").wrap(Wrap { trim: true }),
        instructions,
    );
    frame.render_widget(
        Paragraph::new(prompt.input.as_str()).wrap(Wrap { trim: false }),
        preview,
    );
    let status_text = prompt.error.as_deref().map_or_else(
        || Line::from("Enter Load · Esc Cancel"),
        |error| Line::styled(error, Style::default().fg(Color::Red)),
    );
    frame.render_widget(
        Paragraph::new(status_text).wrap(Wrap { trim: true }),
        status,
    );
}

fn load_block(title: &'static str) -> Block<'static> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE))
}

/// Renders a text input for typing an absolute path to a `.pgn` file.
pub fn render_load_pgn_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = load_block("Load PGN");
    let area = centered_rect(60, 35, frame.area());

    let current_input = prompt.input.as_str();

    let text = vec![
        Line::from("Enter the absolute path to a .pgn file:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(""),
        Line::from("Multi-game PGN files are supported."),
        Line::from(""),
        Line::from("Press `Enter` to load, `Esc` to cancel.").alignment(Alignment::Center),
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
