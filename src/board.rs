use ratatui::{layout::{Constraint, Layout, Direction, Rect, Alignment}, Frame, style::{Color, Stylize}, widgets::{Block, Paragraph}};
use crate::pieces::{K, Q, R, N, P, B};

#[derive(Debug)]
pub struct Board {
    pub board: [[char; 8]; 8],    
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                ['R', 'N', 'B', 'K', 'Q', 'B', 'N', 'R'],
                ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
                ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
            ]
        }
    }
}

impl Board {
    pub fn hello() {
        print!("Hello");
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
                let color: Color = if (i + j) % 2 == 0 {
                    Color::Rgb(210, 200, 190)
                } else {
                    Color::Rgb(128, 95, 69)
                };
                let cell = Block::default().bg(color);
                frame.render_widget(cell.clone(),lines[j]);
    
                let current_piece = match self.board[i][j] {
                    'Q' => Q,
                    'K' => K,
                    'R' => R,
                    'B' => B,
                    'N' => N,
                    'P' => P,
                    _ => " ",
                };
    
                let paragraph = Paragraph::new(format!("{}", current_piece)).alignment(Alignment::Center).fg(Color::Black);
                frame.render_widget(paragraph,lines[j]);
            }
        }
    }
}