use crate::{
    constants::{BLACK, UNDEFINED_POSITION, WHITE},
    pieces::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
        PieceColor, PieceType,
    },
    utils::{convert_position_into_notation, get_piece_color, get_piece_type, is_valid},
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
    pub moves_historic: Vec<(Option<PieceType>, i32)>,
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
                [None, None, None, None, None, None, None, None],
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
                    Some((PieceType::Rook, PieceColor::White)),
                ],
            ],
            cursor_coordinates: [4, 4],
            selected_coordinates: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            selected_piece_cursor: 0,
            old_cursor_position: [UNDEFINED_POSITION, UNDEFINED_POSITION],
            player_turn: PieceColor::White,
            moves_historic: vec![],
        }
    }
}

impl Board {
    // Setters
    pub fn set_board(&mut self, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) {
        self.board = board;
    }

    // Getters
    pub fn authorized_positions_enum(
        &mut self,
        selected_coordinates: [i8; 2],
        piece_type: PieceType,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    ) -> Vec<Vec<i8>> {
        match piece_type {
            PieceType::Pawn => Pawn::authorized_positions(
                selected_coordinates,
                color,
                board,
                self.get_latest_move(),
            ),
            PieceType::Rook => Rook::authorized_positions(selected_coordinates, color, board),
            PieceType::Bishop => Bishop::authorized_positions(selected_coordinates, color, board),
            PieceType::Queen => Queen::authorized_positions(selected_coordinates, color, board),
            PieceType::King => King::authorized_positions(selected_coordinates, color, board),
            PieceType::Knight => Knight::authorized_positions(selected_coordinates, color, board),
        }
    }

    pub fn protected_positions_enum(
        selected_coordinates: [i8; 2],
        piece_type: PieceType,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    ) -> Vec<Vec<i8>> {
        match piece_type {
            PieceType::Pawn => Pawn::protecting_positions(selected_coordinates, color, board),
            PieceType::Rook => Rook::protecting_positions(selected_coordinates, color, board),
            PieceType::Bishop => Bishop::protecting_positions(selected_coordinates, color, board),
            PieceType::Queen => Queen::protecting_positions(selected_coordinates, color, board),
            PieceType::King => King::protecting_positions(selected_coordinates, color, board),
            PieceType::Knight => Knight::protecting_positions(selected_coordinates, color, board),
        }
    }

    // Check if a cell has been selected
    fn is_cell_selected(&mut self) -> bool {
        self.selected_coordinates[0] != UNDEFINED_POSITION
            && self.selected_coordinates[1] != UNDEFINED_POSITION
    }

    fn get_latest_move(&mut self) -> (Option<PieceType>, i32) {
        if self.moves_historic.len() > 0 {
            return self.moves_historic[self.moves_historic.len() - 1];
        }
        (None, 0)
    }

    fn get_authorized_positions(
        &mut self,
        piece_type: Option<PieceType>,
        piece_color: Option<PieceColor>,
        coordinates: [i8; 2],
    ) -> Vec<Vec<i8>> {
        match (piece_type, piece_color) {
            (Some(piece_type), Some(piece_color)) => {
                self.authorized_positions_enum(coordinates, piece_type, piece_color, self.board)
            }
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

    pub fn move_piece_on_the_board(&mut self, from: [usize; 2], to: [usize; 2]) {
        let direction: i32 = if self.player_turn == PieceColor::White {
            -1
        } else {
            1
        };

        let piece_type_from = get_piece_type(self.board, [from[0] as i8, from[1] as i8]);
        let position_number: i32 =
            from[0] as i32 * 1000 + from[1] as i32 * 100 + to[0] as i32 * 10 + to[1] as i32;

        let tuple = (piece_type_from, position_number);
        // We store it in the historic
        self.moves_historic.push(tuple);
        if self.is_latest_move_en_passant(from, to) {
            // we kill the pawn
            let row_index = to[0] as i32 - direction;

            self.board[row_index as usize][to[1]] = None;
        }
        self.board[to[0]][to[1]] = self.board[from[0]][from[1]];
        self.board[from[0]][from[1]] = None;
        // We check if it is en passant or not
    }

    pub fn unselect_cell(&mut self) {
        if self.is_cell_selected() {
            self.selected_coordinates[0] = UNDEFINED_POSITION;
            self.selected_coordinates[1] = UNDEFINED_POSITION;
            self.selected_piece_cursor = 0;
            self.cursor_coordinates = self.old_cursor_position
        }
    }

    pub fn color_to_ratatui_enum(&mut self, piece_color: Option<PieceColor>) -> Color {
        match piece_color {
            Some(PieceColor::Black) => Color::Black,
            Some(PieceColor::White) => Color::White,
            None => Color::Red,
        }
    }
    pub fn piece_type_to_string_enum(&mut self, piece_type: Option<PieceType>) -> &'static str {
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

    pub fn piece_type_to_utf_enum(&mut self, piece_type: Option<PieceType>) -> &'static str {
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
    pub fn board_render(&mut self, area: Rect, frame: &mut Frame) {
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
                }
                // Draw the cell green if this is the selected cell
                else if i == self.selected_coordinates[0] && j == self.selected_coordinates[1] {
                    let cell = Block::default().bg(Color::LightGreen);
                    frame.render_widget(cell.clone(), square);
                } else {
                    let cell = Block::default().bg(cell_color);
                    frame.render_widget(cell.clone(), square);
                }

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

    pub fn historic_render(&mut self, area: Rect, frame: &mut Frame) {
        // We write the historic board on the side
        let historic_block = Block::default()
            .title("Historic")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(WHITE))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(5, 10, 1, 2));

        let mut lines: Vec<Line> = vec![];

        for i in (0..self.moves_historic.len()).step_by(2) {
            let piece_type_from = self.moves_historic[i].0.clone();
            let utf_icon_white = self.piece_type_to_utf_enum(piece_type_from);
            let number_move = self.moves_historic[i].1.clone();
            let move_white = convert_position_into_notation(number_move);

            let mut utf_icon_black = "   ";
            let mut move_black: String = "   ".to_string();

            // If there is something for black
            if i + 1 < self.moves_historic.len() {
                let piece_type_to = self.moves_historic[i + 1].0.clone();
                let number = self.moves_historic[i + 1].1.clone();
                move_black =
                    convert_position_into_notation(number.to_string().parse::<i32>().unwrap());
                utf_icon_black = self.piece_type_to_utf_enum(piece_type_to)
            }

            lines.push(Line::from(vec![
                Span::raw(format!("{}.  ", i / 2 + 1)), // line number
                Span::styled(format!("{} ", utf_icon_white), Style::default().fg(WHITE)), // white symbol
                Span::raw(format!("{}", move_white)), // white move
                Span::raw("     "),                   // separator
                Span::styled(
                    format!("{} ", utf_icon_black),
                    Style::default().fg(Color::Black),
                ), // white symbol
                Span::raw(format!("{}", move_black)), // black move
            ]));
        }

        let historic_paragraph = Paragraph::new(lines).alignment(Alignment::Center);

        let height = area.height;

        let right_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(height - 1), Constraint::Length(1)].as_ref())
            .split(area);

        frame.render_widget(historic_block.clone(), right_panel_layout[0]);
        frame.render_widget(
            historic_paragraph,
            historic_block.inner(right_panel_layout[0]),
        );

        // Bottom paragraph help text
        let text = vec![Line::from("Press h for help").alignment(Alignment::Center)];

        let help_paragraph = Paragraph::new(text)
            .block(Block::new())
            .alignment(Alignment::Center);
        frame.render_widget(help_paragraph, right_panel_layout[1]);
    }
}
