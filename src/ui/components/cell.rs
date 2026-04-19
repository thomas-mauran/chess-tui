use ratatui::{Frame, layout::Rect, style::{Color, Modifier, Stylize}, widgets::Block};


pub fn render_cell(frame: &mut Frame, square: Rect, color: Color, modifier: Option<Modifier>) {
    let mut cell = Block::default().bg(color);
    if let Some(modifier) = modifier {
        cell = cell.add_modifier(modifier);
    }
    frame.render_widget(cell, square);
}
