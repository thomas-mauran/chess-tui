//! In-game board and piece rendering.

use crate::{
    app::App,
    constants::Popups,
    game_logic::{
        clock::{Clock, ClockState},
        game::GameState,
    },
    ui::popup::{end::render_end_popup, promotion::render_promotion_popup},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::Style,
    widgets::{Block, Paragraph},
};
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier},
};
use std::time::Instant;

/// Renders the game board, clock, move history, and any in-game popups.
pub fn render_game_ui(frame: &mut Frame<'_>, app: &mut App, main_area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 20),  // Top padding
            Constraint::Ratio(18, 20), // Board area (increased)
            Constraint::Min(0),        // Bottom padding (minimal)
        ])
        .split(main_area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 18),  // Left padding (reduced)
            Constraint::Ratio(1, 18),  // Rank labels (1-8)
            Constraint::Ratio(11, 18), // Board (increased from 9 to 11)
            Constraint::Ratio(1, 18),  // Right padding
            Constraint::Ratio(4, 18),  // Sidebar (reduced from 5 to 4)
        ])
        .split(main_layout_horizontal[1]);

    // Create layout for board + file labels + clock
    let has_clock = app.game.logic.clock.state() != ClockState::NotStarted;
    let board_with_labels = if has_clock {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Clock area (small text)
                Constraint::Min(0),    // Board (takes remaining space)
                Constraint::Length(1), // File labels (A-H) - minimal height
            ])
            .split(main_layout_vertical[2])
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Board (takes remaining space)
                Constraint::Length(1), // File labels (A-H) - minimal height
            ])
            .split(main_layout_vertical[2])
    };

    // Render clocks above board if present (small text, no borders)
    if has_clock {
        render_clock(
            frame,
            &app.game.logic.clock,
            board_with_labels[0],
            Instant::now(),
        );
    }

    let board_index = if has_clock { 1 } else { 0 };
    let file_labels_index = if has_clock { 2 } else { 1 };

    // Split rank label area to match board height
    let rank_label_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Rank labels (aligned with board - takes remaining space)
            Constraint::Length(1), // Empty space (aligned with file labels - minimal)
        ])
        .split(main_layout_vertical[1]);

    let right_box_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(2, 15),
            Constraint::Ratio(11, 15),
            Constraint::Ratio(2, 15),
        ])
        .split(main_layout_vertical[4]);
    // Board block representing the full board div
    let board_block = Block::default().style(Style::default());

    // We render the board_block in the center layout made above
    frame.render_widget(board_block.clone(), board_with_labels[board_index]);

    // Split borrows to avoid borrow checker issue
    let (ui, logic, kitty_pieces) = (&mut app.game.ui, &app.game.logic, &mut app.kitty_pieces);

    // Get the inner area of the board (accounting for any block padding)
    let board_inner = board_block.inner(board_with_labels[board_index]);
    ui.board_render(
        board_inner,
        frame,
        logic,
        kitty_pieces.as_mut().filter(|_| app.kitty_graphics_enabled),
    );

    // Render rank labels (1-8) on the left - aligned with board's inner area
    ui.render_rank_labels(frame, rank_label_area[0], logic.game_board.is_flipped);

    // Render file labels (A-H) below the board
    ui.render_file_labels(
        frame,
        board_with_labels[file_labels_index],
        logic.game_board.is_flipped,
    );

    //top box for black material
    let black_taken = app.game.logic.game_board.black_taken_pieces();
    app.game
        .ui
        .black_material_render(board_block.inner(right_box_layout[0]), frame, &black_taken);

    // History area
    app.game
        .ui
        .history_render(board_block.inner(right_box_layout[1]), frame, &app.game);

    //bottom box for white material
    let white_taken = app.game.logic.game_board.white_taken_pieces();
    let is_puzzle_mode = app.lichess_state.puzzle_game.is_some();
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
        // When game ended by time, player_turn is already set to the winner
        // For checkmate, the winner is the other player (the one who delivered checkmate)
        let victorious_player = if app.game.logic.game_ended_by_time {
            app.game.logic.player_turn
        } else {
            app.game.logic.player_turn.other()
        };

        let string_color = match victorious_player {
            shakmaty::Color::White => "White",
            shakmaty::Color::Black => "Black",
        };

        if app.ui_state.current_popup == Some(Popups::EndScreen) {
            // Check if it's Lichess multiplayer (restart not available in Lichess)
            let is_lichess = app
                .game
                .logic
                .opponent
                .as_ref()
                .map(|opp| opp.is_lichess())
                .unwrap_or(false);

            // Check if game ended due to time
            let message = if app.game.logic.game_ended_by_time {
                let time_up_color = match victorious_player {
                    shakmaty::Color::White => "Black",
                    shakmaty::Color::Black => "White",
                };
                format!("{string_color} Won \n{time_up_color} ran out of time")
            } else {
                format!("{string_color} Won")
            };

            render_end_popup(frame, &message, is_lichess);
        }
    }

    if app.game.logic.game_state == GameState::Draw
        && app.ui_state.current_popup == Some(Popups::EndScreen)
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

/// Render the clock at the top of the board
pub fn render_clock(frame: &mut Frame<'_>, clock: &Clock, area: Rect, now: Instant) {
    let clock_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 2), // White clock (left)
            Constraint::Ratio(1, 2), // Black clock (right)
        ])
        .split(area);

    let active_color = match clock.state() {
        ClockState::Running { active_color, .. } | ClockState::Paused { active_color } => {
            Some(active_color)
        }
        _ => None,
    };

    let clock_configs = [
        (
            shakmaty::Color::White,
            clock_area[0],
            Alignment::Left,
            "White",
        ),
        (
            shakmaty::Color::Black,
            clock_area[1],
            Alignment::Right,
            "Black",
        ),
    ];

    for (color, section_area, alignment, label) in clock_configs {
        let time_str = clock.format_time(color, now);
        let text = format!("{}: {}", label, time_str);
        let text_width = text.len() as u16;
        let is_active = active_color == Some(color);

        if is_active {
            let x_offset = if alignment == Alignment::Right {
                section_area.width.saturating_sub(text_width)
            } else {
                0
            };

            let text_area = Rect {
                x: section_area.x + x_offset,
                y: section_area.y,
                width: text_width.min(section_area.width),
                height: section_area.height,
            };

            let bg_block = Block::default().style(Style::default().bg(Color::White));
            frame.render_widget(bg_block, text_area);
        }

        let style = if is_active {
            Style::default()
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let paragraph = Paragraph::new(text).style(style).alignment(alignment);
        frame.render_widget(paragraph, section_area);
    }
}
