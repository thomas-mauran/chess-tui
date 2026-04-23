//! Key-bindings help overlay, context-sensitive per active page.

use crate::{
    constants::WHITE, ui::components::centered_rect::centered_rect,
};
use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use crate::ui::popup::pgn::help::render_pgn_help_popup;

/// Renders the help popup, delegating to the PGN-specific variant when in the PGN viewer.
pub fn render_help_popup(frame: &mut Frame, app: &crate::app::App) {
    if app.ui_state.current_page == crate::constants::Pages::PgnViewer {
        render_pgn_help_popup(frame);
        return;
    }

    let block = Block::default()
        .title("Help menu")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 65, frame.area());

    // Check if we're playing against a bot (history navigation only in solo mode)
    let is_solo_mode = app.game.logic.bot.is_none() && app.game.logic.opponent.is_none();
    let is_puzzle_mode = app.lichess_state.puzzle_game.is_some();

    let mut text = vec![
        Line::from("Game controls:".underlined().bold()),
        Line::from(""),
    ];

    // In puzzle mode, 'h' is for hint, not left movement
    if is_puzzle_mode {
        text.push(Line::from(vec![
            "← ↑/k ↓/j →/l: Use these keys or the mouse to move the ".into(),
            "blue".blue(),
            " cursor".into(),
        ]));
        text.push(Line::from(""));
        text.push(Line::from(
            "T: Show hint (select the piece to move)".yellow(),
        ));
        text.push(Line::from(""));
    } else {
        text.push(Line::from(vec![
            "←/h ↑/k ↓/j →/l: Use these keys or the mouse to move the ".into(),
            "blue".blue(),
            " cursor".into(),
        ]));
        text.push(Line::from(""));
    }

    text.extend(vec![
        Line::from("`Ctrl` '+' or '-': Zoom in or out to adjust pieces sizes"),
        Line::from("(Might differ in certain terminals)"),
        Line::from(""),
        Line::from("`Space`: Select a piece"),
        Line::from(""),
        Line::from("`Esc`: Deselect a piece / hide popups"),
        Line::from(""),
        Line::from("q: Quit the game"),
        Line::from(""),
        Line::from("b: Go to the home menu / reset the game"),
        Line::from(""),
        Line::from("s: Cycle through available skins"),
        Line::from(""),
    ]);

    // Only show history navigation controls in solo mode (not against bot or puzzle)
    if is_solo_mode && !is_puzzle_mode {
        text.push(Line::from("P: Navigate to previous position in history"));
        text.push(Line::from(""));
        text.push(Line::from("N: Navigate to next position in history"));
        text.push(Line::from(""));
    }

    text.extend(vec![
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
        Line::from("Grey cell: Available cells for the selected piece"),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
    ]);

    let text = text;

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
