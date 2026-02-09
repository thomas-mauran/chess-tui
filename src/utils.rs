use crate::game_logic::coord::Coord;
use ratatui::style::Color;
use shakmaty::Square;

#[must_use]
pub fn color_to_ratatui_enum(piece_color: Option<shakmaty::Color>) -> Color {
    match piece_color {
        Some(shakmaty::Color::Black) => Color::Black,
        Some(shakmaty::Color::White) => Color::White,
        None => Color::Red,
    }
}

#[must_use]
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

#[must_use]
pub fn get_square_from_coord(coord: Coord, is_flipped: bool) -> Option<Square> {
    if is_flipped {
        coord.reverse().to_square()
    } else {
        coord.to_square()
    }
}

#[must_use]
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
#[must_use]
pub fn get_int_from_char(c: Option<char>) -> u8 {
    match c {
        Some('b' | '1') => 1,
        Some('c' | '2') => 2,
        Some('d' | '3') => 3,
        Some('e' | '4') => 4,
        Some('f' | '5') => 5,
        Some('g' | '6') => 6,
        Some('h' | '7') => 7,
        _ => 0,
    }
}

#[must_use]
pub fn get_opposite_square(square: Option<Square>) -> Option<Square> {
    square.and_then(|s| Coord::from_square(s).reverse().to_square())
}

/// Convert position format ("4644") to UCI notation (e.g., "e4e4")
#[must_use]
pub fn convert_position_into_notation(position: &str) -> String {
    let chars: Vec<char> = position.chars().collect();
    if chars.len() < 4 {
        return String::new();
    }

    // Safe conversion: to_digit returns values 0-9, which fits in u8
    let from_row = chars[0].to_digit(10).unwrap_or(0).min(255) as u8;
    let from_col = chars[1].to_digit(10).unwrap_or(0).min(255) as u8;
    let to_row = chars[2].to_digit(10).unwrap_or(0).min(255) as u8;
    let to_col = chars[3].to_digit(10).unwrap_or(0).min(255) as u8;

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

// normalize lower case sans into valid uppercase san
#[must_use]
pub fn normalize_lowercase_to_san(input: &str) -> String {
    let s0 = input.trim();

    if s0.is_empty() {
        return String::new();
    }

    // ---- Castling normalization (accept common variants) ----
    let lower = s0.to_ascii_lowercase();
    match lower.as_str() {
        "o-o" | "0-0" => return "O-O".to_string(),
        "o-o-o" | "0-0-0" => return "O-O-O".to_string(),
        _ => {}
    }

    let mut s = s0.to_string();

    // ---- Uppercase leading piece letter (only first char) ----
    if let Some(first) = s.chars().next() {
        let up = match first {
            'n' => Some('N'),
            'b' => Some('B'),
            'r' => Some('R'),
            'q' => Some('Q'),
            'k' => Some('K'),
            _ => None, // pawn moves like "e4" should stay lowercase
        };

        if let Some(up) = up {
            s.replace_range(0..first.len_utf8(), &up.to_string());
        }
    }

    // ---- Uppercase promotion piece after '=' (e8=q -> e8=Q) ----
    if let Some(eq) = s.find('=') {
        // SAN promotion piece is a single ASCII letter right after '='
        let bytes = s.as_bytes();
        if eq + 1 < bytes.len() {
            let promo = bytes[eq + 1] as char;
            let up = match promo {
                'q' => Some('Q'),
                'r' => Some('R'),
                'b' => Some('B'),
                'n' => Some('N'),
                _ => None,
            };
            if let Some(up) = up {
                s.replace_range(eq + 1..eq + 2, &up.to_string());
            }
        }
    }

    s
}
