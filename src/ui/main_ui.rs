use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    constants::Popups,
    game_logic::game::GameState,
    ui::popups::{
        render_color_selection_popup, render_credit_popup, render_end_popup,
        render_engine_path_error_popup, render_help_popup, render_promotion_popup,
    },
};

use super::popups::{
    render_enter_multiplayer_ip, render_enter_pgn_path, render_multiplayer_selection_popup, render_wait_for_other_player,
};
use crate::{
    app::App,
    constants::{DisplayMode, Pages, TITLE},
    pieces::PieceColor,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let main_area = frame.area();

    // Solo game
    if app.current_page == Pages::Solo {
        render_game_ui(frame, app, main_area);
    }
    // Multiplayer game
    else if app.current_page == Pages::Multiplayer {
        if app.hosting.is_none() {
            app.current_popup = Some(Popups::MultiplayerSelection);
        } else if app.selected_color.is_none() && app.hosting.unwrap() {
            app.current_popup = Some(Popups::ColorSelection);
        } else if app.game.opponent.is_none() {
            if app.host_ip.is_none() {
                if app.hosting.is_some() && app.hosting.unwrap() {
                    app.setup_game_server(app.selected_color.unwrap());
                    app.host_ip = Some("127.0.0.1".to_string());
                } else {
                    app.current_popup = Some(Popups::EnterHostIP);
                }
            } else {
                app.create_opponent();
            }
        } else if app.game.opponent.as_mut().unwrap().game_started {
            render_game_ui(frame, app, main_area);
        }
    }
    // Play against bot
    else if app.current_page == Pages::Bot {
        if app.chess_engine_path.is_none() || app.chess_engine_path.as_ref().unwrap().is_empty() {
            render_engine_path_error_popup(frame);
        } else if app.selected_color.is_none() {
            app.current_popup = Some(Popups::ColorSelection);
        } else if app.game.bot.is_none() {
            app.bot_setup();
        } else {
            render_game_ui(frame, app, main_area);
        }
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
        Some(Popups::EnterPGNPath) => {
            render_enter_pgn_path(frame, &app.game.ui.prompt);
        }
        Some(Popups::WaitingForOpponentToJoin) => {
            render_wait_for_other_player(frame, app.get_host_ip());
        }
        Some(Popups::Help) => {
            render_help_popup(frame);
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
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(3, 5),
            ]
            .as_ref(),
        )
        .split(main_area);

    // Title
    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, main_layout_horizontal[0]);

    // Board block representing the full board div
    let text: Vec<Line<'_>> = vec![Line::from(""), Line::from("A chess game made in 🦀")];
    let sub_title = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, main_layout_horizontal[1]);

    // Determine the "display mode" text
    let display_mode_menu = {
        let display_mode = match app.game.ui.display_mode {
            DisplayMode::DEFAULT => "Default",
            DisplayMode::ASCII => "ASCII",
        };
        format!("Display mode: {display_mode}")
    };

    // Board block representing the full board div
    let menu_items = [
        "Normal game",
        "Multiplayer",
        "Play against a bot",
        &display_mode_menu,
        "Help",
        "Credits",
    ];
    let mut menu_body: Vec<Line<'_>> = vec![];

    for (i, menu_item) in menu_items.iter().enumerate() {
        menu_body.push(Line::from(""));
        let mut text = if app.menu_cursor == i as u8 {
            "> ".to_string()
        } else {
            String::new()
        };
        text.push_str(menu_item);
        menu_body.push(Line::from(text));
    }

    let sub_title = Paragraph::new(menu_body)
        .bold()
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, main_layout_horizontal[2]);
}

// Method to render the game board and handle game popups
pub fn render_game_ui(frame: &mut Frame<'_>, app: &mut App, main_area: Rect) {
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
        .split(main_layout_vertical[3]);
    // Board block representing the full board div
    let board_block = Block::default().style(Style::default());

    // We render the board_block in the center layout made above
    frame.render_widget(board_block.clone(), main_layout_vertical[1]);

    let game_clone = app.game.clone();
    app.game.ui.board_render(
        board_block.inner(main_layout_vertical[1]),
        frame,
        &game_clone,
    ); // Mutable borrow now allowed

    //top box for white material
    app.game.ui.black_material_render(
        board_block.inner(right_box_layout[0]),
        frame,
        &app.game.game_board.black_taken_pieces,
    );

    // We make the inside of the board
    app.game
        .ui
        .history_render(board_block.inner(right_box_layout[1]), frame, &app.game);

    //bottom box for black matetrial
    app.game.ui.white_material_render(
        board_block.inner(right_box_layout[2]),
        frame,
        &app.game.game_board.white_taken_pieces,
    );

    if app.game.game_state == GameState::Promotion {
        render_promotion_popup(frame, app);
    }

    if app.game.game_state == GameState::Checkmate {
        let victorious_player = app.game.player_turn.opposite();

        let string_color = match victorious_player {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
        };

        if app.current_popup == Some(Popups::EndScreen) {
            render_end_popup(
                frame,
                &format!("{string_color} Won !!!"),
                app.game.opponent.is_some(),
            );
        }
    }

    if app.game.game_state == GameState::Draw && app.current_popup == Some(Popups::EndScreen) {
        render_end_popup(frame, "That's a draw", app.game.opponent.is_some());
    }
}
