use crate::{
    board::Board,
    constants::{DisplayMode, Players, UNDEFINED_POSITION},
    pieces::{PieceColor, PieceMove, PieceType},
};
use crossterm::style::Print;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph},
};

pub fn get_piece_color(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    coordinates: [i8; 2],
) -> Option<PieceColor> {
    board[coordinates[0] as usize][coordinates[1] as usize].map(|(_, piece_color)| piece_color)
}

pub fn get_piece_type(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    coordinates: [i8; 2],
) -> Option<PieceType> {
    board[coordinates[0] as usize][coordinates[1] as usize].map(|(piece_type, _)| piece_type)
}

pub fn get_opposite_color(color: PieceColor) -> PieceColor {
    match color {
        PieceColor::Black => PieceColor::White,
        PieceColor::White => PieceColor::Black,
    }
}

// method to clean the position array to remove impossible positions
pub fn cleaned_positions(positions: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut cleaned_array: Vec<Vec<i8>> = vec![];
    for position in positions {
        if is_valid([position[0], position[1]]) {
            cleaned_array.push(position);
        }
    }
    cleaned_array
}

// Return true forally cell color; false for enemy
pub fn is_cell_color_ally(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    coordinates: [i8; 2],
    color: PieceColor,
) -> bool {
    match get_piece_color(board, coordinates) {
        Some(cell_color) => cell_color == color,
        None => false, // Treat empty cell as ally
    }
}

pub fn is_valid(coordinates: [i8; 2]) -> bool {
    let (y, x) = (coordinates[0], coordinates[1]);

    (0..8).contains(&y) && (0..8).contains(&x)
}

pub fn is_vec_in_array(array: Vec<Vec<i8>>, element: [i8; 2]) -> bool {
    for position in array {
        if position == element {
            return true;
        }
    }
    false
}

// We get all the cells that are getting put in 'check'
pub fn get_all_protected_cells(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    turn: Players,
    player_color: PieceColor,
    move_history: &[PieceMove],
) -> Vec<Vec<i8>> {
    let mut check_cells: Vec<Vec<i8>> = vec![];
    for i in 0..8i8 {
        for j in 0..8i8 {
            // Check if the cell is the same color as the player
            if get_piece_color(board, [i, j]) == Some(player_color) {
                continue;
            }
            // get the current cell piece color and type protecting positions
            if let Some(piece_color) = get_piece_color(board, [i, j]) {
                if let Some(piece_type) = get_piece_type(board, [i, j]) {
                    check_cells.extend(PieceType::protected_positions(
                        [i, j],
                        piece_type,
                        turn,
                        piece_color,
                        board,
                        move_history,
                    ));
                }
            }
        }
    }
    check_cells
}

pub fn col_to_letter(col: i8) -> String {
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

pub fn convert_position_into_notation(position: String) -> String {
    let mut result: String = "".to_string();
    let from_y = get_int_from_char(position.chars().next());
    let from_x = get_int_from_char(position.chars().nth(1));
    let to_y = get_int_from_char(position.chars().nth(2));
    let to_x = get_int_from_char(position.chars().nth(3));

    result += &col_to_letter(from_x);
    result += &format!("{}", 8 - from_y).to_string();
    result += "-";
    result += &col_to_letter(to_x);
    result += &format!("{}", 8 - to_y).to_string();

    result
}

pub fn convert_notation_into_position(notation: String) -> String {
    let from_x = &letter_to_col(notation.chars().next());
    let from_y = (&get_int_from_char(notation.chars().nth(1)) - 8).abs();

    let to_x = &letter_to_col(notation.chars().nth(2));
    let to_y = (&get_int_from_char(notation.chars().nth(3)) - 8).abs();

    format!("{}{}{}{}", from_y, from_x, to_y, to_x)
}

pub fn get_player_turn_in_modulo(player_turn: Players) -> usize {
    match player_turn {
        Players::Local => 0,
        Players::Enemy => 1,
    }
}

pub fn get_int_from_char(ch: Option<char>) -> i8 {
    match ch {
        Some(ch) => ch.to_digit(10).unwrap() as i8,
        _ => -1,
    }
}

pub fn get_latest_move(move_history: &[PieceMove]) -> Option<PieceMove> {
    if !move_history.is_empty() {
        return Some(move_history[move_history.len() - 1]);
    }
    None
}

pub fn did_piece_already_move(
    move_history: &[PieceMove],
    original_piece: (Option<PieceType>, [i8; 2]),
) -> bool {
    for entry in move_history {
        if Some(entry.piece_type) == original_piece.0
            && [entry.from_y, entry.from_x] == original_piece.1
        {
            return true;
        }
    }
    false
}

// Method returning the coordinates of the king of a certain color
pub fn get_king_coordinates(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    current_color: PieceColor,
) -> [i8; 2] {
    for i in 0..8i32 {
        for j in 0..8i32 {
            if let Some((piece_type, piece_color)) = board[i as usize][j as usize] {
                if piece_type == PieceType::King && piece_color == current_color {
                    return [i as i8, j as i8];
                }
            }
        }
    }
    [UNDEFINED_POSITION, UNDEFINED_POSITION]
}

// Is getting checked
pub fn is_getting_checked(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    turn: Players,
    current_color: PieceColor,
    move_history: &[PieceMove],
) -> bool {
    let coordinates = get_king_coordinates(board, current_color);

    let checked_cells = get_all_protected_cells(board, turn, current_color, move_history);

    for position in checked_cells {
        if position == coordinates {
            return true;
        }
    }
    false
}

pub fn impossible_positions_king_checked(
    original_coordinates: [i8; 2],
    positions: Vec<Vec<i8>>,
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    turn: Players,
    color: PieceColor,
    move_history: &[PieceMove],
) -> Vec<Vec<i8>> {
    let mut cleaned_position: Vec<Vec<i8>> = vec![];
    for position in positions {
        // We create a new board
        let mut new_board = Board::new(board, color, move_history.to_owned().clone());

        // We simulate the move
        Board::move_piece_on_the_board(
            &mut new_board,
            [
                original_coordinates[0] as usize,
                original_coordinates[1] as usize,
            ],
            [position[0] as usize, position[1] as usize],
        );

        // We check if the board is still checked with this move meaning it didn't resolve the problem
        if !is_getting_checked(
            new_board.board,
            turn,
            new_board.player_color,
            &new_board.move_history,
        ) {
            cleaned_position.push(position)
        };
    }
    cleaned_position
}

pub fn is_piece_opposite_king(piece: Option<(PieceType, PieceColor)>, color: PieceColor) -> bool {
    match piece {
        Some((piece_type, piece_color)) => {
            piece_type == PieceType::King && piece_color == get_opposite_color(color)
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

pub fn get_cell_paragraph(
    board: &Board,
    cell_coordinates: [i8; 2],
    bounding_rect: Rect,
) -> Paragraph<'_> {
    // Get piece and color
    let piece_color = get_piece_color(board.board, cell_coordinates);
    let piece_type = get_piece_type(board.board, cell_coordinates);
    let piece_enum = PieceType::piece_type_to_string_enum(piece_type, &board.display_mode);

    let paragraph = match board.display_mode {
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

#[cfg(test)]
mod tests {
    use crate::utils::{convert_notation_into_position, convert_position_into_notation};

    #[test]
    fn convert_position_into_notation_1() {
        assert_eq!(convert_position_into_notation("7152".to_string()), "b1-c3")
    }

    #[test]
    fn convert_position_into_notation_2() {
        assert_eq!(convert_position_into_notation("0257".to_string()), "c8-h3")
    }

    #[test]
    fn convert_notation_into_position_1() {
        assert_eq!(convert_notation_into_position("c8b7".to_string()), "0211")
    }
    #[test]
    fn convert_notation_into_position_2() {
        assert_eq!(convert_notation_into_position("g7h8".to_string()), "1607")
    }
    #[test]
    fn convert_notation_into_position_3() {
        assert_eq!(convert_notation_into_position("g1f3".to_string()), "7655")
    }
}
