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

    if app.current_popup == Some(Popups::EnterHostIP) {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Enter => {
                        app.game.ui.prompt.submit_message();
                        if app.current_page == Pages::Multiplayer {
                            app.host_ip = Some(app.game.ui.prompt.message.clone());
                        }
                        app.current_popup = None;
                    },
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
                _ => {}
            }
        }
    } else {
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
                if app.current_page == Pages::Multiplayer && app.hosting.is_none() {
                    app.menu_cursor_right(2);
                } else if app.current_page == Pages::Multiplayer && app.selected_color.is_none() {
                    app.menu_cursor_right(2);
                } else if app.current_page == Pages::Bot && app.selected_color.is_none() {
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
                if app.current_page == Pages::Multiplayer && app.hosting.is_none() {
                    app.menu_cursor_left(2);
                } else if app.current_page == Pages::Multiplayer && app.selected_color.is_none() {
                    app.menu_cursor_left(2);
                } else if app.current_page == Pages::Bot && app.selected_color.is_none() {
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
            KeyCode::Char(' ') | KeyCode::Enter => match app.current_page {
                Pages::Home => {
                    app.menu_select();
                }
                Pages::Bot => {
                    if app.selected_color.is_none() {
                        app.color_selection();
                        app.bot_setup();
                    } else {
                        app.game.select_cell();
                    }
                }
                Pages::Multiplayer => {
                    if app.hosting.is_none() {
                        app.hosting_selection();
                    } else if app.selected_color.is_none() {
                        if app.hosting.is_some() && app.hosting.unwrap() == true {
                            app.color_selection();
                        }
                    } else {
                        app.game.select_cell();
                    }
                }
                Pages::Credit => {
                    app.current_page = Pages::Home;
                }
                _ => {
                    app.game.select_cell();
                }
            },
            KeyCode::Char('?') => {
                if app.current_page != Pages::Credit {
                    app.toggle_help_popup();
                }
            }
            KeyCode::Char('r') => {
                // We can't restart the game if it's a multiplayer one
                if app.game.player.is_none() {
                    app.restart();
                }
            }
            KeyCode::Esc => {
                match app.current_popup {
                    Some(Popups::ColorSelection) => {
                        app.current_popup = None;
                        app.selected_color = None;
                        app.hosting = None;
                        app.current_page = Pages::Home;
                        app.menu_cursor = 0;
                    }
                    Some(Popups::MultiplayerSelection) => {
                        app.current_popup = None;
                        app.selected_color = None;
                        app.hosting = None;
                        app.current_page = Pages::Home;
                        app.menu_cursor = 0;
                    }
                    Some(Popups::Help) => {
                        app.current_popup = None;
                    }
                    _ => {}
                }

                match app.current_page {
                    Pages::Bot => {
                        app.current_page = Pages::Home;
                        app.menu_cursor = 0;
                        app.selected_color = None;
                    }
                    Pages::Credit => {
                        app.current_page = Pages::Home;
                    }
                    _ => {}
                }

                app.game.ui.unselect_cell();
            }
            KeyCode::Char('b') => {
                let display_mode = app.game.ui.display_mode;
                app.selected_color = None;
                if app.game.bot.is_some() {
                    app.game.bot = None;
                }
                if app.game.player.is_some() {
                    app.game.player.as_mut().unwrap().send_end_game_to_server();
                    app.game.player = None;
                    app.hosting = None;
                }

                app.go_to_home();
                app.game.game_board.reset();
                app.game.ui.reset();
                app.game.ui.display_mode = display_mode;
            }
            // Other handlers you could add here.
            _ => {}
        }
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
            app.game.handle_multiplayer_promotion();
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
