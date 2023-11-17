use ratatui::{layout::{Constraint, Layout, Direction, Rect, Alignment}, Frame, style::{Color, Stylize, Modifier}, widgets::{Block, Paragraph}};
use crate::pieces::{K, Q, R, N, P, B};

#[derive(Debug)]
pub struct Board {
    pub board: [[&'static str; 8]; 8],    
    pub cursor_x: usize,
    pub cursor_y: usize

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
            cursor_y: 4
        }
    }
}

impl Board {

    pub fn cursor_up(&mut self) {
        if self.cursor_x > 0 {self.cursor_x -= 1}
    }
    pub fn cursor_down(&mut self) {
        if self.cursor_x < 7 {self.cursor_x += 1}
    }
    pub fn cursor_left(&mut self) {
        if self.cursor_y > 0 {self.cursor_y -= 1}
    }
    pub fn cursor_right(&mut self) {
        if self.cursor_y < 7 {self.cursor_y += 1}
    }

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

                if i == self.cursor_x && j == self.cursor_y{
                    let cell = Block::default().bg(Color::LightBlue).add_modifier(Modifier::RAPID_BLINK);
                    frame.render_widget(cell.clone(),lines[j]); 
                }else{
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
                let paragraph = Paragraph::new(format!("{}", piece_enum)).alignment(Alignment::Center).fg(color_enum);
                frame.render_widget(paragraph,lines[j]);

                
            }
        }
    }
}