//! In-game board and piece rendering.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Rect},
    style::{Style},
    widgets::{Block},
    Frame,
};

use crate::{
    app::App,
    constants::Popups,
    game_logic::game::GameState,
    ui::popup::end::render_end_popup,
    ui::popup::promotion::render_promotion_popup,
};
/// Renders the game board, clock, move history, and any in-game popups.
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

    // Create layout for board + file labels + clock
    let has_clock = app.game.logic.clock.is_some();
    let board_with_labels = if has_clock {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1), // Clock area (small text)
                    Constraint::Min(0),    // Board (takes remaining space)
                    Constraint::Length(1), // File labels (A-H) - minimal height
                ]
                .as_ref(),
            )
            .split(main_layout_vertical[2])
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(0),    // Board (takes remaining space)
                    Constraint::Length(1), // File labels (A-H) - minimal height
                ]
                .as_ref(),
            )
            .split(main_layout_vertical[2])
    };

    // Render clocks above board if present (small text, no borders)
    if has_clock {
        let clock_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 2), // White clock (left)
                    Constraint::Ratio(1, 2), // Black clock (right)
                ]
                .as_ref(),
            )
            .split(board_with_labels[0]);

        use ratatui::style::{Color, Modifier, Style};
        use ratatui::widgets::Paragraph;

        // White clock (left) - just text
        if let Some(ref clock) = app.game.logic.clock {
            let white_time = clock.format_time(shakmaty::Color::White);
            let is_white_active =
                clock.is_running && clock.active_color == Some(shakmaty::Color::White);
            let white_text = format!("White: {}", white_time);
            let text_width = white_text.len() as u16;

            if is_white_active {
                // Render background block only for text width
                let text_area = Rect {
                    x: clock_area[0].x,
                    y: clock_area[0].y,
                    width: text_width.min(clock_area[0].width),
                    height: clock_area[0].height,
                };
                let bg_block = Block::default().style(Style::default().bg(Color::White));
                frame.render_widget(bg_block, text_area);
            }

            let white_clock_style = if is_white_active {
                Style::default()
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let white_clock_text = Paragraph::new(white_text)
                .style(white_clock_style)
                .alignment(ratatui::layout::Alignment::Left);
            frame.render_widget(white_clock_text, clock_area[0]);
        }

        // Black clock (right) - just text
        if let Some(ref clock) = app.game.logic.clock {
            let black_time = clock.format_time(shakmaty::Color::Black);
            let is_black_active =
                clock.is_running && clock.active_color == Some(shakmaty::Color::Black);
            let black_text = format!("Black: {}", black_time);
            let text_width = black_text.len() as u16;

            if is_black_active {
                // Render background block only for text width, aligned to the right
                let text_area = Rect {
                    x: clock_area[1].x + clock_area[1].width.saturating_sub(text_width),
                    y: clock_area[1].y,
                    width: text_width.min(clock_area[1].width),
                    height: clock_area[1].height,
                };
                let bg_block = Block::default().style(Style::default().bg(Color::White));
                frame.render_widget(bg_block, text_area);
            }

            let black_clock_style = if is_black_active {
                Style::default()
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let black_clock_text = Paragraph::new(black_text)
                .style(black_clock_style)
                .alignment(ratatui::layout::Alignment::Right);
            frame.render_widget(black_clock_text, clock_area[1]);
        }
    }

    let board_index = if has_clock { 1 } else { 0 };
    let file_labels_index = if has_clock { 2 } else { 1 };

    // Split rank label area to match board height
    let rank_label_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),    // Rank labels (aligned with board - takes remaining space)
                Constraint::Length(1), // Empty space (aligned with file labels - minimal)
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
    frame.render_widget(board_block.clone(), board_with_labels[board_index]);

    // Split borrows to avoid borrow checker issue
    let (ui, logic) = (&mut app.game.ui, &app.game.logic);

    // Get the inner area of the board (accounting for any block padding)
    let board_inner = board_block.inner(board_with_labels[board_index]);
    ui.board_render(board_inner, frame, logic);

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
