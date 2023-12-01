use crate::{
    constants::{BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
        PieceColor, PieceType,
    },
    utils::{
        convert_position_into_notation, get_king_coordinates, get_piece_color, get_piece_type,
        get_player_turn_in_modulo, is_getting_checked, is_valid,
    },
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

#[derive(Debug)]
pub struct Board {
    pub board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    pub cursor_coordinates: [i8; 2],
    pub selected_coordinates: [i8; 2],
    pub selected_piece_cursor: i8,
    pub old_cursor_position: [i8; 2],
    pub player_turn: PieceColor,
    pub moves_history: Vec<(Option<PieceType>, String)>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                [
                    Some((PieceType::Rook, PieceColor::Black)),
                    Some((PieceType::Knight, PieceColor::Black)),
                    Some((PieceType::Bishop, PieceColor::Black)),
                    Some((PieceType::Queen, PieceColor::Black)),
                    Some((PieceType::King, PieceColor::Black)),
                    Some((PieceType::Bishop, PieceColor::Black)),
                    Some((PieceType::Knight, PieceColor::Black)),
                    Some((PieceType::Rook, PieceColor::Black)),
                ],
                [
                    Some((PieceType::Pawn, PieceColor::Black)),
                    Some((PieceType::Pawn, PieceColor::Black)),
                    Some((PieceType::Pawn, PieceColor::Black)),
                    Some((PieceType::Pawn, PieceColor::Black)),
                    Some((PieceType::Pawn, PieceColor::Black)),
                    Some((PieceType::Pawn, PieceColor::Black)),
                    Some((PieceType::Pawn, PieceColor::Black)),
                    Some((PieceType::Pawn, PieceColor::Black)),
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some((PieceType::Pawn, PieceColor::White)),
                    Some((PieceType::Pawn, PieceColor::White)),
                    Some((PieceType::Pawn, PieceColor::White)),
                    Some((PieceType::Pawn, PieceColor::White)),
                    Some((PieceType::Pawn, PieceColor::White)),
                    Some((PieceType::Pawn, PieceColor::White)),
                    Some((PieceType::Pawn, PieceColor::White)),
                    Some((PieceType::Pawn, PieceColor::White)),
                ],
                [
                    Some((PieceType::Rook, PieceColor::White)),
                    Some((PieceType::Knight, PieceColor::White)),
                    Some((PieceType::Bishop, PieceColor::White)),
                    Some((PieceType::Queen, PieceColor::White)),
                    Some((PieceType::King, PieceColor::White)),
                    Some((PieceType::Bishop, PieceColor::White)),
                    Some((PieceType::Knight, PieceColor::White)),
                    Some((PieceType::King, PieceColor::White)),
                ],
            ],
            cursor_coordinates: [4, 4],
            selected_coordinates: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            selected_piece_cursor: 0,
            old_cursor_position: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            player_turn: PieceColor::White,
            moves_history: vec![],
        }
    }
}

impl Board {
    pub fn new(
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        player_turn: PieceColor,
        moves_history: Vec<(Option<PieceType>, String)>,
    ) -> Self {
        Self {
            board,
            cursor_coordinates: [4, 4],
            selected_coordinates: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            selected_piece_cursor: 0,
            old_cursor_position: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            player_turn,
            moves_history,
        }
    }

    // Setters
    pub fn set_board(&mut self, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) {
        self.board = board;
    }

    pub fn set_player_turn(&mut self, player_turn: PieceColor) {
        self.player_turn = player_turn;
    }
    // Check if a cell has been selected
    fn is_cell_selected(&self) -> bool {
        self.selected_coordinates[0] != UNDEFINED_POSITION
            && self.selected_coordinates[1] != UNDEFINED_POSITION
    }

    fn get_authorized_positions(
        &self,
        piece_type: Option<PieceType>,
        piece_color: Option<PieceColor>,
        coordinates: [i8; 2],
    ) -> Vec<Vec<i8>> {
        match (piece_type, piece_color) {
            (Some(piece_type), Some(piece_color)) => piece_type.authorized_positions(
                coordinates,
                piece_color,
                self.board,
                self.moves_history.clone(),
                is_getting_checked(self.board, self.player_turn, self.moves_history.clone()),
            ),
            _ => Vec::new(),
        }
    }
    pub fn switch_player_turn(&mut self) {
        match self.player_turn {
            PieceColor::White => self.player_turn = PieceColor::Black,
            PieceColor::Black => self.player_turn = PieceColor::White,
        }
    }

    // Methods to change the position of the cursor
    pub fn cursor_up(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1)
        } else if self.cursor_coordinates[0] > 0 {
            self.cursor_coordinates[0] -= 1
        }
    }
    pub fn cursor_down(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, 1)
        } else if self.cursor_coordinates[0] < 7 {
            self.cursor_coordinates[0] += 1
        }
    }
    pub fn cursor_left(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1)
        } else if self.cursor_coordinates[1] > 0 {
            self.cursor_coordinates[1] -= 1
        }
    }
    pub fn cursor_right(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, 1)
        } else if self.cursor_coordinates[1] < 7 {
            self.cursor_coordinates[1] += 1
        }
    }

    pub fn did_king_already_move(&self) -> bool {
        for i in 0..self.moves_history.len() {
            match self.moves_history[i] {
                (Some(piece_type), _) => {
                    if piece_type == PieceType::King
                        && get_player_turn_in_modulo(self.player_turn) == i % 2
                    {
                        return true;
                    }
                }
                _ => unreachable!("Invalid move in history"),
            }
        }
        false
    }

    fn move_selected_piece_cursor(&mut self, first_time_moving: bool, direction: i8) {
        let piece_color = get_piece_color(self.board, self.selected_coordinates);
        let piece_type = get_piece_type(self.board, self.selected_coordinates);

        let mut authorized_positions =
            self.get_authorized_positions(piece_type, piece_color, self.selected_coordinates);

        if !authorized_positions.is_empty() {
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
                self.cursor_coordinates = [position[0], position[1]];
            }
        } else {
            self.cursor_coordinates = [UNDEFINED_POSITION, UNDEFINED_POSITION];
        }
    }

    // Methods to select a cell on the board
    pub fn select_cell(&mut self) {
        if !self.is_cell_selected() {
            match get_piece_color(self.board, self.cursor_coordinates) {
                Some(piece_color) => {
                    if piece_color == self.player_turn {
                        self.selected_coordinates = self.cursor_coordinates;
                        self.old_cursor_position = self.cursor_coordinates;
                        self.move_selected_piece_cursor(true, 1);
                    }
                }
                _ => {}
            }
        } else {
            // We already selected a piece
            if is_valid(self.cursor_coordinates) {
                let selected_coords_usize: [usize; 2] = [
                    self.selected_coordinates[0] as usize,
                    self.selected_coordinates[1] as usize,
                ];
                let cursor_coords_usize: [usize; 2] = [
                    self.cursor_coordinates[0] as usize,
                    self.cursor_coordinates[1] as usize,
                ];
                self.move_piece_on_the_board(selected_coords_usize, cursor_coords_usize);
                self.unselect_cell();
                self.switch_player_turn();
            }
        }
    }

    fn is_latest_move_en_passant(&self, from: [usize; 2], to: [usize; 2]) -> bool {
        let piece_type_from = get_piece_type(self.board, [from[0] as i8, from[1] as i8]);
        let piece_type_to = get_piece_type(self.board, [to[0] as i8, to[1] as i8]);

        let from_y: i32 = from[0] as i32;
        let from_x: i32 = from[1] as i32;
        let to_y: i32 = to[0] as i32;
        let to_x: i32 = to[1] as i32;
        match (piece_type_from, piece_type_to) {
            (Some(PieceType::Pawn), _) => {
                // Check if it's a diagonal move, and the destination is an empty cell
                from_y != to_y && from_x != to_x && self.board[to[0]][to[1]].is_none()
            }
            _ => false,
        }
    }

    fn is_latest_move_castling(&self, from: [usize; 2], to: [usize; 2]) -> bool {
        let piece_type_from = get_piece_type(self.board, [from[0] as i8, from[1] as i8]);
        let piece_type_to = get_piece_type(self.board, [to[0] as i8, to[1] as i8]);

        let from_x: i32 = from[1] as i32;
        let to_x: i32 = to[1] as i32;
        let distance = (from_x - to_x).abs();

        match (piece_type_from, piece_type_to) {
            (Some(PieceType::King), _) => {
                // Check if it's a move on more
                distance > 1
            }
            _ => false,
        }
    }

    pub fn move_piece_on_the_board(&mut self, from: [usize; 2], to: [usize; 2]) {
        let direction_y: i32 = if self.player_turn == PieceColor::White {
            -1
        } else {
            1
        };

        let piece_type_from = get_piece_type(self.board, [from[0] as i8, from[1] as i8]);
        let position_number: String = format!("{}{}{}{}", from[0], from[1], to[0], to[1]);

        let tuple = (piece_type_from, position_number);

        // We check for en passant as the latest move
        if self.is_latest_move_en_passant(from, to) {
            // we kill the pawn
            let row_index = to[0] as i32 - direction_y;

            self.board[row_index as usize][to[1]] = None;
        }

        // We check for castling as the latest move
        if self.is_latest_move_castling(from, to) {
            // we set the king 2 cells on where it came from

            let from_x: i32 = from[1] as i32;
            let to_x: i32 = to[1] as i32;
            let distance = from_x - to_x;
            let direction_x = if distance > 0 { -1 } else { 1 };
            let mut row_index_rook = 0;

            let row_index = from[1] as i32 + direction_x * 2;
            // We put move the king 2 cells
            self.board[to[0]][row_index as usize] = self.board[from[0]][from[1]];

            // We put the rook 3 cells from it's position if it's a big castling else 2 cells
            // big castling
            if distance == 4 {
                row_index_rook = 3
            } else if distance == -3 {
                row_index_rook = 5
            }
            self.board[to[0]][row_index_rook as usize] = self.board[to[0]][to[1]];

            // We remove the latest rook
            self.board[to[0]][to[1]] = None;
        } else {
            self.board[to[0]][to[1]] = self.board[from[0]][from[1]];
        }
        self.board[from[0]][from[1]] = None;

        // We store it in the history
        self.moves_history.push(tuple.clone());
    }

    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_coordinates[0] = UNDEFINED_POSITION;
            self.selected_coordinates[1] = UNDEFINED_POSITION;
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position
        }
    }

    pub fn color_to_ratatui_enum(&self, piece_color: Option<PieceColor>) -> Color {
        match piece_color {
            Some(PieceColor::Black) => Color::Black,
            Some(PieceColor::White) => Color::White,
            None => Color::Red,
        }
    }
    pub fn piece_type_to_string_enum(&self, piece_type: Option<PieceType>) -> &'static str {
        match piece_type {
            Some(PieceType::Queen) => Queen::to_string(),
            Some(PieceType::King) => King::to_string(),
            Some(PieceType::Rook) => Rook::to_string(),
            Some(PieceType::Bishop) => Bishop::to_string(),
            Some(PieceType::Knight) => Knight::to_string(),
            Some(PieceType::Pawn) => Pawn::to_string(),
            None => " ",
        }
    }

    pub fn piece_type_to_utf_enum(&self, piece_type: Option<PieceType>) -> &'static str {
        match piece_type {
            Some(PieceType::Queen) => "♛",
            Some(PieceType::King) => "♚",
            Some(PieceType::Rook) => "♜",
            Some(PieceType::Bishop) => "♝",
            Some(PieceType::Knight) => "♞",
            Some(PieceType::Pawn) => "♟",
            None => "NONE",
        }
    }

    // Method to render the board
    pub fn board_render(&self, area: Rect, frame: &mut Frame) {
        let width = area.width / 8;
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);
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
        for i in 0..8i8 {
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
            for j in 0..8i8 {
                // Color of the cell to draw the board
                let mut cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                // Draw the available moves for the selected piece
                if self.is_cell_selected() {
                    let selected_piece_type = get_piece_type(self.board, self.selected_coordinates);
                    let selected_piece_color: Option<PieceColor> =
                        get_piece_color(self.board, self.selected_coordinates);
                    let positions = self.get_authorized_positions(
                        selected_piece_type,
                        selected_piece_color,
                        self.selected_coordinates,
                    );

                    for coords in positions.clone() {
                        if i == coords[0] && j == coords[1] {
                            cell_color = Color::LightRed
                        }
                    }
                }

                let square = lines[j as usize + 1];
                // Draw the cell blue if this is the current cursor cell
                if i == self.cursor_coordinates[0] && j == self.cursor_coordinates[1] {
                    let cell = Block::default()
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::RAPID_BLINK);
                    frame.render_widget(cell.clone(), square);
                } else if is_getting_checked(
                    self.board,
                    self.player_turn,
                    self.moves_history.clone(),
                ) && [i, j] == get_king_coordinates(self.board, self.player_turn)
                {
                    let cell = Block::default()
                        .bg(Color::Magenta)
                        .add_modifier(Modifier::SLOW_BLINK);
                    frame.render_widget(cell.clone(), square);
                }
                // Draw the cell green if this is the selected cell
                else if i == self.selected_coordinates[0] && j == self.selected_coordinates[1] {
                    let cell = Block::default().bg(Color::LightGreen);
                    frame.render_widget(cell.clone(), square);
                } else {
                    let cell = Block::default().bg(cell_color);
                    frame.render_widget(cell.clone(), square);
                }

                // We check if the current king is getting checked

                // Get piece and color
                let piece_color = get_piece_color(self.board, [i, j]);
                let piece_type = get_piece_type(self.board, [i, j]);

                let color_enum = self.color_to_ratatui_enum(piece_color);
                let piece_enum = self.piece_type_to_string_enum(piece_type);

                // Place the pieces on the board
                let paragraph = Paragraph::new(piece_enum)
                    .alignment(Alignment::Center)
                    .fg(color_enum);
                frame.render_widget(paragraph, square);
            }
        }
    }

    pub fn history_render(&self, area: Rect, frame: &mut Frame) {
        // We write the history board on the side
        let history_block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(5, 10, 1, 2));

        let mut lines: Vec<Line> = vec![];

        for i in (0..self.moves_history.len()).step_by(2) {
            let piece_type_from = self.moves_history[i].0;
            let utf_icon_white = self.piece_type_to_utf_enum(piece_type_from);
            let number_move = &self.moves_history[i].1;
            let move_white = convert_position_into_notation(number_move.to_string());

            let mut utf_icon_black = "   ";
            let mut move_black: String = "   ".to_string();

            // If there is something for black
            if i + 1 < self.moves_history.len() {
                let piece_type_to = self.moves_history[i + 1].0;
                let number = &self.moves_history[i + 1].1;
                move_black = convert_position_into_notation(number.to_string());
                utf_icon_black = self.piece_type_to_utf_enum(piece_type_to)
            }

            lines.push(Line::from(vec![
                Span::raw(format!("{}.  ", i / 2 + 1)), // line number
                Span::styled(format!("{} ", utf_icon_white), Style::default().fg(WHITE)), // white symbol
                Span::raw(move_white.to_string()), // white move
                Span::raw("     "),                // separator
                Span::styled(
                    format!("{} ", utf_icon_black),
                    Style::default().fg(Color::Black),
                ), // white symbol
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

        // Bottom paragraph help text
        let text = vec![Line::from("Press h for help").alignment(Alignment::Center)];

        let help_paragraph = Paragraph::new(text)
            .block(Block::new())
            .alignment(Alignment::Center);
        frame.render_widget(help_paragraph, right_panel_layout[1]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{PieceColor, PieceType},
        utils::is_getting_checked,
    };

    #[test]
    fn is_getting_checked_true() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(is_getting_checked(custom_board, PieceColor::White, vec![]));
    }

    #[test]
    fn is_getting_checked_false() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::White, vec![]));
    }

    #[test]
    fn is_getting_checked_piece_in_front_false() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::Black, vec![]));
    }

    #[test]
    fn is_getting_checked_piece_in_with_gap_false() {
        let custom_board = [
            [
                Some((PieceType::Rook, PieceColor::Black)),
                Some((PieceType::Knight, PieceColor::Black)),
                Some((PieceType::Bishop, PieceColor::Black)),
                Some((PieceType::Queen, PieceColor::Black)),
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
            ],
            [
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::Black)),
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
            ],
            [
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Knight, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                Some((PieceType::Queen, PieceColor::White)),
                Some((PieceType::Rook, PieceColor::White)),
                Some((PieceType::Bishop, PieceColor::White)),
                None,
                Some((PieceType::King, PieceColor::White)),
            ],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        assert!(!is_getting_checked(custom_board, PieceColor::Black, vec![]));
    }
}
