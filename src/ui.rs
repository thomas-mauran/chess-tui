use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Splitting the full tui in 3 vertical boxes and 3 horizontal boxes in the vertical[1]
    let main_area = frame.size();

    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 18),
                Constraint::Ratio(16, 18),
                Constraint::Ratio(1, 18),
            ]
            .as_ref(),
        )
        .split(main_area);
    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(2, 17),
                Constraint::Ratio(9, 17),
                Constraint::Ratio(1, 17),
                Constraint::Ratio(5, 17),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    // Board block representing the full board div
    let board_block = Block::default().style(Style::default().bg(Color::Rgb(210, 200, 190)));

    // We render the board_block in the center layout made above
    frame.render_widget(board_block.clone(), main_layout_vertical[1]);

    // We make the inside of the board
    app.board
        .board_render(board_block.inner(main_layout_vertical[1]), frame);

    // We make the inside of the board
    app.board
        .historic_render(board_block.inner(main_layout_vertical[3]), frame);
}
