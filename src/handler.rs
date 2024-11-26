use crate::game_logic::coord::Coord;
use crate::game_logic::game::GameState;
use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind != KeyEventKind::Press {
        // crossterm on Windows sends Release and Repeat events as well, which we ignore.
        return Ok(());
    }
    if app.game.ui.mouse_used {
        app.game.ui.mouse_used = false;
        if app.game.ui.selected_coordinates != Coord::undefined() {
            app.game.ui.cursor_coordinates = app.game.ui.selected_coordinates;
            app.game.ui.selected_coordinates = Coord::undefined();
        } else {
            app.game.ui.cursor_coordinates.col = 4;
            app.game.ui.cursor_coordinates.row = 4;
        }
    }

    match key_event.code {
        // Exit application on `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right | KeyCode::Char('l') => {
            // When we are in the color selection menu
            if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.menu_cursor_right(2);
            } else if app.game.game_state == GameState::Promotion {
                app.game.ui.cursor_right_promotion();
            } else if !(app.game.game_state == GameState::Checkmate)
                && !(app.game.game_state == GameState::Draw)
            {
                let authorized_positions = app.game.game_board.get_authorized_positions(
                    app.game.player_turn,
                    app.game.ui.selected_coordinates,
                );
                app.game.ui.cursor_right(authorized_positions);
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            // When we are in the color selection menu
            if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.menu_cursor_left(2);
            } else if app.game.game_state == GameState::Promotion {
                app.game.ui.cursor_left_promotion();
            } else if !(app.game.game_state == GameState::Checkmate)
                && !(app.game.game_state == GameState::Draw)
            {
                let authorized_positions = app.game.game_board.get_authorized_positions(
                    app.game.player_turn,
                    app.game.ui.selected_coordinates,
                );

                app.game.ui.cursor_left(authorized_positions);
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_up(Pages::variant_count() as u8);
            } else if !(app.game.game_state == GameState::Checkmate)
                && !(app.game.game_state == GameState::Draw)
                && !(app.game.game_state == GameState::Promotion)
            {
                let authorized_positions = app.game.game_board.get_authorized_positions(
                    app.game.player_turn,
                    app.game.ui.selected_coordinates,
                );
                app.game.ui.cursor_up(authorized_positions);
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_down(Pages::variant_count() as u8);
            } else if !(app.game.game_state == GameState::Checkmate)
                && !(app.game.game_state == GameState::Draw)
                && !(app.game.game_state == GameState::Promotion)
            {
                let authorized_positions = app.game.game_board.get_authorized_positions(
                    app.game.player_turn,
                    app.game.ui.selected_coordinates,
                );

                app.game.ui.cursor_down(authorized_positions);
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.color_selection();
            } else if app.current_page == Pages::Home {
                app.menu_select();
            } else {
                app.game.select_cell();
            }
        }
        KeyCode::Char('?') => {
            if app.current_page != Pages::Credit {
                app.toggle_help_popup();
            }
        }
        KeyCode::Char('r') => app.restart(),
        KeyCode::Esc => {
            if app.show_help_popup {
                app.show_help_popup = false;
            } else if app.show_color_popup {
                app.show_color_popup = false;
                app.current_page = Pages::Home;
            } else if app.current_page == Pages::Credit {
                app.current_page = Pages::Home;
            } else if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.current_page = Pages::Home;
                app.show_color_popup = false;
                app.menu_cursor = 0;
            }
            app.game.ui.unselect_cell();
        }
        KeyCode::Char('b') => {
            let display_mode = app.game.ui.display_mode;
            app.selected_color = None;
            if app.game.bot.is_some() {
                app.game.bot = None;
            }
            app.go_to_home();
            app.game.game_board.reset();
            app.game.ui.reset();
            app.game.ui.display_mode = display_mode;
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    // Mouse control only implemented for actual game
    if app.current_page == Pages::Home {
        return Ok(());
    }
    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
        if app.game.game_state == GameState::Checkmate || app.game.game_state == GameState::Draw {
            return Ok(());
        }

        if app.show_color_popup || app.show_help_popup {
            return Ok(());
        }

        // If there is a promotion to be done the top_x, top_y, width and height
        // values are updated accordingly
        if app.game.game_state == GameState::Promotion {
            let x = (mouse_event.column - app.game.ui.top_x) / app.game.ui.width;
            let y = (mouse_event.row - app.game.ui.top_y) / app.game.ui.height;
            if x > 3 || y > 0 {
                return Ok(());
            }
            app.game.ui.promotion_cursor = x as i8;
            app.game.promote_piece();
        }
        if mouse_event.column < app.game.ui.top_x || mouse_event.row < app.game.ui.top_y {
            return Ok(());
        }
        let x = (mouse_event.column - app.game.ui.top_x) / app.game.ui.width;
        let y = (mouse_event.row - app.game.ui.top_y) / app.game.ui.height;
        if x > 7 || y > 7 {
            return Ok(());
        }
        app.game.ui.mouse_used = true;
        let coords: Coord = Coord::new(y as u8, x as u8);

        let authorized_positions = app
            .game
            .game_board
            .get_authorized_positions(app.game.player_turn, app.game.ui.selected_coordinates);

        let piece_color = app
            .game
            .game_board
            .get_piece_color(&app.game.ui.selected_coordinates);

        if authorized_positions.contains(&coords)
            && match piece_color {
                Some(piece) => Some(piece) == piece_color,
                None => false,
            }
        {
            app.game.ui.cursor_coordinates = coords;
            app.game.select_cell();
        } else {
            app.game.ui.selected_coordinates = coords;
        }
    }
    Ok(())
}
