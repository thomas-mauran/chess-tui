use crate::{
    app::App,
    constants::WHITE,
    pieces::{bishop::Bishop, knight::Knight, queen::Queen, rook::Rook},
    ui::centered_rect,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};

// This renders a popup when the selected game mode is bot and there is no chess engine path
pub fn render_engine_path_error_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.size());

    let text = vec![
        Line::from("You didn't specify the chess engine path").alignment(Alignment::Center),
        Line::from(""),
        Line::from("To do so use the -e argument when running chess-tui to store an engine path"),
        Line::from(""),
        Line::from("Example: "),
        Line::from("chess-tui -e /opt/homebrew/opt/stockfish"),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a popup for a promotion
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
        Line::from("Press `R` to start a new game").alignment(Alignment::Center),
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

    let display_mode = &app.board.display_mode;

    let queen_p = Paragraph::new(Queen::to_string(display_mode))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 0 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(queen_p, inner_popup_layout_horizontal[0]);
    let rook_p = Paragraph::new(Rook::to_string(display_mode))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 1 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(rook_p, inner_popup_layout_horizontal[1]);
    let bishop_p = Paragraph::new(Bishop::to_string(display_mode))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 2 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(bishop_p, inner_popup_layout_horizontal[2]);
    let knight_p = Paragraph::new(Knight::to_string(display_mode))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.board.promotion_cursor == 3 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(knight_p, inner_popup_layout_horizontal[3]);
}

// This render the credit popup
pub fn render_credit_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("Credits")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.size());

    let credits_text = vec![
        Line::from(""),
        Line::from("Hi 👋, I'm Thomas, a 21-year-old French computer science student."),
        Line::from("Thank you for playing Chess-tui! This project started as a personal journey to improve my algorithmic skills and learn Rust."),
        Line::from(""),
        Line::from("The entire source code is available on GitHub at https://github.com/thomas-mauran/chess-tui"),
        Line::from("Feel free to contribute by picking an issue or creating a new one."),
        Line::from(""),
        Line::from("Special thanks to my classmates for their support and inspiration!"),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
    ];

    // Assuming Alignment is an enum or struct you have defined

    let paragraph = Paragraph::new(credits_text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This render the help popup
pub fn render_help_popup(frame: &mut Frame) {
    let block = Block::default()
        .title("Help menu")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 65, frame.size());

    let text = vec![
        Line::from("Game controls:".underlined().bold()),
        Line::from(""),
        Line::from(vec![
            "←/h ↑/k ↓/j →/l: Use these keys to move the ".into(),
            "blue".blue(),
            " cursor".into(),
        ]),
        Line::from(""),
        Line::from("`Ctrl` '+' or '-': Zoom in or out to adjust pieces sizes"),
        Line::from("(Might differ in certain terminals)"),
        Line::from(""),
        Line::from("`Space`: Select a piece"),
        Line::from(""),
        Line::from("`Esc`: Deselect a piece / hide popups"),
        Line::from(""),
        Line::from("`P` for previous position, `N` for next"),
        Line::from(""),
        Line::from("q: Quit the game"),
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
        Line::from("Grey cell: Available cells for the selected piece"),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
