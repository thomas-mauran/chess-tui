use ratatui::{layout::{Constraint, Layout, Direction, Rect, Alignment}, Frame, style::{Color, Stylize, Modifier}, widgets::{Block, Paragraph}};
use crate::{pieces::{K, Q, R, N, P, B}, constants::UNDEFINED_POSITION};

#[derive(Debug)]
pub struct Board {
    pub board: [[&'static str; 8]; 8],    
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub selected_coordinates: [i32; 2]

}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                ["bR", "bN", "bB", "bQ", "bK", "bB", "bN", "bR"],
                ["bP", "bP", "bP", "bP", "bP", "bP", "bP", "bP"],
                ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
                ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
                ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
                ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
                ["wP", "wP", "wP", "wP", "wP", "wP", "wP", "wP"],
                ["wR", "wN", "wB", "wQ", "wK", "wB", "wN", "wR"],
            ],
            cursor_x: 4,
            cursor_y: 4,
            selected_coordinates: [UNDEFINED_POSITION, UNDEFINED_POSITION]
        }
    }
}

impl Board {
    // Check if a cell has been selected
    pub fn is_cell_selected(&mut self) -> bool {
        self.selected_coordinates[0] != UNDEFINED_POSITION && self.selected_coordinates[1] != UNDEFINED_POSITION
    }

    // Methods to change the position of the cursor
    pub fn cursor_up(&mut self) {
        if self.cursor_x > 0 && !self.is_cell_selected() {self.cursor_x -= 1}
    }
    pub fn cursor_down(&mut self) {
        if self.cursor_x < 7 && !self.is_cell_selected() {self.cursor_x += 1}
    }
    pub fn cursor_left(&mut self) {
        if self.cursor_y > 0 && !self.is_cell_selected() {self.cursor_y -= 1}
    }
    pub fn cursor_right(&mut self) {
        if self.cursor_y < 7 && !self.is_cell_selected() {self.cursor_y += 1}
    }

    // Methods to select a cell on the board
    pub fn select_cell(&mut self){
        if !self.is_cell_selected() {
            self.selected_coordinates = [self.cursor_x, self.cursor_y]
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
        for i in 0..8{
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
            ).split(columns[i]);
            for j in 0..8{
                // Color of the cell to draw the board
                let cell_color: Color = if (i + j) % 2 == 0 {
                    Color::Rgb(210, 200, 190)
                } else {
                    Color::Rgb(128, 95, 69)
                };
                // Draw the cell blue if this is the current cursor cell
                if (i as i32 == self.cursor_x && j as i32 == self.cursor_y) && !self.is_cell_selected(){
                    let cell = Block::default().bg(Color::LightBlue).add_modifier(Modifier::RAPID_BLINK);
                    frame.render_widget(cell.clone(),lines[j]); 
                }
                // Draw the cell green if this is the selected cell
                else if i as i32 == self.selected_coordinates[0] && j as i32 == self.selected_coordinates[1]{
                    let cell = Block::default().bg(Color::LightGreen);
                    frame.render_widget(cell.clone(),lines[j]); 
                } 
                else{
                    let cell = Block::default().bg(cell_color);
                    frame.render_widget(cell.clone(),lines[j]);
                }
                let piece_color = &self.board[i][j][0..1];
                let piece_type = &self.board[i][j][1..2];

                let color_enum = match piece_color {
                    "b" => Color::Black,
                    "w" => Color::White,
                    _ => Color::Red
                };

                let piece_enum = match piece_type {
                    "Q" => Q,
                    "K" => K,
                    "R" => R,
                    "B" => B,
                    "N" => N,
                    "P" => P,
                    _ => "",
                };

                // Place the pieces on the board
                let paragraph = Paragraph::new(piece_enum).alignment(Alignment::Center).fg(color_enum);
                frame.render_widget(paragraph,lines[j]);

                
            }
        }
    }
}