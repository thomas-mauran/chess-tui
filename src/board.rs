use ratatui::{layout::{Constraint, Layout, Direction, Rect, Alignment}, Frame, style::{Color, Stylize, Modifier}, widgets::{Block, Paragraph}};
use crate::{constants::UNDEFINED_POSITION, utils::get_piece_color, pieces::{pawn::Pawn, rook::Rook, bishop::Bishop, queen::Queen, king::King, knight::Knight, PieceType, PieceColor}};

#[derive(Debug)]
pub struct Board {
    pub board: [[Option<(PieceType, PieceColor)>; 8]; 8],    
    pub cursor_y: i32,
    pub cursor_x: i32,
    pub selected_coordinates: [i32; 2]

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
                [ Some((PieceType::Pawn, PieceColor::White)), None, None, None, None, None, None, Some((PieceType::Bishop, PieceColor::White))],
                [ None, None, None, Some((PieceType::Pawn, PieceColor::White)), Some((PieceType::King, PieceColor::White)), Some((PieceType::Pawn, PieceColor::Black)), None, None],
                [ None, None, None, None, None, None, None, None],
                [ Some((PieceType::Pawn, PieceColor::White)), None, None, Some((PieceType::Rook, PieceColor::Black)), None, None, None, None],
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
            cursor_y: 4,
            cursor_x: 4,
            selected_coordinates: [UNDEFINED_POSITION, UNDEFINED_POSITION]
        }
    }
}

impl Board {
    // Setters
    pub fn set_board(&mut self, board: [[Option<(PieceType, PieceColor)>; 8]; 8]){
        self.board = board;
    }

    // Getters
    pub fn get_piece_type(&mut self, coordinates: [i32; 2]) -> Option<PieceType> {
        match self.board[coordinates[0] as usize][coordinates[1] as usize] {
            Some((piece_type, _)) => Some(piece_type),
            None => None,
        }
    }

    pub fn authorized_positions_enum(&mut self, piece: PieceType, color: PieceColor) -> Vec<Vec<i32>>{
        match piece {
            PieceType::Pawn => Pawn::authorized_positions(self.selected_coordinates, color, self.board),
            PieceType::Rook => Rook::authorized_positions(self.selected_coordinates, color, self.board),
            PieceType::Bishop => Bishop::authorized_positions(self.selected_coordinates, color, self.board),
            PieceType::Queen => Queen::authorized_positions(self.selected_coordinates, color, self.board),
            PieceType::King => King::authorized_positions(self.selected_coordinates, color, self.board),
            PieceType::Knight => Knight::authorized_positions(self.selected_coordinates, color, self.board),
        }
    }
    // Check if a cell has been selected
    fn is_cell_selected(&mut self) -> bool {
        self.selected_coordinates[0] != UNDEFINED_POSITION && self.selected_coordinates[1] != UNDEFINED_POSITION
    }

    // Methods to change the position of the cursor
    pub fn cursor_up(&mut self) {
        if self.cursor_y > 0 && !self.is_cell_selected() {self.cursor_y -= 1}
    }
    pub fn cursor_down(&mut self) {
        if self.cursor_y < 7 && !self.is_cell_selected() {self.cursor_y += 1}
    }
    pub fn cursor_left(&mut self) {
        if self.cursor_x > 0 && !self.is_cell_selected() {self.cursor_x -= 1}
    }
    pub fn cursor_right(&mut self) {
        if self.cursor_x < 7 && !self.is_cell_selected() {self.cursor_x += 1}
    }

    // Methods to select a cell on the board
    pub fn select_cell(&mut self){
        if !self.is_cell_selected(){
            self.selected_coordinates = [self.cursor_y, self.cursor_x]
        }
    }

    pub fn unselect_cell(&mut self){
        self.selected_coordinates[0] = UNDEFINED_POSITION;
        self.selected_coordinates[1] = UNDEFINED_POSITION;
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
                    let selected_piece_type:Option<PieceType> = self.get_piece_type(self.selected_coordinates);
                    let selected_piece_color:Option<PieceColor> = get_piece_color(self.board, self.selected_coordinates);
                    let positions = match (selected_piece_type, selected_piece_color) {
                        (Some(piece_type), Some(piece_color)) => {
                            self.authorized_positions_enum(piece_type, piece_color)
                        },
                        _ => { Vec::new()}
                    };
                    

                    for coords in positions.clone(){
                        if i == coords[0] && j == coords[1]{
                            cell_color = Color::LightRed
                        }
                    }
                }


                // Draw the cell blue if this is the current cursor cell
                if (i == self.cursor_y && j == self.cursor_x) && !self.is_cell_selected(){
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
                let piece_type =self.get_piece_type([i, j]);

                let color_enum = match piece_color {
                    Some(PieceColor::Black) => Color::Black,
                    Some(PieceColor::White) => Color::White,
                    None => Color::Red,

                };

                let piece_enum = match piece_type {
                    Some(PieceType::Queen) => Queen::to_string(),
                    Some(PieceType::King) => King::to_string(),
                    Some(PieceType::Rook) => Rook::to_string(),
                    Some(PieceType::Bishop) => Bishop::to_string(),
                    Some(PieceType::Knight) => Knight::to_string(),
                    Some(PieceType::Pawn) => Pawn::to_string(),
                    None => " ",
                };

                // Place the pieces on the board
                let paragraph = Paragraph::new(piece_enum).alignment(Alignment::Center).fg(color_enum);
                frame.render_widget(paragraph,lines[j as usize]);
            }
        }
    }
}