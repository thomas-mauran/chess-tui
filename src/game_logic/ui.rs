use super::{coord::Coord, game::Game};
use crate::{
    constants::{DisplayMode, BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{PieceColor, PieceMove, PieceType},
    ui::{main_ui::render_cell, prompt::Prompt},
    utils::{convert_position_into_notation, get_cell_paragraph},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

#[derive(Clone)]
pub struct UI {
    /// The cursor position
    pub cursor_coordinates: Coord,
    /// The selected cell
    pub selected_coordinates: Coord,
    /// The selected piece cursor when we already selected a piece
    pub selected_piece_cursor: i8,
    /// The cursor for The promotion popup
    pub promotion_cursor: i8,
    /// The old cursor position used when unslecting a cell
    pub old_cursor_position: Coord,
    /// coordinates of the interactable part of the screen (either normal chess board or promotion screen)
    pub top_x: u16,
    pub top_y: u16,
    /// dimension of a selectable cell (either 1 of the 64 cells, or 1 of the 4 promotion options)
    pub width: u16,
    pub height: u16,
    /// last move was with a mouse
    pub mouse_used: bool,
    /// The skin of the game
    pub display_mode: DisplayMode,
    // The prompt for the player
    pub prompt: Prompt,
}

impl Default for UI {
    fn default() -> Self {
        UI {
            cursor_coordinates: Coord::new(4, 4),
            selected_coordinates: Coord::undefined(),
            selected_piece_cursor: 0,
            promotion_cursor: 0,
            old_cursor_position: Coord::undefined(),
            top_x: 0,
            top_y: 0,
            width: 0,
            height: 0,
            mouse_used: false,
            display_mode: DisplayMode::DEFAULT,
            prompt: Prompt::new(),
        }
    }
}

impl UI {
    pub fn reset(&mut self) {
        self.cursor_coordinates = Coord::new(4, 4);
        self.selected_coordinates = Coord::undefined();
        self.selected_piece_cursor = 0;
        self.promotion_cursor = 0;
        self.old_cursor_position = Coord::undefined();
        self.top_x = 0;
        self.top_y = 0;
        self.width = 0;
        self.height = 0;
        self.mouse_used = false;
    }

    /// Check if a cell has been selected
    pub fn is_cell_selected(&self) -> bool {
        self.selected_coordinates.row != UNDEFINED_POSITION
            && self.selected_coordinates.col != UNDEFINED_POSITION
    }

    /* Method to move the selected piece cursor
    We make sure that the cursor is in the authorized positions
    */
    pub fn move_selected_piece_cursor(
        &mut self,
        first_time_moving: bool,
        direction: i8,
        mut authorized_positions: Vec<Coord>,
    ) {
        if authorized_positions.is_empty() {
            self.cursor_coordinates = Coord::undefined();
        } else {
            self.selected_piece_cursor = if self.selected_piece_cursor == 0 && first_time_moving {
                0
            } else {
                let new_cursor =
                    (self.selected_piece_cursor + direction) % authorized_positions.len() as i8;
                if new_cursor == -1 {
                    authorized_positions.len() as i8 - 1
                } else {
                    new_cursor
                }
            };

            authorized_positions.sort();

            if let Some(position) = authorized_positions.get(self.selected_piece_cursor as usize) {
                self.cursor_coordinates = *position;
            }
        }
    }

    // CURSOR MOVEMENT
    /// Move the cursor up
    pub fn cursor_up(&mut self, authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1, authorized_positions);
        } else if self.cursor_coordinates.row > 0 {
            self.cursor_coordinates.row -= 1;
        }
    }

    /// Move the cursor down
    pub fn cursor_down(&mut self, authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, 1, authorized_positions);
        } else if self.cursor_coordinates.row < 7 {
            self.cursor_coordinates.row += 1;
        }
    }

    /// Move the cursor to the left
    pub fn cursor_left(&mut self, authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1, authorized_positions);
        } else if self.cursor_coordinates.col > 0 {
            self.cursor_coordinates.col -= 1;
        }
    }

    /// Move the cursor to the left when we are showing the promotion popup
    pub fn cursor_left_promotion(&mut self) {
        self.promotion_cursor = if self.promotion_cursor > 0 {
            self.promotion_cursor - 1
        } else {
            3
        };
    }

    /// Move the cursor to the right
    pub fn cursor_right(&mut self, authorized_positions: Vec<Coord>) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, 1, authorized_positions);
        } else if self.cursor_coordinates.col < 7 {
            self.cursor_coordinates.col += 1;
        }
    }

    /// Move the cursor to the right when we are doing a promotion
    pub fn cursor_right_promotion(&mut self) {
        self.promotion_cursor = (self.promotion_cursor + 1) % 4;
    }

    /// Method to unselect a cell
    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_coordinates = Coord::undefined();
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position;
        }
    }

    /// Method to render the right panel history
    pub fn history_render(&self, area: Rect, frame: &mut Frame, game: &Game) {
        // We write the history board on the side
        let history_block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(5, 10, 1, 2));

        let mut lines: Vec<Line> = vec![];

        // Group moves by color and display them correctly
        let mut white_moves: Vec<&PieceMove> = vec![];
        let mut black_moves: Vec<&PieceMove> = vec![];

        for piece_move in &game.game_board.move_history {
            match piece_move.piece_color {
                PieceColor::White => white_moves.push(piece_move),
                PieceColor::Black => black_moves.push(piece_move),
            }
        }

        // Display moves in pairs (white, black)
        let max_moves = std::cmp::max(white_moves.len(), black_moves.len());
        for i in 0..max_moves {
            let mut move_white = "   ".to_string();
            let mut utf_icon_white = "   ";

            if i < white_moves.len() {
                let white_move = white_moves[i];
                utf_icon_white =
                    PieceType::piece_to_utf_enum(&white_move.piece_type, Some(PieceColor::White));
                move_white = convert_position_into_notation(&format!(
                    "{}{}{}{}",
                    white_move.from.row, white_move.from.col, white_move.to.row, white_move.to.col
                ));
            }

            let mut move_black = "   ".to_string();
            let mut utf_icon_black = "   ";

            if i < black_moves.len() {
                let black_move = black_moves[i];
                utf_icon_black =
                    PieceType::piece_to_utf_enum(&black_move.piece_type, Some(PieceColor::Black));

                // Transform black moves for display using perspective system
                let black_piece_move = PieceMove {
                    piece_type: black_move.piece_type,
                    piece_color: black_move.piece_color,
                    from: black_move.from,
                    to: black_move.to,
                };
                let transformed_move = game
                    .perspective
                    .transform_move_for_display(&black_piece_move);
                move_black = convert_position_into_notation(&format!(
                    "{}{}{}{}",
                    transformed_move.from.row,
                    transformed_move.from.col,
                    transformed_move.to.row,
                    transformed_move.to.col
                ));
            }

            lines.push(Line::from(vec![
                Span::raw(format!("{}.  ", i + 1)), // line number
                Span::styled(format!("{utf_icon_white} "), Style::default().fg(WHITE)), // white symbol
                Span::raw(move_white.to_string()), // white move
                Span::raw("     "),                // separator
                Span::styled(format!("{utf_icon_black} "), Style::default().fg(WHITE)), // black symbol
                Span::raw(move_black.to_string()), // black move
            ]));
        }

        let history_paragraph = Paragraph::new(lines).alignment(Alignment::Center);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        frame.render_widget(history_block.clone(), right_panel_layout[0]);
        frame.render_widget(
            history_paragraph,
            history_block.inner(right_panel_layout[0]),
        );
    }

    /// Method to render the white material
    pub fn white_material_render(
        &self,
        area: Rect,
        frame: &mut Frame,
        white_taken_pieces: &[PieceType],
    ) {
        let white_block = Block::default()
            .title("White material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for piece in white_taken_pieces {
            let utf_icon_white = PieceType::piece_to_utf_enum(piece, Some(PieceColor::Black));

            pieces.push_str(&format!("{utf_icon_white} "));
        }
        let white_material_paragraph = Paragraph::new(pieces)
            .alignment(Alignment::Center)
            .add_modifier(Modifier::BOLD);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);
        frame.render_widget(white_block.clone(), right_panel_layout[0]);
        frame.render_widget(
            white_material_paragraph,
            white_block.inner(right_panel_layout[0]),
        );
        // Bottom paragraph help text
        let text = vec![Line::from("Press ? for help").alignment(Alignment::Center)];

        let help_paragraph = Paragraph::new(text)
            .block(Block::new())
            .alignment(Alignment::Center);
        frame.render_widget(help_paragraph, right_panel_layout[1]);
    }

    /// Method to render the black material
    pub fn black_material_render(
        &self,
        area: Rect,
        frame: &mut Frame,
        black_taken_pieces: &Vec<PieceType>,
    ) {
        let black_block = Block::default()
            .title("Black material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for piece in black_taken_pieces {
            let utf_icon_black = PieceType::piece_to_utf_enum(piece, Some(PieceColor::White));

            pieces.push_str(&format!("{utf_icon_black} "));
        }

        let black_material_paragraph = Paragraph::new(pieces)
            .alignment(Alignment::Center)
            .add_modifier(Modifier::BOLD);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        frame.render_widget(black_block.clone(), right_panel_layout[0]);
        frame.render_widget(
            black_material_paragraph,
            black_block.inner(right_panel_layout[0]),
        );
    }

    /// Method to render the board
    pub fn board_render(&mut self, area: Rect, frame: &mut Frame<'_>, game: &Game) {
        let width = area.width / 8;
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);

        // we update the starting coordinates
        self.top_x = area.x + border_width;
        self.top_y = area.y + border_height;
        self.width = width;
        self.height = height;
        // We have 8 vertical lines
        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    // spread the excess border
                    Constraint::Length(border_height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(border_height),
                ]
                .as_ref(),
            )
            .split(area);

        // For each line we set 8 layout
        for display_row in 0..8u8 {
            // Transform display row to logical row based on perspective
            let _logical_row = if game.perspective.needs_transformation() {
                7 - display_row
            } else {
                display_row
            };

            let lines = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(border_width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(border_width),
                    ]
                    .as_ref(),
                )
                .split(columns[display_row as usize + 1]);
            for j in 0..8u8 {
                // Color of the cell to draw the board
                let cell_color: Color = if (display_row + j) % 2 == 0 {
                    WHITE
                } else {
                    BLACK
                };

                let last_move;
                let mut last_move_from = Coord::undefined();
                let mut last_move_to = Coord::undefined();
                if !game.game_board.move_history.is_empty() {
                    last_move = game.game_board.move_history.last();
                    // Transform the last move for display using the perspective system
                    if let Some(last_move_data) = last_move {
                        let transformed_move =
                            game.perspective.transform_move_for_display(last_move_data);
                        last_move_from = transformed_move.from;
                        last_move_to = transformed_move.to;
                    }

                    // If the opponent is the same as the last move player, we don't want to show his last move
                    if game.opponent.is_some()
                        && game.opponent.as_ref().unwrap().color == game.player_turn
                    {
                        last_move_from = Coord::undefined();
                        last_move_to = Coord::undefined();
                    }
                }

                let mut positions: Vec<Coord> = vec![];
                let is_cell_in_positions = |positions: &Vec<Coord>, i: u8, j: u8| {
                    positions.iter().any(|&coord| coord == Coord::new(i, j))
                };
                // Draw the available moves for the selected piece
                if self.is_cell_selected() {
                    let selected_piece_color: Option<PieceColor> =
                        game.game_board.get_piece_color(&self.selected_coordinates);
                    // only draw available moves if it is the right players turn
                    if match selected_piece_color {
                        Some(color) => color == game.player_turn,
                        None => false,
                    } {
                        positions = game.game_board.get_authorized_positions_with_perspective(
                            game.player_turn,
                            self.selected_coordinates,
                            Some(&game.perspective),
                        );

                        // Draw grey if the color is in the authorized positions
                        let display_positions: Vec<Coord> = positions
                            .iter()
                            .map(|c| game.perspective.logical_to_display(*c))
                            .collect();
                        for coords in display_positions.clone() {
                            if display_row == coords.row && j == coords.col {
                                // cell_color = Color::Rgb(100, 100, 100);
                            }
                        }
                    }
                }

                let square = lines[j as usize + 1];
                // Here we have all the possibilities for a cell:
                // - selected cell: green
                // - cursor cell: blue
                // - available move cell: grey
                // - checked king cell: magenta
                // - last move cell: green
                // - default cell: white or black
                // Draw the cell blue if this is the current cursor cell
                let display_cursor = game.perspective.logical_to_display(self.cursor_coordinates);
                if display_row == display_cursor.row && j == display_cursor.col && !self.mouse_used
                {
                    render_cell(frame, square, Color::LightBlue, None);
                }
                // Draw the cell magenta if this is the king in check
                else if game
                    .game_board
                    .is_getting_checked(game.game_board.board, game.player_turn)
                {
                    let king_coord_display = game.perspective.logical_to_display(
                        game.game_board
                            .get_king_coordinates(game.game_board.board, game.player_turn),
                    );
                    if Coord::new(display_row, j) == king_coord_display {
                        render_cell(frame, square, Color::Magenta, Some(Modifier::SLOW_BLINK));
                    } else {
                        // For non-king cells when king is in check, continue with normal rendering
                        let display_selected = game
                            .perspective
                            .logical_to_display(self.selected_coordinates);
                        if (display_row == display_selected.row && j == display_selected.col)
                            || (last_move_from == Coord::new(display_row, j)
                                || (last_move_to == Coord::new(display_row, j) && {
                                    let display_positions: Vec<Coord> = positions
                                        .iter()
                                        .map(|c| game.perspective.logical_to_display(*c))
                                        .collect();
                                    !is_cell_in_positions(&display_positions, display_row, j)
                                }))
                        // and not in the authorized positions (grey instead of green)
                        {
                            render_cell(frame, square, Color::LightGreen, None);
                        } else {
                            let display_positions: Vec<Coord> = positions
                                .iter()
                                .map(|c| game.perspective.logical_to_display(*c))
                                .collect();
                            if is_cell_in_positions(&display_positions, display_row, j) {
                                render_cell(frame, square, Color::Rgb(100, 100, 100), None);
                            } else {
                                let mut cell = Block::default();
                                cell = match self.display_mode {
                                    DisplayMode::DEFAULT => cell.bg(cell_color),
                                    DisplayMode::ASCII => match cell_color {
                                        WHITE => cell.bg(Color::White).fg(Color::Black),
                                        BLACK => cell.bg(Color::Black).fg(Color::White),
                                        _ => cell.bg(cell_color),
                                    },
                                };
                                frame.render_widget(cell.clone(), square);
                            }
                        }
                    }
                }
                // Draw the cell green if this is the selected cell or if the cell is part of the last move
                else {
                    let display_selected = game
                        .perspective
                        .logical_to_display(self.selected_coordinates);
                    if (display_row == display_selected.row && j == display_selected.col)
                        || (last_move_from == Coord::new(display_row, j)
                            || (last_move_to == Coord::new(display_row, j) && {
                                let display_positions: Vec<Coord> = positions
                                    .iter()
                                    .map(|c| game.perspective.logical_to_display(*c))
                                    .collect();
                                !is_cell_in_positions(&display_positions, display_row, j)
                            }))
                    // and not in the authorized positions (grey instead of green)
                    {
                        render_cell(frame, square, Color::LightGreen, None);
                    } else {
                        let display_positions: Vec<Coord> = positions
                            .iter()
                            .map(|c| game.perspective.logical_to_display(*c))
                            .collect();
                        if is_cell_in_positions(&display_positions, display_row, j) {
                            render_cell(frame, square, Color::Rgb(100, 100, 100), None);
                        } else {
                            let mut cell = Block::default();
                            cell = match self.display_mode {
                                DisplayMode::DEFAULT => cell.bg(cell_color),
                                DisplayMode::ASCII => match cell_color {
                                    WHITE => cell.bg(Color::White).fg(Color::Black),
                                    BLACK => cell.bg(Color::Black).fg(Color::White),
                                    _ => cell.bg(cell_color),
                                },
                            };
                            frame.render_widget(cell.clone(), square);
                        }
                    }
                }

                // Get piece and color
                let logical_coord = game
                    .perspective
                    .display_to_logical(Coord::new(display_row, j));
                let paragraph = get_cell_paragraph(game, &logical_coord, square);

                frame.render_widget(paragraph, square);
            }
        }
    }
}
