//! Loading popup

use crate::{
    constants::{CHESS_SET, WHITE},
    ui::components::centered_rect::centered_rect,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders, Clear, Padding},
};

/// Renders a centered popup with a throbber chess spinner and a custom text message
pub fn render_loading_popup(frame: &mut Frame, loading_text: String) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));

    let area = centered_rect(40, 40, frame.area());
    let inner_area = block.inner(area);

    // Chess pieces are double-width (2 cols) + space (1) + label
    let content_width = loading_text.len() as u16 + 3;

    let throbber_row = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner_area)[1];

    let throbber_col = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(content_width),
            Constraint::Fill(1),
        ])
        .split(throbber_row)[1];

    let full = throbber_widgets_tui::Throbber::default()
        .label(loading_text)
        .throbber_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .throbber_set(CHESS_SET)
        .use_type(throbber_widgets_tui::WhichUse::Spin);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(full, throbber_col);
}
