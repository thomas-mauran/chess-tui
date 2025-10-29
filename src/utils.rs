use crate::game_logic::coord::Coord;
use crate::game_logic::game::Game;
use crate::{constants::DisplayMode, pieces::role_to_utf_enum};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph},
};
use shakmaty::Role;

pub fn color_to_ratatui_enum(piece_color: Option<shakmaty::Color>) -> Color {
    match piece_color {
        Some(shakmaty::Color::Black) => Color::Black,
        Some(shakmaty::Color::White) => Color::White,
        None => Color::Red,
    }
}

pub fn get_cell_paragraph<'a>(
    game: &'a mut Game,
    cell_coordinates: &'a Coord,
    bounding_rect: Rect,
) -> Paragraph<'a> {
    use crate::pieces::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
    };

    // Get piece and color
    let piece_color = game
        .game_board
        .get_piece_color_at_square(&cell_coordinates.to_square().unwrap())
        .map(|c| c.into())
        .unwrap_or(None);
    let piece_type = game
        .game_board
        .get_role_at_square(&cell_coordinates.to_square().unwrap())
        .map(|r| r.into())
        .unwrap_or(None);

    let paragraph = match game.ui.display_mode {
        DisplayMode::DEFAULT => {
            let color_enum = color_to_ratatui_enum(piece_color);

            // Use custom multi-line designs for DEFAULT mode
            let piece_str = match piece_type {
                Some(Role::King) => King::to_string(&game.ui.display_mode),
                Some(Role::Queen) => Queen::to_string(&game.ui.display_mode),
                Some(Role::Rook) => Rook::to_string(&game.ui.display_mode),
                Some(Role::Bishop) => Bishop::to_string(&game.ui.display_mode),
                Some(Role::Knight) => Knight::to_string(&game.ui.display_mode),
                Some(Role::Pawn) => Pawn::to_string(&game.ui.display_mode),
                None => " ",
            };

            // Place the pieces on the board
            Paragraph::new(piece_str)
                .fg(color_enum)
                .alignment(Alignment::Center)
        }
        DisplayMode::ASCII => {
            let piece_enum = role_to_utf_enum(&piece_type.unwrap(), piece_color);
            // Determine piece letter case
            let paragraph = match piece_color {
                // pieces belonging to the player on top will be lower case
                Some(shakmaty::Color::Black) => Paragraph::new(piece_enum.to_lowercase()),
                // pieces belonging to the player on bottom will be upper case
                Some(shakmaty::Color::White) => {
                    Paragraph::new(piece_enum.to_uppercase().underlined())
                }
                // Pass through original value
                None => Paragraph::new(piece_enum),
            };

            // Place the pieces on the board
            paragraph
                .block(Block::new().padding(Padding::vertical(bounding_rect.height / 2)))
                .alignment(Alignment::Center)
        }
    };

    paragraph
}

/// Convert a character to an integer for parsing UCI moves
pub fn get_int_from_char(c: Option<char>) -> u8 {
    match c {
        Some('a') | Some('0') => 0,
        Some('b') | Some('1') => 1,
        Some('c') | Some('2') => 2,
        Some('d') | Some('3') => 3,
        Some('e') | Some('4') => 4,
        Some('f') | Some('5') => 5,
        Some('g') | Some('6') => 6,
        Some('h') | Some('7') => 7,
        _ => 0,
    }
}

/// Convert UCI notation (e.g., "e2e4") to position format ("4644")
pub fn convert_notation_into_position(notation: &str) -> String {
    let chars: Vec<char> = notation.chars().collect();
    if chars.len() < 4 {
        return String::new();
    }

    let from_file = get_int_from_char(Some(chars[0]));
    let from_rank = get_int_from_char(Some(chars[1]));
    let to_file = get_int_from_char(Some(chars[2]));
    let to_rank = get_int_from_char(Some(chars[3]));

    // Convert from chess notation (a1-h8) to our internal format (0-7, 0-7)
    // Row is inverted: rank 1 = row 7, rank 8 = row 0
    let from_row = 7 - from_rank;
    let to_row = 7 - to_rank;

    let mut result = format!("{}{}{}{}", from_row, from_file, to_row, to_file);

    // If there's a promotion piece, append it
    if chars.len() >= 5 {
        result.push(chars[4]);
    }

    result
}

/// Convert position format ("4644") to UCI notation (e.g., "e4e4")
pub fn convert_position_into_notation(position: &str) -> String {
    let chars: Vec<char> = position.chars().collect();
    if chars.len() < 4 {
        return String::new();
    }

    let from_row = chars[0].to_digit(10).unwrap_or(0) as u8;
    let from_col = chars[1].to_digit(10).unwrap_or(0) as u8;
    let to_row = chars[2].to_digit(10).unwrap_or(0) as u8;
    let to_col = chars[3].to_digit(10).unwrap_or(0) as u8;

    // Convert from our internal format to chess notation
    // Row is inverted: row 0 = rank 8, row 7 = rank 1
    let from_rank = 7 - from_row;
    let to_rank = 7 - to_row;

    let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];

    format!(
        "{}{}{}{}",
        files[from_col as usize],
        ranks[from_rank as usize],
        files[to_col as usize],
        ranks[to_rank as usize]
    )
}

/// Invert a position (for flipping the board perspective)
pub fn invert_position(coord: &Coord) -> Coord {
    Coord::new(7 - coord.row, 7 - coord.col)
}
