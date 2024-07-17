use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    app::App,
    constants::{DisplayMode, Pages, TITLE},
    pieces::PieceColor,
    popups::{
        render_credit_popup, render_end_popup, render_engine_path_error_popup, render_help_popup,
        render_promotion_popup,
    },
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let main_area = frame.size();

    if app.current_page == Pages::Solo {
        render_game_ui(frame, app, main_area);
    } else if app.current_page == Pages::Bot {
        if app.board.engine.is_none() {
            match &app.chess_engine_path {
                Some(path) => {
                    app.board.set_engine(path);
                    render_game_ui(frame, app, main_area);
                }
                None => render_engine_path_error_popup(frame),
            }
        } else {
            render_game_ui(frame, app, main_area);
        }
    } else {
        render_menu_ui(frame, app, main_area);
    }

    if app.show_help_popup {
        render_help_popup(frame);
    }

    if app.current_page == Pages::Credit {
        render_credit_popup(frame);
    }
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
        let display_mode = match app.board.display_mode {
            DisplayMode::DEFAULT => "Default",
            DisplayMode::ASCII => "ASCII",
        };
        format!("Display mode: {display_mode}")
    };

    // Board block representing the full board div
    let menu_items = [
        "Normal game",
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
pub fn render_game_ui(frame: &mut Frame, app: &App, main_area: Rect) {
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

    if app.board.is_promotion {
        render_promotion_popup(frame, app);
    }

    if app.board.is_draw {
        render_end_popup(frame, "That's a draw");
    }

    if app.board.is_checkmate {
        let victorious_player = app.board.player_turn.opposite();

        let string_color = match victorious_player {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
        };

        render_end_popup(frame, &format!("{string_color} Won !!!"));
    }
}
