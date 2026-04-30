//! Calculates a right-aligned [`Rect`] sized as a percentage of a parent area.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Returns a rect anchored to the right of `r`, sized to `percent_x` × `percent_y` of `r`.
pub fn right_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100 - percent_x),
            Constraint::Percentage(percent_x),
        ])
        .split(popup_layout[1]);

    cols[1]
}
