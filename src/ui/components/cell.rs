//! Fills a single board square with a background color and an optional style modifier.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Stylize},
    widgets::Block,
};

/// Renders a single board cell as a colored `Block`, optionally applying a style modifier.
pub fn render_cell(frame: &mut Frame, square: Rect, color: Color, modifier: Option<Modifier>) {
    let mut cell = Block::default().bg(color);
    if let Some(modifier) = modifier {
        cell = cell.add_modifier(modifier);
    }
    frame.render_widget(cell, square);
}
