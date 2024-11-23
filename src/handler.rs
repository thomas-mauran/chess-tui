use crate::{
    app::{App, AppResult}, board::Coord, constants::Pages
};
use ratatui::
    crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind
};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind != KeyEventKind::Press {
        // crossterm on Windows sends Release and Repeat events as well, which we ignore.
        return Ok(());
    }
    if app.board.mouse_used == true{
        app.board.mouse_used = false;
        if app.board.selected_coordinates != Coord::undefined() {
            app.board.cursor_coordinates = app.board.selected_coordinates;
            app.board.selected_coordinates = Coord::undefined();
        }
        else {
            app.board.cursor_coordinates.col = 4;
            app.board.cursor_coordinates.row = 4;
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
            } else {
                app.board.cursor_right();
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            // When we are in the color selection menu
            if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.menu_cursor_left(2);
            } else {
                app.board.cursor_left();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_up(Pages::variant_count() as u8);
            } else {
                app.board.cursor_up();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_down(Pages::variant_count() as u8);
            } else {
                app.board.cursor_down();
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.color_selection();
            } else if app.current_page == Pages::Home {
                app.menu_select();
            } else {
                app.board.select_cell();
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
            } else if app.current_page == Pages::Credit {
                app.current_page = Pages::Home;
            } else if app.current_page == Pages::Bot && app.selected_color.is_none() {
                app.current_page = Pages::Home;
                app.show_color_popup = false;
                app.menu_cursor = 0;
            }
            app.board.unselect_cell();
        }
        KeyCode::Char('b') => {
            app.go_to_home();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}



pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()>{

    // Mouse control only implemented for actual game
    if app.current_page == Pages::Home {
        return Ok(());
    }
    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
        if app.board.is_checkmate || app.board.is_draw {
            return Ok(());
        }

        // If there is a promotion to be done the top_x, top_y, width and height
        // values are updated accordingly
        if app.board.is_promotion {
            let x = (mouse_event.column - app.board.top_x) / app.board.width;
            let y = (mouse_event.row - app.board.top_y) / app.board.height;
            if x > 3 || y > 0 {
                return Ok(());
            }
            app.board.promotion_cursor = x as i8;
            app.board.promote_piece();
        }
        let x = (mouse_event.column - app.board.top_x) / app.board.width;
        let y = (mouse_event.row - app.board.top_y) / app.board.height;
        if x > 7 || y > 7 {
            return Ok(());
        }
        app.board.mouse_used = true;
        let coords: Coord = Coord::new(y as u8, x as u8);
        app.board.move_selected_piece_cursor_mouse(coords);
    }
    Ok(())
}
