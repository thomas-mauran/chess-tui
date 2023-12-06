use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::App,
    constants::WHITE,
    pieces::{bishop::Bishop, knight::Knight, queen::Queen, rook::Rook, PieceColor},
    utils::get_opposite_color,
};

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
    let board_block = Block::default().style(Style::default());

    // We render the board_block in the center layout made above
    frame.render_widget(board_block.clone(), main_layout_vertical[1]);

    // We make the inside of the board
    app.board
        .board_render(board_block.inner(main_layout_vertical[1]), frame);

    // We make the inside of the board
    app.board
        .history_render(board_block.inner(main_layout_vertical[3]), frame);

    if app.show_popup {
        render_help_popup(frame)
    }

    if app.board.is_promotion {
        render_promotion_popup(frame, app)
    }

    if app.board.is_draw {
        render_end_popup(frame, "That's a draw".to_string())
    }

    if app.board.is_checkmate {
        let victorious_player = get_opposite_color(app.board.player_turn);

        let string_color = match victorious_player {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
        };

        render_end_popup(frame, format!("{} Won !!!", string_color))
    }
}

pub fn render_help_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("Help menu")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 60, frame.size());

    let text = vec![
        Line::from(""),
        Line::from(""),
        Line::from("Game controls:".underlined().bold()),
        Line::from(""),
        Line::from(vec![
            "← ↑ ↓ →: Use the arrow keys to move the ".into(),
            "blue".blue(),
            " cursor".into(),
        ]),
        Line::from(""),
        Line::from("SPACE_BAR: Select a piece"),
        Line::from(""),
        Line::from("ESCAPE: Deselect a piece",),
        Line::from(""),
        Line::from("q: Press q to quit the game"),
        Line::from(""),
        Line::from("Ctrl + or -: Zoom in or out to adjust pieces sizes"),
        Line::from(""),
        Line::from(""),
        Line::from("Color codes:".underlined().bold()),
        Line::from(""),
        Line::from(vec!["Blue cell".blue(), ": Your cursor ".into()]),
        Line::from(""),
        Line::from(vec!["Green cell".green(), ": Selected Piece ".into()]),
        Line::from(""),
        Line::from(vec![
            "Purple cell".magenta(),
            ": The king is getting checked ".into(),
        ]),
        Line::from(""),
        Line::from(vec![
            "Red cell".light_red(),
            ": Available cells for the selected piece ".into(),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from("press h to close the help menu").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render_end_popup(frame: &mut Frame, sentence: String) {
    let block = Block::default()
        .title("Game ended")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.size());

    let text = vec![
        Line::from(sentence).alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from("Press R to start a new game").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

pub fn render_promotion_popup(frame: &mut Frame, app: &App) {
    let block = Block::default()
        .title("Pawn promotion")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.size());

    let text = vec![
        Line::from(""),
        Line::from("-- Choose your pawn promotion --").alignment(Alignment::Center),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);

    let inner_popup_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(area);

    let inner_popup_layout_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
        )
        .split(inner_popup_layout_vertical[1]);

    let queen_p = Paragraph::new(Queen::to_string())
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 0 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(queen_p, inner_popup_layout_horizontal[0]);
    let rook_p = Paragraph::new(Rook::to_string())
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 1 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(rook_p, inner_popup_layout_horizontal[1]);
    let bishop_p = Paragraph::new(Bishop::to_string())
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 2 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(bishop_p, inner_popup_layout_horizontal[2]);
    let knight_p = Paragraph::new(Knight::to_string())
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 3 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(knight_p, inner_popup_layout_horizontal[3]);
}
