use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    constants::Popups,
    game_logic::game::GameState,
    ui::popups::{
        render_color_selection_popup, render_credit_popup, render_end_popup,
        render_enter_game_code_popup, render_enter_lichess_token_popup, render_error_popup,
        render_help_popup, render_promotion_popup, render_puzzle_end_popup,
        render_resign_confirmation_popup, render_success_popup,
    },
};

use super::lichess_menu::render_lichess_menu;
use super::ongoing_games::render_ongoing_games;
use super::popups::{
    render_enter_multiplayer_ip, render_multiplayer_selection_popup, render_wait_for_other_player,
};
use crate::{
    app::App,
    constants::{DisplayMode, Pages, TITLE},
};
use std::path::Path;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let main_area = frame.area();

    // Solo game
    if app.current_page == Pages::Solo {
        render_game_ui(frame, app, main_area);
    }
    // Multiplayer game
    else if app.current_page == Pages::Multiplayer {
        // Don't override Error popup
        if app.current_popup != Some(Popups::Error) {
            if app.hosting.is_none() {
                app.current_popup = Some(Popups::MultiplayerSelection);
            } else if app.selected_color.is_none() && app.hosting == Some(true) {
                app.current_popup = Some(Popups::ColorSelection);
            } else if app.game.logic.opponent.is_none() {
                if app.host_ip.is_none() {
                    if app.hosting == Some(true) {
                        if let Some(color) = app.selected_color {
                            app.setup_game_server(color);
                            app.host_ip = Some("127.0.0.1".to_string());
                        }
                    } else {
                        app.current_popup = Some(Popups::EnterHostIP);
                    }
                } else {
                    app.create_opponent();
                }
            } else if app
                .game
                .logic
                .opponent
                .as_ref()
                .is_some_and(|opponent| opponent.game_started)
            {
                render_game_ui(frame, app, main_area);
            }
        }
    }
    // Lichess game
    else if app.current_page == Pages::Lichess {
        if app.game.logic.opponent.is_some() {
            render_game_ui(frame, app, main_area);
        }
    }
    // Play against bot
    else if app.current_page == Pages::Bot {
        // Check if engine path is configured
        if app
            .chess_engine_path
            .as_ref()
            .is_none_or(|path| path.is_empty())
        {
            app.error_message = Some(
                "Chess engine path not configured.\n\n".to_string()
                    + "To configure the chess engine follow the documentation: https://thomas-mauran.github.io/chess-tui/docs/Configuration/bot\n\n"
                    + "Example:\n"
                    + "chess-tui -e /opt/homebrew/opt/stockfish\n"
                    + " or execute the script: ./scripts/install-stockfish.sh\n"
                    + "to install stockfish automatically and set it as the chess engine path \n"
            );
            app.current_popup = Some(Popups::Error);
        }
        // Check if engine path exists and is a valid file
        else if let Some(engine_path) = &app.chess_engine_path {
            // Extract just the command path (first part before any arguments)
            let command_path = engine_path.split_whitespace().next().unwrap_or(engine_path);
            let path = Path::new(command_path);
            if !path.exists() || !path.is_file() {
                app.error_message = Some(
                    format!(
                        "Chess engine path is invalid.\n\n\
                        The configured path does not exist or is not a file:\n\
                        {}\n\n\
                        Please check the path and update it in your configuration.\n\n\
                        To configure the chess engine follow the documentation: https://thomas-mauran.github.io/chess-tui/docs/Configuration/bot\n\n\
                        Example:\n\
                        chess-tui -e /opt/homebrew/bin/stockfish\n\
                        or execute the script: ./scripts/install-stockfish.sh\n\
                        to install stockfish automatically and set it as the chess engine path",
                        command_path
                    )
                );
                app.current_popup = Some(Popups::Error);
            }
        }

        // If we passed all validation checks, proceed with bot setup
        if app.current_popup != Some(Popups::Error) {
            if app.selected_color.is_none() {
                app.current_popup = Some(Popups::ColorSelection);
            } else if app.game.logic.bot.is_none() {
                app.bot_setup();
            } else {
                render_game_ui(frame, app, main_area);
            }
        }
    }
    // Lichess menu
    else if app.current_page == Pages::LichessMenu {
        render_lichess_menu(frame, app);
    }
    // Ongoing games list
    else if app.current_page == Pages::OngoingGames {
        render_ongoing_games(frame, app);
    }
    // Render menu
    else {
        render_menu_ui(frame, app, main_area);
    }

    if app.current_page == Pages::Credit {
        render_credit_popup(frame);
    }

    // Render popups
    match app.current_popup {
        Some(Popups::ColorSelection) => {
            render_color_selection_popup(frame, app);
        }
        Some(Popups::MultiplayerSelection) => {
            render_multiplayer_selection_popup(frame, app);
        }
        Some(Popups::EnterHostIP) => {
            render_enter_multiplayer_ip(frame, &app.game.ui.prompt);
        }
        Some(Popups::WaitingForOpponentToJoin) => {
            render_wait_for_other_player(frame, app.get_host_ip());
        }
        Some(Popups::Help) => {
            render_help_popup(frame, app);
        }
        Some(Popups::Error) => {
            if let Some(ref error_msg) = app.error_message {
                render_error_popup(frame, error_msg);
            }
        }
        Some(Popups::Success) => {
            if let Some(ref success_msg) = app.error_message {
                render_success_popup(frame, success_msg);
            }
        }
        Some(Popups::SeekingLichessGame) => {
            let popup_area = centered_rect(60, 20, main_area);
            let block = Block::default()
                .title("Lichess")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::DarkGray));
            let paragraph = Paragraph::new("Seeking a game on Lichess...\n(Press 'Esc' to cancel)")
                .block(block)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, popup_area);
        }
        Some(Popups::EnterGameCode) => {
            render_enter_game_code_popup(frame, &app.game.ui.prompt);
        }
        Some(Popups::EnterLichessToken) => {
            render_enter_lichess_token_popup(frame, &app.game.ui.prompt);
        }
        Some(Popups::ResignConfirmation) => {
            render_resign_confirmation_popup(frame, app);
        }
        Some(Popups::PuzzleEndScreen) => {
            // Show puzzle completion message
            let message = if let Some(ref error_msg) = app.error_message {
                error_msg.clone()
            } else {
                "Puzzle solved! Well done!".to_string()
            };

            // Check if we're still waiting for Elo change calculation
            let (elo_change, is_calculating) = if let Some(puzzle_game) = &app.puzzle_game {
                (
                    puzzle_game.elo_change,
                    puzzle_game.elo_change.is_none() && puzzle_game.elo_change_receiver.is_some(),
                )
            } else {
                (None, false)
            };

            render_puzzle_end_popup(frame, &message, elo_change, is_calculating);
        }
        _ => {}
    }
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
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

pub fn render_cell(frame: &mut Frame, square: Rect, color: Color, modifier: Option<Modifier>) {
    let mut cell = Block::default().bg(color);
    if let Some(modifier) = modifier {
        cell = cell.add_modifier(modifier);
    }
    frame.render_widget(cell, square);
}

// Method to render the home menu and the options
pub fn render_menu_ui(frame: &mut Frame, app: &App, main_area: Rect) {
    // Determine the "skin" text
    let display_mode_menu = {
        let skin_name = match app.game.ui.display_mode {
            DisplayMode::DEFAULT => "Default",
            DisplayMode::ASCII => "ASCII",
            DisplayMode::CUSTOM => app.game.ui.skin.name.as_str(),
        };
        format!("Skin: {skin_name}")
    };

    // Determine the "sound" text (only if sound feature is enabled)
    #[cfg(feature = "sound")]
    let sound_menu = {
        let sound_status = if app.sound_enabled {
            "On 🔊"
        } else {
            "Off 🔇"
        };
        format!("Sound: {sound_status}")
    };

    // Menu items with descriptions
    let mut menu_items: Vec<(&str, &str)> = vec![
        ("Local game", "Practice mode - play against yourself"),
        ("Multiplayer", "Play with friends over network"),
        ("Lichess Online", "Play on Lichess.org"),
        ("Play Bot", "Challenge a chess engine"),
        (&display_mode_menu, "Change display theme"),
    ];

    // Add sound menu item only if sound feature is enabled
    #[cfg(feature = "sound")]
    {
        menu_items.push((&sound_menu, "Toggle sound effects"));
    }

    menu_items.extend(vec![
        ("Help", "View keyboard shortcuts and controls"),
        ("About", "Project information and credits"),
    ]);

    // Menu height depends on number of items, each takes 3 lines (item + description/empty + spacing), plus padding
    let menu_height = menu_items.len() as u16 * 3 + 4;

    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),         // Title
                Constraint::Length(1),           // Subtitle
                Constraint::Min(0),              // Flexible space above menu
                Constraint::Length(menu_height), // Menu (fixed height)
                Constraint::Min(0),              // Flexible space below menu
                Constraint::Ratio(1, 10),        // Footer/hints
            ]
            .as_ref(),
        )
        .split(main_area);

    // Title
    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, main_layout_horizontal[0]);

    // Subtitle
    let sub_title = Paragraph::new("A chess game made in 🦀")
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, main_layout_horizontal[1]);

    // Menu area (centered in the middle section)
    let menu_area = main_layout_horizontal[3];

    // Render menu items
    let mut menu_lines: Vec<Line<'_>> = vec![];

    for (i, (item, description)) in menu_items.iter().enumerate() {
        let is_selected = app.menu_cursor == i as u8;

        // Create styled menu item
        let item_style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Menu item line with indicator
        let indicator = if is_selected { "▶ " } else { "  " };
        let item_text = format!("{}{}", indicator, item);
        menu_lines.push(Line::from(vec![Span::styled(item_text, item_style)]));

        // Description line (only for selected item to save space)
        if is_selected {
            menu_lines.push(Line::from(vec![Span::styled(
                format!("   {}", description),
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            )]));
        } else {
            menu_lines.push(Line::from(""));
        }

        // Add spacing between menu items
        menu_lines.push(Line::from(""));
    }

    let menu_paragraph = Paragraph::new(menu_lines)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(menu_paragraph, menu_area);

    // Footer with keyboard hints
    let version = env!("CARGO_PKG_VERSION");
    let footer_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation: ", Style::default().fg(Color::Gray)),
            Span::styled("↑/k ", Style::default().fg(Color::Yellow)),
            Span::styled("↓/j ", Style::default().fg(Color::Yellow)),
            Span::styled("| Select: ", Style::default().fg(Color::Gray)),
            Span::styled("Enter/Space", Style::default().fg(Color::Yellow)),
            Span::styled(" | Help: ", Style::default().fg(Color::Gray)),
            Span::styled("?", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(format!("Version: {}", version))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray)),
    ];

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(footer, main_layout_horizontal[5]);
}

// Method to render the game board and handle game popups
pub fn render_game_ui(frame: &mut Frame<'_>, app: &mut App, main_area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 20),  // Top padding
                Constraint::Ratio(18, 20), // Board area (increased)
                Constraint::Min(0),        // Bottom padding (minimal)
            ]
            .as_ref(),
        )
        .split(main_area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 18),  // Left padding (reduced)
                Constraint::Ratio(1, 18),  // Rank labels (1-8)
                Constraint::Ratio(11, 18), // Board (increased from 9 to 11)
                Constraint::Ratio(1, 18),  // Right padding
                Constraint::Ratio(4, 18),  // Sidebar (reduced from 5 to 4)
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    // Create layout for board + file labels
    let board_with_labels = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(8, 9), // Board
                Constraint::Ratio(1, 9), // File labels (A-H)
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[2]);

    // Split rank label area to match board height
    let rank_label_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(8, 9), // Rank labels (aligned with board)
                Constraint::Ratio(1, 9), // Empty space (aligned with file labels)
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[1]);

    let right_box_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(2, 15),
                Constraint::Ratio(11, 15),
                Constraint::Ratio(2, 15),
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[4]);
    // Board block representing the full board div
    let board_block = Block::default().style(Style::default());

    // We render the board_block in the center layout made above
    frame.render_widget(board_block.clone(), board_with_labels[0]);

    // Split borrows to avoid borrow checker issue
    let (ui, logic) = (&mut app.game.ui, &app.game.logic);

    // Get the inner area of the board (accounting for any block padding)
    let board_inner = board_block.inner(board_with_labels[0]);
    ui.board_render(board_inner, frame, logic);

    // Render rank labels (1-8) on the left - aligned with board's inner area
    ui.render_rank_labels(frame, rank_label_area[0], logic.game_board.is_flipped);

    // Render file labels (A-H) below the board
    ui.render_file_labels(frame, board_with_labels[1], logic.game_board.is_flipped);

    //top box for white material
    let black_taken = app.game.logic.game_board.black_taken_pieces();
    app.game
        .ui
        .black_material_render(board_block.inner(right_box_layout[0]), frame, &black_taken);

    // We make the inside of the board
    app.game
        .ui
        .history_render(board_block.inner(right_box_layout[1]), frame, &app.game);

    //bottom box for black matetrial
    let white_taken = app.game.logic.game_board.white_taken_pieces();
    let is_puzzle_mode = app.puzzle_game.is_some();
    if is_puzzle_mode {
        app.game.ui.white_material_render_with_puzzle_hint(
            board_block.inner(right_box_layout[2]),
            frame,
            &white_taken,
            true,
        );
    } else {
        app.game.ui.white_material_render(
            board_block.inner(right_box_layout[2]),
            frame,
            &white_taken,
        );
    }

    if app.game.logic.game_state == GameState::Promotion {
        render_promotion_popup(frame, app);
    }

    // If the game ended (checkmate or draw) and there's no active popup yet,
    // open the EndScreen popup so it appears immediately instead of waiting for
    // another user interaction.
    if app.game.logic.game_state == GameState::Checkmate {
        let victorious_player = app.game.logic.player_turn.other();

        let string_color = match victorious_player {
            shakmaty::Color::White => "White",
            shakmaty::Color::Black => "Black",
        };

        if app.current_popup == Some(Popups::EndScreen) {
            // Check if it's Lichess multiplayer (restart not available in Lichess)
            let is_lichess = app
                .game
                .logic
                .opponent
                .as_ref()
                .map(|opp| opp.is_lichess())
                .unwrap_or(false);
            render_end_popup(frame, &format!("{string_color} Won !!!"), is_lichess);
        }
    }

    if app.game.logic.game_state == GameState::Draw && app.current_popup == Some(Popups::EndScreen)
    {
        // Check if it's Lichess multiplayer (restart not available in Lichess)
        let is_lichess = app
            .game
            .logic
            .opponent
            .as_ref()
            .map(|opp| opp.is_lichess())
            .unwrap_or(false);
        render_end_popup(frame, "That's a draw", is_lichess);
    }
}
