use crate::game_logic::coord::Coord;
use ratatui::style::Color;
use shakmaty::Square;

pub fn color_to_ratatui_enum(piece_color: Option<shakmaty::Color>) -> Color {
    match piece_color {
        Some(shakmaty::Color::Black) => Color::Black,
        Some(shakmaty::Color::White) => Color::White,
        None => Color::Red,
    }
}

pub fn flip_square_if_needed(square: Square, is_flipped: bool) -> Square {
    if is_flipped {
        Coord::from_square(square)
            .reverse()
            .to_square()
            .unwrap_or(square)
    } else {
        square
    }
}

pub fn get_square_from_coord(coord: Coord, is_flipped: bool) -> Option<Square> {
    if is_flipped {
        coord.reverse().to_square()
    } else {
        coord.to_square()
    }
}

pub fn get_coord_from_square(square: Option<Square>, is_flipped: bool) -> Coord {
    if let Some(s) = square {
        if is_flipped {
            Coord::from_square(s).reverse()
        } else {
            Coord::from_square(s)
        }
    } else {
        Coord::undefined()
    }
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

pub fn get_opposite_square(square: Option<Square>) -> Option<Square> {
    square.and_then(|s| Coord::from_square(s).reverse().to_square())
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
