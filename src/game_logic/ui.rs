use super::{
    coord::Coord,
    game::{Game, GameLogic},
};
use crate::{
    constants::{DisplayMode, BLACK, WHITE},
    pieces::{role_to_utf_enum, PieceSize},
    skin::Skin,
    ui::{main_ui::render_cell, prompt::Prompt},
    utils::{flip_square_if_needed, get_coord_from_square, get_square_from_coord},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};
use shakmaty::{Position, Role, Square};

#[derive(Clone)]
pub struct UI {
    /// The cursor position
    pub cursor_coordinates: Coord,
    /// The selected square
    pub selected_square: Option<Square>,
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
    // The skin of the game
    pub skin: Skin,
    /// Internal flag used to implement a manual blink/flicker for the cursor cell
    pub cursor_blink_visible: bool,
    /// Counter to control how often the cursor blink toggles (in ticks)
    pub cursor_blink_counter: u8,
}

impl Default for UI {
    fn default() -> Self {
        UI {
            cursor_coordinates: Coord::new(4, 4),
            selected_square: None,
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
            skin: Skin::default(),
            cursor_blink_visible: true,
            cursor_blink_counter: 0,
        }
    }
}

impl UI {
    pub fn reset(&mut self) {
        self.cursor_coordinates = Coord::new(4, 4);
        self.selected_square = None;
        self.selected_piece_cursor = 0;
        self.promotion_cursor = 0;
        self.old_cursor_position = Coord::undefined();
        self.top_x = 0;
        self.top_y = 0;
        self.width = 0;
        self.height = 0;
        self.mouse_used = false;
        self.cursor_blink_visible = true;
        self.cursor_blink_counter = 0;
    }

    /// Update the cursor blink state. This is called from the global tick handler.
    /// When a piece is selected, the cursor cell will toggle visibility every few ticks.
    pub fn update_cursor_blink(&mut self) {
        if self.is_cell_selected() {
            // Number of ticks between visibility toggles (higher = slower blink)
            const BLINK_INTERVAL_TICKS: u8 = 2;

            self.cursor_blink_counter = self.cursor_blink_counter.wrapping_add(1);
            if self.cursor_blink_counter >= BLINK_INTERVAL_TICKS {
                self.cursor_blink_visible = !self.cursor_blink_visible;
                self.cursor_blink_counter = 0;
            }
        } else {
            // Ensure cursor is always visible when nothing is selected
            self.cursor_blink_visible = true;
            self.cursor_blink_counter = 0;
        }
    }

    /// Check if a cell has been selected
    pub fn is_cell_selected(&self) -> bool {
        self.selected_square.is_some()
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
            self.selected_square = None;
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position;
        }
    }

    /// Helper method to render a piece paragraph
    fn render_piece_paragraph(
        &self,
        piece_type: Option<Role>,
        piece_color: Option<shakmaty::Color>,
        square: Rect,
    ) -> Paragraph<'static> {
        use crate::{
            pieces::{
                bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
            },
            utils::color_to_ratatui_enum,
        };

        // Determine piece size based on available cell dimensions
        let piece_size = PieceSize::from_dimensions(square.height);

        match self.display_mode {
            DisplayMode::DEFAULT => {
                let color_enum = color_to_ratatui_enum(piece_color);

                let piece_str = match piece_type {
                    Some(Role::King) => {
                        King::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Queen) => {
                        Queen::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Rook) => {
                        Rook::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Bishop) => {
                        Bishop::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Knight) => {
                        Knight::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Pawn) => {
                        Pawn::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    None => " ".to_string(),
                };

                Paragraph::new(piece_str)
                    .fg(color_enum)
                    .alignment(Alignment::Center)
            }
            DisplayMode::CUSTOM => {
                let color_enum = match piece_color {
                    Some(shakmaty::Color::White) => self.skin.piece_white_color,
                    Some(shakmaty::Color::Black) => self.skin.piece_black_color,
                    None => Color::Red,
                };

                let piece_str = match piece_type {
                    Some(Role::King) => {
                        King::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Queen) => {
                        Queen::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Rook) => {
                        Rook::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Bishop) => {
                        Bishop::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Knight) => {
                        Knight::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Pawn) => {
                        Pawn::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    None => " ".to_string(),
                };

                Paragraph::new(piece_str)
                    .fg(color_enum)
                    .alignment(Alignment::Center)
            }
            DisplayMode::ASCII => {
                let piece_str = match piece_type {
                    Some(Role::King) => {
                        King::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Queen) => {
                        Queen::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Rook) => {
                        Rook::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Bishop) => {
                        Bishop::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Knight) => {
                        Knight::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    Some(Role::Pawn) => {
                        Pawn::to_string(&self.display_mode, piece_size, piece_color)
                    }
                    None => " ".to_string(),
                };

                let final_piece_str = match piece_color {
                    Some(shakmaty::Color::Black) => piece_str.to_lowercase(),
                    Some(shakmaty::Color::White) => piece_str.to_uppercase(),
                    None => piece_str,
                };

                // Use bright yellow for ASCII pieces to ensure visibility on both black and white squares
                Paragraph::new(final_piece_str)
                    .fg(Color::Yellow)
                    .alignment(Alignment::Center)
            }
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
            .padding(Padding::new(1, 2, 1, 2));

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        // Calculate available width in the inner area (after borders and padding)
        let inner_area = history_block.inner(right_panel_layout[0]);
        let available_width = inner_area.width as usize;

        // Calculate widths needed for different formats
        // Big mode: line number (5 chars: "  1. ") + white move (8) + black move (8) = 21 chars
        // Medium mode: white move (8) + black move (8) = 16 chars (no numbers)
        // Small mode: one move per line, 8 chars per move (no numbers)
        const BIG_MODE_WIDTH: usize = 21; // 5 + 8 + 8 (with numbers, 2 moves on one line)
        const MEDIUM_MODE_WIDTH: usize = 16; // 8 + 8 (no numbers, 2 moves on one line)

        let use_big_mode = available_width >= BIG_MODE_WIDTH;
        let use_medium_mode = !use_big_mode && available_width >= MEDIUM_MODE_WIDTH;

        let mut lines: Vec<Line> = vec![];

        for i in (0..game.logic.game_board.move_history.len()).step_by(2) {
            let role_from = game.logic.game_board.move_history[i].role();

            let utf_icon_white = role_to_utf_enum(&role_from, Some(shakmaty::Color::White));
            let move_white = game.logic.game_board.move_to_san(i);

            let mut utf_icon_black = "   ";
            let mut move_black: String = "   ".to_string();

            // If there is something for black
            if i + 1 < game.logic.game_board.move_history.len() {
                let role_to = game.logic.game_board.move_history[i + 1].role();

                move_black = game.logic.game_board.move_to_san(i + 1);
                utf_icon_black = role_to_utf_enum(&role_to, Some(shakmaty::Color::Black));
            }

            if use_big_mode {
                // Format white move: icon + space + move notation (fixed total width of 8 chars)
                let white_move_formatted =
                    format!("{:<8}", format!("{utf_icon_white} {}", move_white));
                // Format black move: icon + space + move notation (fixed total width of 8 chars)
                let black_move_formatted =
                    format!("{:<8}", format!("{utf_icon_black} {}", move_black));

                // Big mode: "  1. ♙ e4    ♟ d5   " (centered, with numbers, 2 moves on one line)
                lines.push(Line::from(vec![
                    Span::raw(format!("{:>3}. ", i / 2 + 1)), // line number (right-aligned, 3 chars + ". ")
                    Span::styled(white_move_formatted, Style::default().fg(WHITE)), // white icon + move (fixed width)
                    Span::styled(black_move_formatted, Style::default().fg(WHITE)), // black icon + move (fixed width)
                ]));
            } else if use_medium_mode {
                // Format white move: icon + space + move notation (fixed total width of 8 chars)
                let white_move_formatted =
                    format!("{:<8}", format!("{utf_icon_white} {}", move_white));
                // Format black move: icon + space + move notation (fixed total width of 8 chars)
                let black_move_formatted =
                    format!("{:<8}", format!("{utf_icon_black} {}", move_black));

                // Medium mode: "♙ e4    ♟ d5   " (centered, no numbers, 2 moves on one line)
                lines.push(Line::from(vec![
                    Span::styled(white_move_formatted, Style::default().fg(WHITE)), // white icon + move (fixed width)
                    Span::styled(black_move_formatted, Style::default().fg(WHITE)), // black icon + move (fixed width)
                ]));
            } else {
                // Small mode: one move per line, no numbers
                // "♙ e4"
                // "♟ d5"
                let white_move_text = format!("{utf_icon_white} {}", move_white);
                lines.push(Line::from(vec![
                    Span::styled(white_move_text, Style::default().fg(WHITE)), // white move
                ]));
                if !move_black.trim().is_empty() {
                    let black_move_text = format!("{utf_icon_black} {}", move_black);
                    lines.push(Line::from(vec![
                        Span::styled(black_move_text, Style::default().fg(WHITE)), // black move
                    ]));
                }
            }
        }

        // Determine alignment: center for all modes
        let alignment = Alignment::Center;

        let history_paragraph = Paragraph::new(lines).alignment(alignment);

        frame.render_widget(history_block.clone(), right_panel_layout[0]);
        frame.render_widget(history_paragraph, inner_area);
    }

    /// Method to render the white material
    pub fn white_material_render(
        &self,
        area: Rect,
        frame: &mut Frame,
        white_taken_pieces: &[Role],
    ) {
        let white_block = Block::default()
            .title("White material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for piece in white_taken_pieces {
            let utf_icon_white = role_to_utf_enum(piece, Some(shakmaty::Color::Black));

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

    /// Method to render the white material with puzzle hint support
    pub fn white_material_render_with_puzzle_hint(
        &self,
        area: Rect,
        frame: &mut Frame,
        white_taken_pieces: &[Role],
        is_puzzle_mode: bool,
    ) {
        let white_block = Block::default()
            .title("White material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for piece in white_taken_pieces {
            let utf_icon_white = role_to_utf_enum(piece, Some(shakmaty::Color::Black));

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
        // Bottom paragraph help text - show puzzle hint if in puzzle mode
        let text = if is_puzzle_mode {
            vec![Line::from("Press T for hint | ? for help").alignment(Alignment::Center)]
        } else {
            vec![Line::from("Press ? for help").alignment(Alignment::Center)]
        };

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
        black_taken_pieces: &[Role],
    ) {
        let black_block = Block::default()
            .title("Black material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for piece in black_taken_pieces {
            let utf_icon_black = role_to_utf_enum(piece, Some(shakmaty::Color::White));

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
    fn get_last_move_squares(&self, logic: &GameLogic) -> (Option<Square>, Option<Square>) {
        if logic.game_board.move_history.is_empty() {
            return (None, None);
        }

        let last_move = logic.game_board.move_history.last();
        let last_move_from = last_move.map(|m| m.from()).unwrap();
        let last_move_to = last_move.map(|m| m.to());

        // Check if this is multiplayer mode first (TCP or Lichess)
        // In multiplayer modes, always show the last move regardless of who made it
        if let Some(opponent) = logic.opponent.as_ref() {
            // Check if this is multiplayer (TCP or Lichess)
            let is_multiplayer = opponent.is_tcp_multiplayer() || opponent.is_lichess();

            if is_multiplayer {
                // In multiplayer mode (TCP or Lichess), always show the last move
                return (last_move_from, last_move_to);
            }
        }

        // For bot mode or when position is available, determine who made the last move
        // After a move, the turn switches, so the last move was made by the opposite color
        if let Some(position) = logic.game_board.current_position() {
            let last_move_color = match position.turn() {
                shakmaty::Color::White => shakmaty::Color::Black,
                shakmaty::Color::Black => shakmaty::Color::White,
            };

            // For bot mode: only show opponent's moves
            if let Some(opponent) = logic.opponent.as_ref() {
                // If the last move was made by the opponent, show it
                if last_move_color == opponent.color {
                    return (last_move_from, last_move_to);
                } else {
                    // Last move was made by the player, don't show it
                    return (None, None);
                }
            }
        }

        // Fallback: show the move if no opponent (solo mode)
        (last_move_from, last_move_to)
    }

    fn get_authorized_positions_for_render(
        &self,
        logic: &GameLogic,
        actual_square: Option<Square>,
    ) -> Vec<Coord> {
        if !self.is_cell_selected() || actual_square.is_none() {
            return vec![];
        }

        let selected_piece_color = logic
            .game_board
            .get_piece_color_at_square(&actual_square.unwrap());

        if let Some(color) = selected_piece_color {
            if color == logic.player_turn {
                let mut authorized_positions: Vec<Coord> = logic
                    .game_board
                    .get_authorized_positions(logic.player_turn, &actual_square.unwrap())
                    .iter()
                    .map(|s| Coord::from_square(*s))
                    .collect();

                if logic.game_board.is_flipped {
                    authorized_positions =
                        authorized_positions.iter().map(|s| s.reverse()).collect();
                }
                return authorized_positions;
            }
        }
        vec![]
    }
    pub fn board_render(&mut self, area: Rect, frame: &mut Frame<'_>, logic: &GameLogic) {
        let mut board = logic
            .game_board
            .position_history
            .last()
            .unwrap()
            .board()
            .clone();

        // if the board is flipped, we need to flip the board
        if logic.game_board.is_flipped {
            board.flip_vertical();
            board.flip_horizontal();
        }

        let mut actual_square = self.selected_square;
        if self.selected_square.is_some() {
            actual_square = Some(flip_square_if_needed(
                self.selected_square.unwrap(),
                logic.game_board.is_flipped,
            ));
        }

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
        for i in 0..8u8 {
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
                .split(columns[i as usize + 1]);
            for j in 0..8u8 {
                // Color of the cell to draw the board
                let cell_color: Color = if (i + j) % 2 == 0 {
                    match self.display_mode {
                        DisplayMode::CUSTOM => self.skin.board_white_color,
                        _ => WHITE,
                    }
                } else {
                    match self.display_mode {
                        DisplayMode::CUSTOM => self.skin.board_black_color,
                        _ => BLACK,
                    }
                };

                let (last_move_from, last_move_to) = self.get_last_move_squares(logic);

                let authorized_positions =
                    self.get_authorized_positions_for_render(logic, actual_square);

                let is_cell_in_positions = |positions: &Vec<Coord>, i: u8, j: u8| {
                    positions.iter().any(|&coord| coord == Coord::new(i, j))
                };

                let square = lines[j as usize + 1];
                // Here we have all the possibilities for a cell:
                // - selected cell: green
                // - cursor cell: blue
                // - available move cell: grey
                // - checked king cell: magenta
                // - last move cell: green
                // - default cell: white or black
                // Draw the cell blue if this is the current cursor cell
                if i == self.cursor_coordinates.row
                    && j == self.cursor_coordinates.col
                    && !self.mouse_used
                    // When a piece is selected, only draw the cursor cell on "visible" ticks
                    // so that it appears to flicker. On "hidden" ticks, we let the other
                    // branches below (selected cell, available moves, etc.) handle the cell.
                    && (!self.is_cell_selected() || self.cursor_blink_visible)
                {
                    let cursor_color = match self.display_mode {
                        DisplayMode::CUSTOM => self.skin.cursor_color,
                        _ => Color::LightBlue,
                    };
                    render_cell(frame, square, cursor_color, None);
                }
                // Draw the cell magenta if the king is getting checked
                else if logic.game_board.is_getting_checked(logic.player_turn)
                    && Coord::new(i, j) == logic.game_board.get_king_coordinates(logic.player_turn)
                {
                    render_cell(frame, square, Color::Magenta, Some(Modifier::SLOW_BLINK));
                }
                // Draw the cell green if this is the selected cell or if the cell is part of the last move
                else if (i
                    == get_coord_from_square(actual_square, logic.game_board.is_flipped).row
                    && j == get_coord_from_square(actual_square, logic.game_board.is_flipped).col)
                    || (last_move_from
                        == get_square_from_coord(Coord::new(i, j), logic.game_board.is_flipped))
                    || (last_move_to
                        == get_square_from_coord(Coord::new(i, j), logic.game_board.is_flipped))
                        && !is_cell_in_positions(&authorized_positions, i, j)
                // and not in the authorized positions (grey instead of green)
                {
                    let highlight_color = match self.display_mode {
                        DisplayMode::CUSTOM => {
                            // Use selection color for selected square, last move color for last move
                            if i == get_coord_from_square(
                                actual_square,
                                logic.game_board.is_flipped,
                            )
                            .row && j
                                == get_coord_from_square(actual_square, logic.game_board.is_flipped)
                                    .col
                            {
                                self.skin.selection_color
                            } else {
                                self.skin.last_move_color
                            }
                        }
                        _ => Color::LightGreen,
                    };
                    render_cell(frame, square, highlight_color, None);
                } else if is_cell_in_positions(&authorized_positions, i, j) {
                    render_cell(frame, square, Color::Rgb(100, 100, 100), None);
                }
                // else as a last resort we draw the cell with the default color either white or black
                else {
                    let mut cell = Block::default();
                    cell = match self.display_mode {
                        DisplayMode::DEFAULT | DisplayMode::CUSTOM => cell.bg(cell_color),
                        DisplayMode::ASCII => match cell_color {
                            WHITE => cell.bg(Color::White).fg(Color::Black),
                            BLACK => cell.bg(Color::Black).fg(Color::White),
                            _ => cell.bg(cell_color),
                        },
                    };
                    frame.render_widget(cell.clone(), square);
                }

                // Get piece and color
                let coord = Coord::new(i, j);
                let square_index =
                    get_square_from_coord(coord, logic.game_board.is_flipped).unwrap();
                let piece_color = logic.game_board.get_piece_color_at_square(&square_index);
                let piece_type = logic.game_board.get_role_at_square(&square_index);

                let paragraph = self.render_piece_paragraph(piece_type, piece_color, square);
                frame.render_widget(paragraph, square);
            }
        }
    }

    /// Render rank labels (1-8) on the left side of the board
    pub fn render_rank_labels(&self, frame: &mut Frame, area: Rect, is_flipped: bool) {
        let ranks = if is_flipped {
            vec!["1", "2", "3", "4", "5", "6", "7", "8"]
        } else {
            vec!["8", "7", "6", "5", "4", "3", "2", "1"]
        };

        // Calculate the same border as the board uses
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(border_height), // Top border
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(height),
                Constraint::Length(border_height), // Bottom border
            ])
            .split(area);

        // Render labels starting from index 1 (after top border)
        for (i, rank) in ranks.iter().enumerate() {
            let rank_area = layout[i + 1];
            // Calculate vertical padding to center the text
            // Add empty lines above and below to center the number vertically
            let top_padding = if rank_area.height > 1 {
                (rank_area.height - 1) / 2
            } else {
                0
            };
            let bottom_padding = rank_area.height.saturating_sub(1 + top_padding);

            // Create text with empty lines for vertical centering
            let mut lines: Vec<Line> = Vec::new();
            for _ in 0..top_padding {
                lines.push(Line::from(""));
            }
            lines.push(
                Line::from(*rank)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Gray)),
            );
            for _ in 0..bottom_padding {
                lines.push(Line::from(""));
            }

            let label = Paragraph::new(lines);
            frame.render_widget(label, rank_area);
        }
    }

    /// Render file labels (A-H) below the board
    pub fn render_file_labels(&self, frame: &mut Frame, area: Rect, is_flipped: bool) {
        let files = if is_flipped {
            vec!["H", "G", "F", "E", "D", "C", "B", "A"]
        } else {
            vec!["A", "B", "C", "D", "E", "F", "G", "H"]
        };

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 8); 8])
            .split(area);

        for (i, file) in files.iter().enumerate() {
            let file_area = layout[i];
            // Add minimal top padding to keep letters close to the board
            // Put most padding at the bottom for slight vertical centering
            let top_padding = if file_area.height > 2 { 1 } else { 0 };
            let bottom_padding = file_area.height.saturating_sub(1 + top_padding);

            // Create text with minimal top padding
            let mut lines: Vec<Line> = Vec::new();
            for _ in 0..top_padding {
                lines.push(Line::from(""));
            }
            lines.push(
                Line::from(*file)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Gray)),
            );
            for _ in 0..bottom_padding {
                lines.push(Line::from(""));
            }

            let label = Paragraph::new(lines);
            frame.render_widget(label, file_area);
        }
    }
}
