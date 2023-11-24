use ratatui::{layout::{Constraint, Layout, Direction, Rect, Alignment}, Frame, style::{Color, Stylize, Modifier}, widgets::{Block, Paragraph}};
use crate::{constants::UNDEFINED_POSITION, utils::{get_piece_color, get_piece_type}, pieces::{pawn::Pawn, rook::Rook, bishop::Bishop, queen::Queen, king::King, knight::Knight, PieceType, PieceColor}};

#[derive(Debug)]
pub struct Board {
    pub board: [[Option<(PieceType, PieceColor)>; 8]; 8],    
    pub cursor_coordinates: [i32; 2],
    pub selected_coordinates: [i32; 2],
    pub selected_piece_cursor: i32,
    pub old_cursor_position: [i32; 2]

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
                [ None, None, None, None, None, None, None, None],
                [ None, None, None, None, None, None, None, None],
                [ None, None, None, None, None, None, None, None],
                [ None, None, None, None, None, None, None, None],
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
        }
    }
}

impl Board {
    // Setters
    pub fn set_board(&mut self, board: [[Option<(PieceType, PieceColor)>; 8]; 8]){
        self.board = board;
    }

    // Getters
    pub fn authorized_positions_enum(selected_coordinates: [i32; 2], piece_type: PieceType, color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
        match piece_type {
            PieceType::Pawn => Pawn::authorized_positions(selected_coordinates, color, board),
            PieceType::Rook => Rook::authorized_positions(selected_coordinates, color, board),
            PieceType::Bishop => Bishop::authorized_positions(selected_coordinates, color, board),
            PieceType::Queen => Queen::authorized_positions(selected_coordinates, color, board),
            PieceType::King => King::authorized_positions(selected_coordinates, color, board),
            PieceType::Knight => Knight::authorized_positions(selected_coordinates, color, board),
        }
    }

    pub fn protected_positions_enum(selected_coordinates: [i32; 2], piece_type: PieceType, color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
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
        self.selected_coordinates[0] != UNDEFINED_POSITION && self.selected_coordinates[1] != UNDEFINED_POSITION
    }

    fn get_authorized_positions(&mut self, piece_type: Option<PieceType>, piece_color: Option<PieceColor>, coordinates: [i32; 2]) -> Vec<Vec<i32>>{
        return match (piece_type, piece_color) {
            (Some(piece_type), Some(piece_color)) => {
                Board::authorized_positions_enum(coordinates, piece_type, piece_color, self.board)
            },
            _ => { Vec::new()}
        };
    }

    // Methods to change the position of the cursor
    pub fn cursor_up(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1)
        }else{
            if self.cursor_coordinates[0] > 0 {
                self.cursor_coordinates[0] -= 1 
            }
        }
    }
    pub fn cursor_down(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false,  1)
        }else{
            if self.cursor_coordinates[0] < 7 {
                self.cursor_coordinates[0] += 1 
            }
        }
    }
    pub fn cursor_left(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, -1)
        }else{
            if self.cursor_coordinates[1] > 0 {
                self.cursor_coordinates[1] -= 1 
            }
        }
    }
    pub fn cursor_right(&mut self) {
        if self.is_cell_selected() {
            self.move_selected_piece_cursor(false, 1)
        }else{
            if self.cursor_coordinates[1] < 7 {
                self.cursor_coordinates[1] += 1 
            }
        }
    }

    fn move_selected_piece_cursor(&mut self, first_time_moving: bool, direction: i32) {
        let piece_color = get_piece_color(self.board.clone(), self.selected_coordinates);
        let piece_type = get_piece_type(self.board.clone(), self.selected_coordinates);
    
        let mut authorized_positions = self.get_authorized_positions(piece_type, piece_color, self.selected_coordinates);
    
        if !authorized_positions.is_empty() {
            self.selected_piece_cursor = if self.selected_piece_cursor == 0 && first_time_moving {
                0
            } else {
                let new_cursor = (self.selected_piece_cursor + direction) % authorized_positions.len() as i32;
                if new_cursor == -1 {
                    authorized_positions.len() as i32 - 1
                } else {
                    new_cursor
                }
            };
    
            authorized_positions.sort();
    
            if let Some(position) = authorized_positions.get(self.selected_piece_cursor as usize) {
                self.cursor_coordinates = [position[0], position[1]];
            }
        }
    }
    

    // Methods to select a cell on the board
    pub fn select_cell(&mut self){
        if !self.is_cell_selected(){
            self.selected_coordinates = self.cursor_coordinates;
            self.old_cursor_position = self.cursor_coordinates;
            self.move_selected_piece_cursor(true, 1);
        }else{
            let selected_coords_usize: [usize; 2] = [self.selected_coordinates[0] as usize, self.selected_coordinates[1] as usize];
            let cursor_coords_usize: [usize; 2] = [self.cursor_coordinates[0] as usize, self.cursor_coordinates[1] as usize];
            self.move_piece_on_the_board(selected_coords_usize, cursor_coords_usize);
            self.unselect_cell();
        }
    }

    pub fn move_piece_on_the_board(&mut self, from: [usize; 2], to: [usize; 2]) {
        self.board[to[0]][to[1]] = self.board[from[0]][from[1]];
        self.board[from[0]][from[1]] = None;
    }
    

    pub fn unselect_cell(&mut self){
        self.selected_coordinates[0] = UNDEFINED_POSITION;
        self.selected_coordinates[1] = UNDEFINED_POSITION;
        self.selected_piece_cursor = 0;
        self.cursor_coordinates = self.old_cursor_position
    }

    pub fn color_to_ratatui_enum(&mut self, piece_color: Option<PieceColor>) -> Color{
        return match piece_color {
            Some(PieceColor::Black) => Color::Black,
            Some(PieceColor::White) => Color::White,
            None => Color::Red,
        };
    }
    pub fn piece_type_to_string_enum(&mut self, piece_type: Option<PieceType>) -> &'static str{
        return match piece_type {
            Some(PieceType::Queen) => Queen::to_string(),
            Some(PieceType::King) => King::to_string(),
            Some(PieceType::Rook) => Rook::to_string(),
            Some(PieceType::Bishop) => Bishop::to_string(),
            Some(PieceType::Knight) => Knight::to_string(),
            Some(PieceType::Pawn) => Pawn::to_string(),
            None => " ",
        };
    }

    // Method to render the board
    pub fn board_render(&mut self, area: Rect, frame: &mut Frame) {
        // We have 8 vertical lines
        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    ]
                .as_ref(),
            )
            .split(area);
    
        // For each line we set 8 layout
        for i in 0..8i32{
            let lines = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
                    Constraint::Ratio(1, 8),
        
                ]
                .as_ref(),
            ).split(columns[i as usize]);
            for j in 0..8i32{
                // Color of the cell to draw the board
                let mut cell_color: Color = if (i + j) % 2 == 0 {
                    Color::Rgb(210, 200, 190)
                } else {
                    Color::Rgb(128, 95, 69)
                };

                // Draw the available moves for the selected piece
                if self.is_cell_selected() {
                    let selected_piece_type = get_piece_type(self.board, self.selected_coordinates);
                    let selected_piece_color:Option<PieceColor> = get_piece_color(self.board, self.selected_coordinates);
                    let positions = self.get_authorized_positions(selected_piece_type, selected_piece_color, self.selected_coordinates);

                    for coords in positions.clone(){
                        if i == coords[0] && j == coords[1]{
                            cell_color = Color::LightRed
                        }
                    }
                }


                // Draw the cell blue if this is the current cursor cell
                if i == self.cursor_coordinates[0] && j == self.cursor_coordinates[1]{
                    let cell = Block::default().bg(Color::LightBlue).add_modifier(Modifier::RAPID_BLINK);
                    frame.render_widget(cell.clone(),lines[j as usize]); 
                }
                // Draw the cell green if this is the selected cell
                else if i == self.selected_coordinates[0] && j == self.selected_coordinates[1]{
                    let cell = Block::default().bg(Color::LightGreen);
                    frame.render_widget(cell.clone(),lines[j as usize]); 
                } 
                else{
                    let cell = Block::default().bg(cell_color);
                    frame.render_widget(cell.clone(),lines[j as usize]);
                }

                // Get piece and color
                let piece_color = get_piece_color(self.board, [i, j]);
                let piece_type =get_piece_type(self.board, [i, j]);

                let color_enum = self.color_to_ratatui_enum(piece_color);
                let piece_enum = self.piece_type_to_string_enum(piece_type);

                // Place the pieces on the board
                let paragraph = Paragraph::new(piece_enum).alignment(Alignment::Center).fg(color_enum);
                frame.render_widget(paragraph,lines[j as usize]);
            }
        }
    }
}