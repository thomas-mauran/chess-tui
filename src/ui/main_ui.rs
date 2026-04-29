//! Top-level frame composition.

use ratatui::{
    prelude::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    constants::Popups,
    server::game_server::{get_host_ip, setup_game_server},
    ui::{
        components::centered_rect::centered_rect,
        game_ui::render_game_ui,
        menu::{
            game_mode_menu::render_game_mode_menu, lichess_menu::render_lichess_menu,
            main_menu::render_menu_ui,
        },
        popup::{
            credits::render_credit_popup,
            error::render_error_popup,
            help::render_help_popup,
            lichess::{
                game_code::render_enter_game_code_popup, puzzle_end::render_puzzle_end_popup,
                token::render_enter_lichess_token_popup,
            },
            move_input::render_move_input_popup,
            multiplayer::{
                enter_ip::render_enter_multiplayer_ip,
                wait_for_player::render_wait_for_other_player,
            },
            pgn::path::render_load_pgn_popup,
            resignation::render_resign_confirmation_popup,
            success::render_success_popup,
        },
    },
};

use super::ongoing_games::render_ongoing_games;
use super::pgn_viewer_ui::render_pgn_viewer;

use crate::{app::App, constants::Pages};
use std::path::Path;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let main_area = frame.area();

    match app.ui_state.current_page {
        Pages::Solo => render_game_ui(frame, app, main_area),
        Pages::Multiplayer => render_multiplayer_page(frame, app, main_area),
        Pages::Lichess => {
            if app.game.logic.opponent.is_some() {
                render_game_ui(frame, app, main_area);
            }
        }
        Pages::Bot => render_bot_page(frame, app, main_area),
        Pages::LichessMenu => render_lichess_menu(frame, app),
        Pages::GameModeMenu => render_game_mode_menu(frame, app),
        Pages::OngoingGames => render_ongoing_games(frame, app),
        Pages::PgnViewer => render_pgn_viewer(frame, app),
        Pages::Home | Pages::Credit => render_menu_ui(frame, app, main_area),
    }

    if app.ui_state.current_page == Pages::Credit {
        render_credit_popup(frame);
    }

    // Render popups
    match app.ui_state.current_popup {
        Some(Popups::EnterHostIP) => {
            render_enter_multiplayer_ip(frame, &app.game.ui.prompt);
        }
        Some(Popups::MoveInputSelection) => render_move_input_popup(frame, &app.game.ui.prompt),
        Some(Popups::WaitingForOpponentToJoin) => {
            render_wait_for_other_player(frame, get_host_ip());
        }
        Some(Popups::Help) => {
            render_help_popup(frame, app);
        }
        Some(Popups::Error) => {
            if let Some(ref msg) = app.ui_state.popup_message {
                render_error_popup(frame, msg);
            }
        }
        Some(Popups::Success) => {
            if let Some(ref msg) = app.ui_state.popup_message {
                render_success_popup(frame, msg);
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
        Some(Popups::LoadPgnPath) => {
            render_load_pgn_popup(frame, &app.game.ui.prompt);
        }
        Some(Popups::EnterLichessToken) => {
            render_enter_lichess_token_popup(frame, &app.game.ui.prompt);
        }
        Some(Popups::ResignConfirmation) => {
            render_resign_confirmation_popup(frame, app);
        }
        Some(Popups::PuzzleEndScreen) => {
            // Show puzzle completion message
            let message = if let Some(ref msg) = app.ui_state.popup_message {
                msg.clone()
            } else {
                "Puzzle solved! Well done!".to_string()
            };

            // Check if we're still waiting for Elo change calculation
            let (elo_change, is_calculating) = if let Some(puzzle_game) =
                &app.lichess_state.puzzle_game
            {
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

fn render_multiplayer_page(frame: &mut Frame<'_>, app: &mut App, main_area: ratatui::layout::Rect) {
    app.game_mode_state.resolve_selected_color();
    if app.ui_state.current_popup == Some(Popups::Error) {
        return;
    }
    if app.game.logic.opponent.is_none() {
        if app.multiplayer_state.host_ip.is_none() {
            if app.multiplayer_state.hosting == Some(true) {
                if let Some(color) = app.game_mode_state.selected_color {
                    setup_game_server(color);
                    app.multiplayer_state.host_ip = Some("127.0.0.1".to_string());
                }
            } else {
                app.ui_state.current_popup = Some(Popups::EnterHostIP);
            }
        } else {
            app.create_opponent();
        }
    } else if app
        .game
        .logic
        .opponent
        .as_ref()
        .is_some_and(|o| o.game_started)
    {
        render_game_ui(frame, app, main_area);
    }
}

fn render_bot_page(frame: &mut Frame<'_>, app: &mut App, main_area: ratatui::layout::Rect) {
    if app
        .bot_state
        .chess_engine_path
        .as_ref()
        .is_none_or(|p| p.is_empty())
    {
        app.ui_state.popup_message = Some(
            "Chess engine path not configured.\n\n".to_string()
                + "To configure the chess engine follow the documentation: https://thomas-mauran.github.io/chess-tui/docs/Configuration/bot\n\n"
                + "Example:\n"
                + "chess-tui -e /opt/homebrew/opt/stockfish\n"
                + " or execute the script: ./scripts/install-stockfish.sh\n"
                + "to install stockfish automatically and set it as the chess engine path \n",
        );
        app.ui_state.current_popup = Some(Popups::Error);
    } else if let Some(engine_path) = &app.bot_state.chess_engine_path {
        let command_path = engine_path.split_whitespace().next().unwrap_or(engine_path);
        if !Path::new(command_path).exists() || !Path::new(command_path).is_file() {
            app.ui_state.popup_message = Some(format!(
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
            ));
            app.ui_state.current_popup = Some(Popups::Error);
        }
    }

    if app.ui_state.current_popup != Some(Popups::Error) {
        app.game_mode_state.resolve_selected_color();
        if app.game.logic.bot.is_none() {
            app.bot_setup();
        } else {
            render_game_ui(frame, app, main_area);
        }
    }
}
