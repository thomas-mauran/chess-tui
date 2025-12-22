use std::net::IpAddr;

use crate::{
    app::App,
    constants::{NETWORK_PORT, WHITE},
    pieces::{bishop::Bishop, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook, PieceSize},
    ui::main_ui::{centered_rect, render_cell},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use shakmaty::Color as ShakmatyColor;

use super::prompt::Prompt;

// This renders a confirmation popup for resigning a game
pub fn render_resign_confirmation_popup(frame: &mut Frame, app: &App) {
    let block = Block::default()
        .title("Resign Game")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(50, 30, frame.area());

    let opponent_name = if let Some(game) = app.ongoing_games.get(app.menu_cursor as usize) {
        format!("vs {}", game.opponent.username)
    } else {
        "this game".to_string()
    };

    let text = vec![
        Line::from(""),
        Line::from(format!(
            "Are you sure you want to resign {}?",
            opponent_name
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from("This action cannot be undone.").alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Y",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("es / "),
            Span::styled(
                "N",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw("o"),
        ])
        .alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a generic error popup with a custom message
pub fn render_error_popup(frame: &mut Frame, error_message: &str) {
    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(Color::Red));
    let area = centered_rect(50, 30, frame.area());

    // Split the error message by newlines and create a Line for each part
    let mut text = vec![Line::from("")];

    for line in error_message.split('\n') {
        if !line.is_empty() {
            text.push(Line::from(line).alignment(Alignment::Center));
        } else {
            text.push(Line::from(""));
        }
    }

    text.extend(vec![
        Line::from(""),
        Line::from("Press `Esc` or `Enter` to close.").alignment(Alignment::Center),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a generic success popup with a custom message
pub fn render_success_popup(frame: &mut Frame, success_message: &str) {
    let block = Block::default()
        .title("Success")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(Color::Green));
    let area = centered_rect(50, 30, frame.area());

    // Split the success message by newlines and create a Line for each part
    let mut text = vec![Line::from("")];

    for line in success_message.split('\n') {
        if !line.is_empty() {
            text.push(Line::from(line).alignment(Alignment::Center));
        } else {
            text.push(Line::from(""));
        }
    }

    text.extend(vec![
        Line::from(""),
        Line::from("Press `Esc` or `Enter` to close.").alignment(Alignment::Center),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a popup for a promotion
pub fn render_end_popup(frame: &mut Frame, sentence: &str, is_lichess: bool) {
    let block = Block::default()
        .title("Game Over")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::horizontal(2))
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::DarkGray));
    let area = centered_rect(50, 50, frame.area());

    // Create styled text with better formatting
    let mut text = vec![
        Line::from(""),
        Line::from(""),
        Line::from(sentence).alignment(Alignment::Center).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from(""),
        Line::from("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
            .style(Style::default().fg(Color::Gray)),
        Line::from(""),
        Line::from("Press `H` to hide this screen")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightBlue)),
    ];

    // Only show restart option for non-Lichess games (Lichess games can't be restarted)
    if !is_lichess {
        text.push(Line::from(""));
        text.push(
            Line::from("Press `R` to restart a new game")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::LightGreen)),
        );
    }

    text.push(Line::from(""));
    text.push(
        Line::from("Press `B` to go back to the menu")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightCyan)),
    );
    text.push(Line::from(""));
    text.push(Line::from(""));

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a popup for puzzle completion
pub fn render_puzzle_end_popup(
    frame: &mut Frame,
    sentence: &str,
    elo_change: Option<i32>,
    is_calculating: bool,
) {
    let block = Block::default()
        .title("Puzzle Complete")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::horizontal(2))
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::DarkGray));
    let area = centered_rect(50, 50, frame.area());

    // Create styled text with better formatting
    let mut text = vec![
        Line::from(""),
        Line::from(""),
        Line::from(sentence).alignment(Alignment::Center).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    // Add Elo change if available, or show calculating message
    if let Some(change) = elo_change {
        text.push(Line::from(""));
        let (change_text, color) = if change > 0 {
            (format!("+{} Elo", change), Color::Green)
        } else if change < 0 {
            (format!("{} Elo", change), Color::Red)
        } else {
            ("+0 Elo".to_string(), Color::Yellow)
        };
        text.push(
            Line::from(change_text)
                .alignment(Alignment::Center)
                .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
        );
    } else if is_calculating {
        text.push(Line::from(""));
        text.push(
            Line::from("Calculating Elo change...")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Cyan)),
        );
    }

    text.extend(vec![
        Line::from(""),
        Line::from(""),
        Line::from("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
            .style(Style::default().fg(Color::Gray)),
        Line::from(""),
        Line::from("Press `H` or `Esc` to hide this screen")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightBlue)),
        Line::from(""),
        Line::from("Press `N` for a new puzzle")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightGreen)),
        Line::from(""),
        Line::from("Press `B` to go back to the menu")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightCyan)),
        Line::from(""),
        Line::from(""),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a popup for a promotion
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

    let display_mode = &app.game.ui.display_mode;
    let piece_size = PieceSize::from_dimensions(inner_popup_layout_horizontal[0].height);
    let piece_color = Some(app.game.logic.player_turn);

    let queen_p = Paragraph::new(Queen::to_string(display_mode, piece_size, piece_color))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.game.ui.promotion_cursor == 0 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(queen_p, inner_popup_layout_horizontal[0]);
    let rook_p = Paragraph::new(Rook::to_string(display_mode, piece_size, piece_color))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.game.ui.promotion_cursor == 1 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(rook_p, inner_popup_layout_horizontal[1]);
    let bishop_p = Paragraph::new(Bishop::to_string(display_mode, piece_size, piece_color))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.game.ui.promotion_cursor == 2 {
            Color::LightBlue
        } else {
            Color::Reset // Set to the default background color when the condition is false
        }));
    frame.render_widget(bishop_p, inner_popup_layout_horizontal[2]);
    let knight_p = Paragraph::new(Knight::to_string(display_mode, piece_size, piece_color))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(if app.game.ui.promotion_cursor == 3 {
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
    let area = centered_rect(40, 40, frame.area());

    let credits_text = vec![
        Line::from(""),
        Line::from("Hi 👋, I'm Thomas, a 23 years old French computer science student."),
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
pub fn render_help_popup(frame: &mut Frame, app: &crate::app::App) {
    let block = Block::default()
        .title("Help menu")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 65, frame.area());

    // Check if we're playing against a bot (history navigation only in solo mode)
    let is_solo_mode = app.game.logic.bot.is_none() && app.game.logic.opponent.is_none();
    let is_puzzle_mode = app.puzzle_game.is_some();

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

// This renders a popup for the color selection
pub fn render_color_selection_popup(frame: &mut Frame, app: &App) {
    let block = Block::default()
        .title("Color selection")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let text = vec![
        Line::from(""),
        Line::from("-- Choose your color --").alignment(Alignment::Center),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);

    let inner_popup_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Min(3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(area);

    let inner_popup_layout_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(inner_popup_layout_vertical[1]);

    let display_mode = &app.game.ui.display_mode;

    // White option: white pawn on black background (like a chess square)
    let white_selected = app.menu_cursor == 0;
    let white_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if white_selected {
            Color::Yellow
        } else {
            Color::DarkGray
        }));
    let white_area = inner_popup_layout_horizontal[0];
    frame.render_widget(white_block.clone(), white_area);

    let white_inner_area = white_block.inner(white_area);
    let piece_size = PieceSize::from_dimensions(white_inner_area.height);

    // Render background only in the inner area (inside the border)
    if white_selected {
        render_cell(frame, white_inner_area, Color::Black, None);
    }

    let white_pawn = Paragraph::new(Pawn::to_string(
        display_mode,
        piece_size,
        Some(ShakmatyColor::White),
    ))
    .block(Block::default())
    .alignment(Alignment::Center)
    .style(
        Style::default()
            .fg(Color::White)
            .bg(if white_selected {
                Color::Black
            } else {
                Color::Reset
            })
            .add_modifier(if white_selected {
                Modifier::BOLD
            } else {
                Modifier::empty()
            }),
    );
    frame.render_widget(white_pawn, white_inner_area);

    // Black option: black pawn on white background (like a chess square)
    let black_selected = app.menu_cursor == 1;
    let black_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if black_selected {
            Color::Yellow
        } else {
            Color::DarkGray
        }));
    let black_area = inner_popup_layout_horizontal[2];
    frame.render_widget(black_block.clone(), black_area);

    let black_inner_area = black_block.inner(black_area);
    let black_piece_size = PieceSize::from_dimensions(black_inner_area.height);

    // Render background only in the inner area (inside the border)
    if black_selected {
        render_cell(frame, black_inner_area, Color::White, None);
    }

    let black_pawn = Paragraph::new(Pawn::to_string(
        display_mode,
        black_piece_size,
        Some(ShakmatyColor::Black),
    ))
    .block(Block::default())
    .alignment(Alignment::Center)
    .style(
        Style::default()
            .fg(Color::Black)
            .bg(if black_selected {
                Color::White
            } else {
                Color::Reset
            })
            .add_modifier(if black_selected {
                Modifier::BOLD
            } else {
                Modifier::empty()
            }),
    );
    frame.render_widget(black_pawn, black_inner_area);

    // Labels under each option to make the choice explicit and readable on any theme
    let label_layout_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(inner_popup_layout_vertical[2]);

    let white_label = Paragraph::new(" White pieces ")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(if white_selected {
                    Color::Yellow
                } else {
                    Color::Gray
                })
                .add_modifier(if app.menu_cursor == 0 {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        );
    frame.render_widget(white_label, label_layout_horizontal[0]);

    let black_label = Paragraph::new(" Black pieces ")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(if black_selected {
                    Color::Yellow
                } else {
                    Color::Gray
                })
                .add_modifier(if app.menu_cursor == 1 {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        );
    frame.render_widget(black_label, label_layout_horizontal[2]);
}

// This renders a popup for the multiplayer hosting / joining popup
pub fn render_multiplayer_selection_popup(frame: &mut Frame, app: &App) {
    let block: Block<'_> = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let text = vec![
        Line::from(""),
        Line::from("-- Are you hosting or joining a game ? --").alignment(Alignment::Center),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    frame.render_widget(Clear, area);
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
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(inner_popup_layout_vertical[1]);

    let hosting = Paragraph::new(Text::from(vec![Line::from(vec![Span::styled(
        "HOSTING",
        Style::default().add_modifier(if app.menu_cursor == 0 {
            Modifier::UNDERLINED
        } else {
            Modifier::empty()
        }),
    )])]))
    .block(Block::default())
    .alignment(Alignment::Center);

    frame.render_widget(hosting, inner_popup_layout_horizontal[0]);

    let joining = Paragraph::new(Text::from(vec![Line::from(vec![Span::styled(
        "JOINING",
        Style::default().add_modifier(if app.menu_cursor == 1 {
            Modifier::UNDERLINED
        } else {
            Modifier::empty()
        }),
    )])]))
    .block(Block::default())
    .alignment(Alignment::Center);
    frame.render_widget(joining, inner_popup_layout_horizontal[2]);
}

// MULTIPLAYER POPUPS
// This renders a popup indicating we are waiting for the other player
pub fn render_wait_for_other_player(frame: &mut Frame, ip: Option<IpAddr>) {
    let block = Block::default()
        .title("Waiting ...")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let ip_str = ip
        .map(|i| i.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let text = vec![
        Line::from(""),
        Line::from(""),
        Line::from("Waiting for other player").alignment(Alignment::Center),
        Line::from(format!(
            "Host IP address and port: {}:{}",
            ip_str, NETWORK_PORT
        ))
        .alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a popup allowing us to get a user input
pub fn render_enter_multiplayer_ip(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Join a game")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(40, 40, frame.area());

    let current_input = prompt.input.as_str();

    let text = vec![
        Line::from("Enter the ip address and port of the host:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(format!("Example: 10.111.6.50:{};", NETWORK_PORT)),
        Line::from("Documentation: https://thomas-mauran.github.io/chess-tui/docs/Multiplayer/Online%20multiplayer/"),
        Line::from(""),
        Line::from(""),
        Line::from("Press `Esc` to close the popup.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        area.x + prompt.character_index as u16 + 2,
        // Move one line down, from the border to the input line
        area.y + 3,
    ));

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a popup allowing us to enter a game code to join a Lichess game
pub fn render_enter_game_code_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Join Lichess Game")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(50, 30, frame.area());

    let current_input = prompt.input.as_str();

    let text = vec![
        Line::from("Enter game ID or URL:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(current_input),
        Line::from(""),
        Line::from(""),
        Line::from("You can paste the full URL or just the game ID."),
        Line::from("Note: You must be a participant in the game."),
        Line::from(""),
        Line::from("Press `Esc` to cancel.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        area.x + prompt.character_index as u16 + 2,
        // Move one line down, from the border to the input line
        area.y + 3,
    ));

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}

// This renders a popup allowing us to enter a Lichess API token
pub fn render_enter_lichess_token_popup(frame: &mut Frame, prompt: &Prompt) {
    let block = Block::default()
        .title("Enter Lichess API Token")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
        .border_style(Style::default().fg(WHITE));
    let area = centered_rect(70, 40, frame.area());

    let current_input = prompt.input.as_str();
    // Mask the token for security (show only last 4 characters)
    let masked_input = if current_input.len() > 4 {
        format!(
            "{}...{}",
            "*".repeat(current_input.len().saturating_sub(4)),
            &current_input[current_input.len().saturating_sub(4)..]
        )
    } else if !current_input.is_empty() {
        "*".repeat(current_input.len())
    } else {
        String::new()
    };

    let text = vec![
        Line::from("Enter your Lichess API token:").alignment(Alignment::Center),
        Line::from(""),
        Line::from(masked_input),
        Line::from(""),
        Line::from(""),
        Line::from("To get a token, follow the documentation:"),
        Line::from("Documentation: https://thomas-mauran.github.io/chess-tui/docs/Lichess/setup"),
        Line::from(""),
        Line::from("Press `Enter` to save, `Esc` to cancel.").alignment(Alignment::Center),
    ];

    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        area.x + prompt.character_index as u16 + 2,
        // Move one line down, from the border to the input line
        area.y + 3,
    ));

    frame.render_widget(Clear, area); //this clears out the background
    frame.render_widget(block, area);
    frame.render_widget(paragraph, area);
}
