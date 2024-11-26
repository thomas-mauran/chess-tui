use crate::game_logic::coord::Coord;
use crate::game_logic::game::Game;
use crate::game_logic::game_board::GameBoard;
use crate::{
    constants::{DisplayMode, UNDEFINED_POSITION},
    pieces::{PieceColor, PieceType},
};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph},
};

/// method to clean `positions`: remove impossible positions
pub fn cleaned_positions(positions: &[Coord]) -> Vec<Coord> {
    positions
        .iter()
        .filter(|position| position.is_valid())
        .copied()
        .collect()
}

/// Return true for ally cell color; false for enemy
pub fn is_cell_color_ally(game_board: &GameBoard, coordinates: &Coord, color: PieceColor) -> bool {
    match game_board.get_piece_color(coordinates) {
        Some(cell_color) => cell_color == color,
        None => false, // Treat empty cell as ally
    }
}

pub fn col_to_letter(col: u8) -> String {
    match col {
        0 => "a".to_string(),
        1 => "b".to_string(),
        2 => "c".to_string(),
        3 => "d".to_string(),
        4 => "e".to_string(),
        5 => "f".to_string(),
        6 => "g".to_string(),
        7 => "h".to_string(),
        _ => unreachable!("Col out of bound {}", col),
    }
}

pub fn letter_to_col(col: Option<char>) -> i8 {
    match col {
        Some('a') => 0,
        Some('b') => 1,
        Some('c') => 2,
        Some('d') => 3,
        Some('e') => 4,
        Some('f') => 5,
        Some('g') => 6,
        Some('h') => 7,
        _ => unreachable!("Col out of bound"),
    }
}

pub fn convert_position_into_notation(position: &str) -> String {
    let from_y = get_int_from_char(position.chars().next());
    let from_x = get_int_from_char(position.chars().nth(1));
    let to_y = get_int_from_char(position.chars().nth(2));
    let to_x = get_int_from_char(position.chars().nth(3));

    let from_x = col_to_letter(from_x);
    let to_x = col_to_letter(to_x);

    format!("{from_x}{}-{to_x}{}", 8 - from_y, 8 - to_y)
}

pub fn convert_notation_into_position(notation: &str) -> String {
    let from_x = &letter_to_col(notation.chars().next());
    let from_y = (get_int_from_char(notation.chars().nth(1)) as i8 - 8).abs();

    let to_x = &letter_to_col(notation.chars().nth(2));
    let to_y = (get_int_from_char(notation.chars().nth(3)) as i8 - 8).abs();

    format!("{from_y}{from_x}{to_y}{to_x}")
}

pub fn get_int_from_char(ch: Option<char>) -> u8 {
    match ch {
        Some(ch) => ch.to_digit(10).unwrap() as u8,
        _ => UNDEFINED_POSITION,
    }
}

pub fn is_piece_opposite_king(piece: Option<(PieceType, PieceColor)>, color: PieceColor) -> bool {
    match piece {
        Some((piece_type, piece_color)) => {
            piece_type == PieceType::King && piece_color == color.opposite()
        }
        _ => false,
    }
}

pub fn color_to_ratatui_enum(piece_color: Option<PieceColor>) -> Color {
    match piece_color {
        Some(PieceColor::Black) => Color::Black,
        Some(PieceColor::White) => Color::White,
        None => Color::Red,
    }
}

pub fn get_cell_paragraph<'a>(
    game: &'a Game,
    cell_coordinates: &'a Coord,
    bounding_rect: Rect,
) -> Paragraph<'a> {
    // Get piece and color
    let piece_color = game.game_board.get_piece_color(cell_coordinates);
    let piece_type = game.game_board.get_piece_type(cell_coordinates);
    let piece_enum = PieceType::piece_type_to_string_enum(piece_type, &game.ui.display_mode);

    let paragraph = match game.ui.display_mode {
        DisplayMode::DEFAULT => {
            let color_enum = color_to_ratatui_enum(piece_color);

            // Place the pieces on the board
            Paragraph::new(piece_enum).fg(color_enum)
        }
        DisplayMode::ASCII => {
            // Determine piece letter case
            let paragraph = match piece_color {
                // pieces belonging to the player on top will be lower case
                Some(PieceColor::Black) => Paragraph::new(piece_enum.to_lowercase()),
                // pieces belonging to the player on bottom will be upper case
                Some(PieceColor::White) => Paragraph::new(piece_enum.to_uppercase().underlined()),
                // Pass through original value
                None => Paragraph::new(piece_enum),
            };

            // Place the pieces on the board
            paragraph.block(Block::new().padding(Padding::vertical(bounding_rect.height / 2)))
        }
    };

    paragraph.alignment(Alignment::Center)
}

pub fn invert_position(coord: &Coord) -> Coord {
    Coord::new(7 - coord.row, 7 - coord.col)
}
