use crate::constants::Popups;
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

    // If a popup is active, the key should affect the popup and not the page,
    // therefore, if there is some popup active, handle it, if not, handle the page event
    match app.current_popup {
        Some(popup) => handle_popup_input(app, key_event, popup),
        None => handle_page_input(app, key_event),
    }

    Ok(())
}

fn handle_popup_input(app: &mut App, key_event: KeyEvent, popup: Popups) {
    match popup {
        Popups::EnterHostIP => match key_event.code {
            KeyCode::Enter => {
                app.game.ui.prompt.submit_message();
                assert_eq!(app.current_page, Pages::Multiplayer);
                if app.current_page == Pages::Multiplayer {
                    app.host_ip = Some(app.game.ui.prompt.message.clone());
                }
                app.current_popup = None;
            }
            KeyCode::Char(to_insert) => app.game.ui.prompt.enter_char(to_insert),
            KeyCode::Backspace => app.game.ui.prompt.delete_char(),
            KeyCode::Left => app.game.ui.prompt.move_cursor_left(),
            KeyCode::Right => app.game.ui.prompt.move_cursor_right(),
            KeyCode::Esc => {
                app.current_popup = None;
                if app.current_page == Pages::Multiplayer {
                    app.hosting = None;
                    app.selected_color = None;
                    app.menu_cursor = 0;
                }
                app.current_page = Pages::Home;
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::Help => match key_event.code {
            KeyCode::Char('?') => app.toggle_help_popup(),
            KeyCode::Esc => app.toggle_help_popup(),
            _ => fallback_key_handler(app, key_event),
        },
        Popups::ColorSelection => match key_event.code {
            KeyCode::Esc => {
                app.current_popup = None;
                app.current_page = Pages::Home;
            }
            KeyCode::Right | KeyCode::Char('l') => app.menu_cursor_right(2),
            KeyCode::Left | KeyCode::Char('h') => app.menu_cursor_left(2),
            KeyCode::Char(' ') | KeyCode::Enter => app.color_selection(),
            _ => fallback_key_handler(app, key_event),
        },
        Popups::MultiplayerSelection => match key_event.code {
            KeyCode::Esc => {
                app.current_popup = None;
                app.current_page = Pages::Home;
            }
            KeyCode::Right | KeyCode::Char('l') => app.menu_cursor_right(2),
            KeyCode::Left | KeyCode::Char('h') => app.menu_cursor_left(2),
            KeyCode::Char(' ') | KeyCode::Enter => app.hosting_selection(),
            _ => fallback_key_handler(app, key_event),
        },
        Popups::EnginePathError => match key_event.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                app.current_popup = None;
                app.current_page = Pages::Home;
            }
            _ => fallback_key_handler(app, key_event),
        },
        Popups::WaitingForOpponentToJoin => match key_event.code {
            KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                app.current_popup = None;
                app.current_page = Pages::Home;
            }
            _ => fallback_key_handler(app, key_event),
        },
    };
}

fn handle_page_input(app: &mut App, key_event: KeyEvent) {
    match &app.current_page {
        Pages::Home => handle_home_page_events(app, key_event),
        Pages::Solo => handle_solo_page_events(app, key_event),
        Pages::Multiplayer => handle_multiplayer_page_events(app, key_event),
        Pages::Bot => handle_bot_page_events(app, key_event),
        Pages::Credit => handle_credit_page_events(app, key_event),
    }
}

fn handle_home_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') => app.menu_cursor_up(Pages::variant_count() as u8),
        KeyCode::Down | KeyCode::Char('j') => app.menu_cursor_down(Pages::variant_count() as u8),
        KeyCode::Char(' ') | KeyCode::Enter => app.menu_select(),
        KeyCode::Char('?') => app.toggle_help_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_solo_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('r') => app.restart(),
        KeyCode::Char('b') => {
            let display_mode = app.game.ui.display_mode;
            app.selected_color = None;
            app.game.bot = None;
            app.go_to_home();
            app.game.game_board.reset();
            app.game.ui.reset();
            app.game.ui.display_mode = display_mode;
        }
        _ => chess_inputs(app, key_event),
    }
}

fn chess_inputs(app: &mut App, key_event: KeyEvent) {
    let is_playing = app.game.game_state == GameState::Playing;

    match key_event.code {
        KeyCode::Up | KeyCode::Char('k') if is_playing => app.go_up_in_game(),
        KeyCode::Down | KeyCode::Char('j') if is_playing => app.go_down_in_game(),

        KeyCode::Right | KeyCode::Char('l') => match app.game.game_state {
            GameState::Promotion => app.game.ui.cursor_right_promotion(),
            GameState::Playing => app.go_right_in_game(),
            _ => (),
        },
        KeyCode::Left | KeyCode::Char('h') => match app.game.game_state {
            GameState::Promotion => app.game.ui.cursor_left_promotion(),
            GameState::Playing => app.go_left_in_game(),
            _ => (),
        },
        KeyCode::Char(' ') | KeyCode::Enter => app.game.handle_cell_click(),
        KeyCode::Char('?') => app.toggle_help_popup(),
        KeyCode::Esc => app.game.ui.unselect_cell(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn handle_multiplayer_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('b') => {
            let display_mode = app.game.ui.display_mode;
            app.selected_color = None;
            app.game.bot = None;

            if let Some(opponent) = app.game.opponent.as_mut() {
                opponent.send_end_game_to_server();
                app.game.opponent = None;
                app.hosting = None;
                app.host_ip = None;
            }

            app.go_to_home();
            app.game.game_board.reset();
            app.game.ui.reset();
            app.game.ui.display_mode = display_mode;
        }

        _ => chess_inputs(app, key_event),
        // Continue from here to add more commands for Multiplayer
    }
}

fn handle_bot_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('r') => app.restart(),
        KeyCode::Char('b') => {
            let display_mode = app.game.ui.display_mode;
            app.selected_color = None;
            app.game.bot = None;

            if let Some(opponent) = app.game.opponent.as_mut() {
                opponent.send_end_game_to_server();
                app.game.opponent = None;
                app.hosting = None;
                app.host_ip = None;
            }

            app.go_to_home();
            app.game.game_board.reset();
            app.game.ui.reset();
            app.game.ui.display_mode = display_mode;
        }
        _ => chess_inputs(app, key_event),
    }
}

fn handle_credit_page_events(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char(' ') | KeyCode::Esc | KeyCode::Enter => app.toggle_credit_popup(),
        _ => fallback_key_handler(app, key_event),
    }
}

fn fallback_key_handler(app: &mut App, key_event: KeyEvent) {
    // Exit application on `q` or `Cntrl + C`
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => app.quit(),
        _ => (),
    }
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    // Mouse control only implemented for actual game
    if app.current_page == Pages::Home || app.current_page == Pages::Credit {
        return Ok(());
    }
    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
        if app.game.game_state == GameState::Checkmate || app.game.game_state == GameState::Draw {
            return Ok(());
        }

        if app.current_popup.is_some() {
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
            if app.game.opponent.is_some() {
                app.game.handle_multiplayer_promotion();
            }
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
        let display_coords: Coord = Coord::new(y as u8, x as u8);
        // Map screen/display coordinates to logical board coordinates using perspective
        let coords: Coord = app.game.perspective.display_to_logical(display_coords);

        let authorized_positions = app
            .game
            .game_board
            .get_authorized_positions_with_perspective(
                app.game.player_turn, 
                app.game.ui.selected_coordinates,
                Some(&app.game.perspective)
            );

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
            app.game.handle_cell_click();
        } else {
            app.game.ui.selected_coordinates = coords;
        }
    }
    Ok(())
}
