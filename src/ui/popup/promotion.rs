//! Pawn promotion piece-selection popup.

use crate::{
    app::App,
    constants::WHITE,
    pieces::{PieceSize, bishop::Bishop, knight::Knight, queen::Queen, rook::Rook}, ui::components::centered_rect::centered_rect,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

/// Renders a 4-option piece grid (Queen / Rook / Bishop / Knight) for pawn promotion.
///
/// Also writes the popup cell geometry into `app.game.ui` so that mouse clicks
/// on the pieces are handled correctly by the input layer.
pub fn render_promotion_popup(frame: &mut Frame, app: &mut App) {
    let block = Block::default()
        .title("Pawn promotion")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

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

    // When a promotion is happening, the mouse should be able to know where the icons are
    app.game.ui.top_x = inner_popup_layout_horizontal[0].x;
    app.game.ui.top_y = inner_popup_layout_horizontal[0].y;
    app.game.ui.width = inner_popup_layout_horizontal[0].width;
    app.game.ui.height = inner_popup_layout_horizontal[0].height;

    let piece_style = &app.game.ui.skin.piece_style;
    let piece_size = PieceSize::from_dimensions(inner_popup_layout_horizontal[0].height);
    let piece_color = Some(app.game.logic.player_turn);

    let piece_styles = &app.game.ui.available_piece_styles;
    let queen_p = Paragraph::new(Queen::to_string(
        piece_style.clone(),
        piece_size,
        piece_color,
        piece_styles,
    ))
    .block(Block::default())
    .alignment(Alignment::Center)
    .style(Style::default().bg(if app.game.ui.promotion_cursor == 0 {
        Color::LightBlue
    } else {
        Color::Reset // Set to the default background color when the condition is false
    }));
    frame.render_widget(queen_p, inner_popup_layout_horizontal[0]);
    let rook_p = Paragraph::new(Rook::to_string(
        piece_style.clone(),
        piece_size,
        piece_color,
        piece_styles,
    ))
    .block(Block::default())
    .alignment(Alignment::Center)
    .style(Style::default().bg(if app.game.ui.promotion_cursor == 1 {
        Color::LightBlue
    } else {
        Color::Reset // Set to the default background color when the condition is false
    }));
    frame.render_widget(rook_p, inner_popup_layout_horizontal[1]);
    let bishop_p = Paragraph::new(Bishop::to_string(
        piece_style.clone(),
        piece_size,
        piece_color,
        piece_styles,
    ))
    .block(Block::default())
    .alignment(Alignment::Center)
    .style(Style::default().bg(if app.game.ui.promotion_cursor == 2 {
        Color::LightBlue
    } else {
        Color::Reset // Set to the default background color when the condition is false
    }));
    frame.render_widget(bishop_p, inner_popup_layout_horizontal[2]);
    let knight_p = Paragraph::new(Knight::to_string(
        piece_style.clone(),
        piece_size,
        piece_color,
        piece_styles,
    ))
    .block(Block::default())
    .alignment(Alignment::Center)
    .style(Style::default().bg(if app.game.ui.promotion_cursor == 3 {
        Color::LightBlue
    } else {
        Color::Reset // Set to the default background color when the condition is false
    }));
    frame.render_widget(knight_p, inner_popup_layout_horizontal[3]);
}