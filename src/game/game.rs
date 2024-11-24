use super::{coord::Coord, game_board::GameBoard, ui::UI};
use crate::{
    constants::{DisplayMode, BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{PieceColor, PieceMove, PieceType},
    utils::{
        convert_notation_into_position, convert_position_into_notation, get_cell_paragraph,
        get_int_from_char, invert_position,
    },
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};
use uci::Engine;

pub struct Game {
    // the actual board
    pub game_board: GameBoard,
    // The struct to handle UI related stuff
    pub ui: UI,
    // the player turn
    pub player_turn: PieceColor,
    // if the game is a draw
    pub is_draw: bool,
    // if the game is a checkmate
    pub is_checkmate: bool,
    // if we are doing a promotion
    pub is_promotion: bool,
    // the chess engine
    pub engine: Option<Engine>,
    // if the game is against a bot
    pub is_game_against_bot: bool,
    // the display mode
    pub display_mode: DisplayMode,
    /// Used to indicate if a bot move is following
    pub bot_will_move: bool,

    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
    // The white piece that got taken
    pub white_taken_pieces: Vec<PieceType>,
    // The black piece that got taken
    pub black_taken_pieces: Vec<PieceType>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            game_board: GameBoard::default(),
            ui: UI::default(),
            player_turn: PieceColor::White,
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            engine: None,
            is_game_against_bot: false,
            display_mode: DisplayMode::DEFAULT,
            bot_will_move: false,
            is_bot_starting: false,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }
}

impl Game {
    pub fn new(game_board: GameBoard, player_turn: PieceColor) -> Self {
        Self {
            game_board,
            ui: UI::default(),
            player_turn,
            is_draw: false,
            is_checkmate: false,
            is_promotion: false,
            engine: None,
            is_game_against_bot: false,
            display_mode: DisplayMode::DEFAULT,
            bot_will_move: false,
            is_bot_starting: false,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }

    // Setters
    pub fn set_board(&mut self, game_board: GameBoard) {
        self.game_board = game_board;
    }

    pub fn set_player_turn(&mut self, player_turn: PieceColor) {
        self.player_turn = player_turn;
    }

    pub fn set_engine(&mut self, engine_path: &str) {
        self.is_game_against_bot = true;

        self.engine = match Engine::new(engine_path) {
            Ok(engine) => Some(engine),
            _ => panic!("An error occcured with the selected chess engine path: {engine_path} Make sure you specified the right path using chess-tui -e"),
        }
    }

    pub fn switch_player_turn(&mut self) {
        match self.player_turn {
            PieceColor::White => self.player_turn = PieceColor::Black,
            PieceColor::Black => self.player_turn = PieceColor::White,
        }
    }

    pub fn move_selected_piece_cursor_mouse(&mut self, coordinates: Coord) {
        let piece_color = self
            .game_board
            .get_piece_color(&self.ui.selected_coordinates);

        let authorized_positions = self
            .game_board
            .get_authorized_positions(self.player_turn, self.ui.selected_coordinates);

        if authorized_positions.contains(&coordinates)
            && match piece_color {
                Some(piece) => piece == self.player_turn,
                None => false,
            }
        {
            self.ui.cursor_coordinates = coordinates;
            self.select_cell();
        } else {
            self.ui.selected_coordinates = coordinates;
        }
    }

    // Methods to select a cell on the board
    pub fn select_cell(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.is_promotion {
            self.promote_piece();
        } else if !self.is_checkmate && !self.is_draw {
            if self.ui.is_cell_selected() {
                // We already selected a piece so we apply the move
                if self.ui.cursor_coordinates.is_valid() {
                    let selected_coords_usize = &self.ui.selected_coordinates.clone();
                    let cursor_coords_usize = &self.ui.cursor_coordinates.clone();
                    self.move_piece_on_the_board(selected_coords_usize, cursor_coords_usize);
                    self.ui.unselect_cell();
                    self.switch_player_turn();
                    self.is_draw = self.game_board.is_draw(self.player_turn);
                    if (!self.is_game_against_bot || self.is_bot_starting)
                        && (!self.game_board.is_latest_move_promotion()
                            || self.game_board.is_draw(self.player_turn)
                            || self.game_board.is_checkmate(self.player_turn))
                    {
                        self.game_board.flip_the_board();
                    }
                    // If we play against a bot we will play his move and switch the player turn again
                    if self.is_game_against_bot {
                        // do this in background
                        self.is_promotion = self.game_board.is_latest_move_promotion();
                        if !self.is_promotion {
                            self.is_checkmate = self.game_board.is_checkmate(self.player_turn);
                            self.is_promotion = self.game_board.is_latest_move_promotion();
                            if !self.is_checkmate {
                                self.bot_will_move = true;
                            }
                        }
                    }
                }
            } else {
                // Check if the piece on the cell can move before selecting it
                let authorized_positions = self
                    .game_board
                    .get_authorized_positions(self.player_turn, self.ui.cursor_coordinates);

                if authorized_positions.is_empty() {
                    return;
                }
                if let Some(piece_color) =
                    self.game_board.get_piece_color(&self.ui.cursor_coordinates)
                {
                    let mut authorized_positions = self
                        .game_board
                        .get_authorized_positions(self.player_turn, self.ui.selected_coordinates);

                    if piece_color == self.player_turn {
                        self.ui.selected_coordinates = self.ui.cursor_coordinates;
                        self.ui.old_cursor_position = self.ui.cursor_coordinates;
                        self.ui
                            .move_selected_piece_cursor(true, 1, authorized_positions);
                    }
                }
            }
        }
        self.is_checkmate = self.game_board.is_checkmate(self.player_turn);
        self.is_promotion = self.game_board.is_latest_move_promotion();
        self.is_draw = self.game_board.is_draw(self.player_turn);
    }

    /* Method to make a move for the bot
       We use the UCI protocol to communicate with the chess engine
    */
    pub fn bot_move(&mut self) {
        let engine = self.engine.clone().expect("Missing the chess engine");
        let fen_position = self
            .game_board
            .fen_position(self.is_bot_starting, self.player_turn);

        engine.set_position(&(fen_position as String)).unwrap();
        let best_move = engine.bestmove();
        let Ok(movement) = best_move else {
            panic!("An error has occured")
        };
        let converted_move = convert_notation_into_position(&movement);

        let from_y = get_int_from_char(converted_move.chars().next());
        let from_x = get_int_from_char(converted_move.chars().nth(1));
        let to_y = get_int_from_char(converted_move.chars().nth(2));
        let to_x = get_int_from_char(converted_move.chars().nth(3));

        let mut promotion_piece: Option<PieceType> = None;
        if movement.chars().count() == 5 {
            promotion_piece = match movement.chars().nth(4) {
                Some('q') => Some(PieceType::Queen),
                Some('r') => Some(PieceType::Rook),
                Some('b') => Some(PieceType::Bishop),
                Some('n') => Some(PieceType::Knight),
                _ => None,
            };
        }

        self.move_piece_on_the_board(
            &Coord::new(from_y as u8, from_x as u8),
            &Coord::new(to_y as u8, to_x as u8),
        );

        if promotion_piece.is_some() {
            self.game_board.board[to_y as usize][to_x as usize] =
                Some((promotion_piece.unwrap(), self.player_turn));
        }
        if self.is_bot_starting {
            self.game_board.flip_the_board();
        }
    }

    // Method to promote a pawn
    pub fn promote_piece(&mut self) {
        if let Some(last_move) = self.game_board.move_history.last() {
            let new_piece = match self.ui.promotion_cursor {
                0 => PieceType::Queen,
                1 => PieceType::Rook,
                2 => PieceType::Bishop,
                3 => PieceType::Knight,
                _ => unreachable!("Promotion cursor out of boundaries"),
            };

            let current_piece_color = self
                .game_board
                .get_piece_color(&Coord::new(last_move.to.row, last_move.to.col));
            if let Some(piece_color) = current_piece_color {
                // we replace the piece by the new piece type
                self.game_board.board[last_move.to.row as usize][last_move.to.col as usize] =
                    Some((new_piece, piece_color));
            }
        }
        self.is_promotion = false;
        self.ui.promotion_cursor = 0;
        if !self.game_board.is_draw(self.player_turn)
            && !self.game_board.is_checkmate(self.player_turn)
        {
            self.game_board.flip_the_board();
        }
    }

    // Move a piece from a cell to another
    pub fn move_piece_on_the_board(&mut self, from: &Coord, to: &Coord) {
        if !from.is_valid() || !to.is_valid() {
            return;
        }

        let piece_type_from = self.game_board.get_piece_type(from);
        let piece_type_to = self.game_board.get_piece_type(to);

        // Check if moving a piece
        let Some(piece_type_from) = piece_type_from else {
            return;
        };

        // We increment the consecutive_non_pawn_or_capture if the piece type is a pawn or if there is no capture
        match (piece_type_from, piece_type_to) {
            (PieceType::Pawn, _) | (_, Some(_)) => {
                self.game_board.set_consecutive_non_pawn_or_capture(0);
            }
            _ => {
                let value = self.game_board.get_consecutive_non_pawn_or_capture() + 1;
                self.game_board.set_consecutive_non_pawn_or_capture(value);
            }
        }

        // We check if the move is a capture and add the piece to the taken pieces
        match (piece_type_from, piece_type_to) {
            (_, None) => {}
            (_, Some(piece)) => {
                let piece_color = self.game_board.get_piece_color(to);
                // We check if there is a piece and we are not doing a castle
                if piece_color.is_some()
                    && (piece_type_to != Some(PieceType::Rook)
                        && piece_color != Some(self.player_turn))
                {
                    match piece_color {
                        Some(PieceColor::Black) => {
                            self.white_taken_pieces.push(piece);
                            self.white_taken_pieces.sort();
                        }
                        Some(PieceColor::White) => {
                            self.black_taken_pieces.push(piece);
                            self.black_taken_pieces.sort();
                        }
                        _ => {}
                    }
                }
            }
        }

        // We check for en passant as the latest move
        if self.game_board.is_latest_move_en_passant(*from, *to) {
            // we kill the pawn
            let row_index = to.row as i32 + 1;

            self.game_board.board[row_index as usize][to.col as usize] = None;
        }

        // We check for castling as the latest move
        if self.game_board.is_latest_move_castling(*from, *to) {
            // we set the king 2 cells on where it came from
            let from_x: i32 = from.col as i32;
            let mut new_to = to;
            let to_x: i32 = to.col as i32;

            let distance = from_x - to_x;
            // We set the direction of the rook > 0 meaning he went on the left else on the right
            let direction_x = if distance > 0 { -1 } else { 1 };

            let col_king = from_x + direction_x * 2;

            // We put move the king 2 cells
            self.game_board.board[to.row as usize][col_king as usize] = self.game_board.board[from];

            // We put the rook 3 cells from it's position if it's a big castling else 2 cells
            // If it is playing against a bot we will receive 4 -> 6  and 4 -> 2 for to_x instead of 4 -> 7 and 4 -> 0
            if self.is_game_against_bot && to_x == 6 && to.row == 0 {
                new_to = &Coord { row: 0, col: 7 };
            }
            if self.is_game_against_bot && to_x == 2 && to.row == 0 {
                new_to = &Coord { row: 0, col: 0 };
            }

            let col_rook = if distance > 0 {
                col_king + 1
            } else {
                col_king - 1
            };

            self.game_board.board[new_to.row as usize][col_rook as usize] =
                Some((PieceType::Rook, self.player_turn));

            // We remove the latest rook
            self.game_board.board[new_to] = None;
        } else {
            self.game_board.board[to] = self.game_board.board[from];
        }

        self.game_board.board[from] = None;

        // We store it in the history
        self.game_board.move_history.push(PieceMove {
            piece_type: piece_type_from,
            piece_color: self.player_turn,
            from: *from,
            to: *to,
        });
        // We store the current position of the board
        self.game_board.board_history.push(self.game_board.board);
    }

    // Method to render the board
    pub fn board_render(&mut self, area: Rect, frame: &mut Frame) {
        let width = area.width / 8;
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);

        // we update the starting coordinates
        self.ui.top_x = area.x + border_width;
        self.ui.top_y = area.y + border_height;
        self.ui.width = width;
        self.ui.height = height;
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
                let cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                let last_move;
                let mut last_move_from = Coord::undefined();
                let mut last_move_to = Coord::undefined();
                if !self.game_board.move_history.is_empty() {
                    last_move = self.game_board.move_history.last();
                    if self.is_game_against_bot && !self.is_bot_starting {
                        last_move_from = last_move.map(|m| m.from).unwrap();
                        last_move_to = last_move.map(|m| m.to).unwrap();
                    } else {
                        last_move_from = invert_position(&last_move.map(|m| m.from).unwrap());
                        last_move_to = invert_position(&last_move.map(|m| m.to).unwrap());
                    }
                }

                let mut positions: Vec<Coord> = vec![];
                let is_cell_in_positions = |positions: &Vec<Coord>, i: u8, j: u8| {
                    positions.iter().any(|&coord| coord == Coord::new(i, j))
                };
                // Draw the available moves for the selected piece
                if self.ui.is_cell_selected() {
                    let selected_piece_color: Option<PieceColor> = self
                        .game_board
                        .get_piece_color(&self.ui.selected_coordinates);
                    // only draw available moves if it is the right players turn
                    if match selected_piece_color {
                        Some(color) => color == self.player_turn,
                        None => false,
                    } {
                        positions = self.game_board.get_authorized_positions(
                            self.player_turn,
                            self.ui.selected_coordinates,
                        );

                        // Draw grey if the color is in the authorized positions
                        for coords in positions.clone() {
                            if i == coords.row && j == coords.col {
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
                if i == self.ui.cursor_coordinates.row
                    && j == self.ui.cursor_coordinates.col
                    && !self.ui.mouse_used
                {
                    Game::render_cell(frame, square, Color::LightBlue, None);
                }
                // Draw the cell magenta if the king is getting checked
                else if self
                    .game_board
                    .is_getting_checked(self.game_board.board, self.player_turn)
                    && Coord::new(i, j)
                        == self
                            .game_board
                            .get_king_coordinates(self.game_board.board, self.player_turn)
                {
                    Game::render_cell(frame, square, Color::Magenta, Some(Modifier::SLOW_BLINK));
                }
                // Draw the cell green if this is the selected cell or if the cell is part of the last move
                else if (i == self.ui.selected_coordinates.row
                    && j == self.ui.selected_coordinates.col)
                    || (last_move_from == Coord::new(i, j) // If the last move from 
                        || (last_move_to == Coord::new(i, j) // If last move to
                            && !is_cell_in_positions(&positions, i, j)))
                // and not in the authorized positions (grey instead of green)
                {
                    Game::render_cell(frame, square, Color::LightGreen, None);
                } else if is_cell_in_positions(&positions, i, j) {
                    Game::render_cell(frame, square, Color::Rgb(100, 100, 100), None);
                }
                // else as a last resort we draw the cell with the default color either white or black
                else {
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

                // Get piece and color
                let coord = Coord::new(i, j);
                let paragraph = get_cell_paragraph(self, &coord, square);

                frame.render_widget(paragraph, square);
            }
        }
    }
    fn render_cell(frame: &mut Frame, square: Rect, color: Color, modifier: Option<Modifier>) {
        let mut cell = Block::default().bg(color);
        if let Some(modifier) = modifier {
            cell = cell.add_modifier(modifier);
        }
        frame.render_widget(cell, square);
    }

    // Method to render the right panel history
    pub fn history_render(&self, area: Rect, frame: &mut Frame) {
        // We write the history board on the side
        let history_block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(5, 10, 1, 2));

        let mut lines: Vec<Line> = vec![];

        for i in (0..self.game_board.move_history.len()).step_by(2) {
            let piece_type_from = self.game_board.move_history[i].piece_type;

            let utf_icon_white =
                PieceType::piece_to_utf_enum(piece_type_from, Some(PieceColor::White));
            let move_white = convert_position_into_notation(&format!(
                "{}{}{}{}",
                self.game_board.move_history[i].from.row,
                self.game_board.move_history[i].from.col,
                self.game_board.move_history[i].to.row,
                self.game_board.move_history[i].to.col
            ));

            let mut utf_icon_black = "   ";
            let mut move_black: String = "   ".to_string();

            // If there is something for black
            if i + 1 < self.game_board.move_history.len() {
                let piece_type_to = self.game_board.move_history[i + 1].piece_type;

                move_black = convert_position_into_notation(&format!(
                    "{}{}{}{}",
                    self.game_board.move_history[i + 1].from.row,
                    self.game_board.move_history[i + 1].from.col,
                    self.game_board.move_history[i + 1].to.row,
                    self.game_board.move_history[i + 1].to.col
                ));
                utf_icon_black =
                    PieceType::piece_to_utf_enum(piece_type_to, Some(PieceColor::Black));
            }

            lines.push(Line::from(vec![
                Span::raw(format!("{}.  ", i / 2 + 1)), // line number
                Span::styled(format!("{utf_icon_white} "), Style::default().fg(WHITE)), // white symbol
                Span::raw(move_white.to_string()), // white move
                Span::raw("     "),                // separator
                Span::styled(format!("{utf_icon_black} "), Style::default().fg(WHITE)), // white symbol
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

    pub fn white_material_render(&self, area: Rect, frame: &mut Frame) {
        let white_block = Block::default()
            .title("White material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for i in 0..self.white_taken_pieces.len() {
            let utf_icon_white =
                PieceType::piece_to_utf_enum(self.white_taken_pieces[i], Some(PieceColor::Black));

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

    pub fn black_material_render(&self, area: Rect, frame: &mut Frame) {
        let black_block = Block::default()
            .title("Black material")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded);

        let mut pieces: String = String::new();

        for i in 0..self.black_taken_pieces.len() {
            let utf_icon_black =
                PieceType::piece_to_utf_enum(self.black_taken_pieces[i], Some(PieceColor::White));

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
}
