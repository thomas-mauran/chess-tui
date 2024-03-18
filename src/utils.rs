use crate::{
    board::{Board, Coords, GameBoard, HistRec, Piece},
    pieces::{PieceColor, PieceType},
};
use ratatui::style::Color;

pub fn get_piece_color(board: GameBoard, coordinates: &Coords) -> Option<PieceColor> {
    board[coordinates.row as usize][coordinates.col as usize].map(|(_, piece_color)| piece_color)
}

pub fn get_piece_type(board: GameBoard, coordinates: &Coords) -> Option<PieceType> {
    board[coordinates.row as usize][coordinates.col as usize].map(|(piece_type, _)| piece_type)
}

/// method to clean the position array to remove impossible positions
pub fn cleaned_positions(positions: Vec<Coords>) -> Vec<Coords> {
    positions
        .into_iter()
        .filter(|item| item.is_valid())
        .collect()
}

/// Return true for ally cell color; false for enemy
pub fn is_cell_color_ally(board: GameBoard, coordinates: Coords, color: PieceColor) -> bool {
    match get_piece_color(board, &coordinates) {
        Some(cell_color) => cell_color == color,
        None => false, // Treat empty cell as ally
    }
}

/// We get all the cells that are getting put in 'check'
pub fn get_all_protected_cells(
    board: GameBoard,
    player_turn: PieceColor,
    move_history: &[HistRec],
) -> Vec<Coords> {
    let mut check_cells: Vec<Coords> = vec![];
    for i in 0..8i8 {
        for j in 0..8i8 {
            let coords = &Coords::new(i, j);
            if get_piece_color(board, coords) == Some(player_turn) {
                continue;
            }
            // get the current cell piece color and type protecting positions
            if let Some(piece_color) = get_piece_color(board, coords) {
                if let Some(piece_type) = get_piece_type(board, coords) {
                    check_cells.extend(PieceType::protected_positions(
                        coords,
                        piece_type,
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
    let from_y = chtoi(position.chars().next());
    let from_x = chtoi(position.chars().nth(1));
    let to_y = chtoi(position.chars().nth(2));
    let to_x = chtoi(position.chars().nth(3));

    format!(
        "{}{}-{}{}",
        &col_to_letter(from_x),
        8 - from_y,
        &col_to_letter(to_x),
        8 - to_y
    )
}

pub fn convert_notation_into_position(notation: String) -> String {
    let from_x = &letter_to_col(notation.chars().next());
    let from_y = (&chtoi(notation.chars().nth(1)) - 8).abs();

    let to_x = &letter_to_col(notation.chars().nth(2));
    let to_y = (&chtoi(notation.chars().nth(3)) - 8).abs();

    format!("{}{}{}{}", from_y, from_x, to_y, to_x)
}

/// character to int conversion
pub fn chtoi(ch: Option<char>) -> i8 {
    match ch {
        Some(ch) => ch.to_digit(10).unwrap() as i8,
        _ => unreachable!("only valid numbers are supposed to be passed"),
    }
}

pub fn get_latest_move(move_history: &[HistRec]) -> Option<HistRec> {
    if !move_history.is_empty() {
        return Some(move_history[move_history.len() - 1].clone());
    }
    None
}

pub fn did_piece_already_move(
    move_history: &[HistRec],
    original_piece: (PieceType, Coords),
) -> bool {
    for entry in move_history {
        let position = entry.1.clone();
        let from_y = chtoi(position.chars().next());
        let from_x = chtoi(position.chars().nth(1));
        // Here there is an entry with the same piece type and the same original position, meaning it moved at some point
        if entry.0 == original_piece.0 && Coords::new(from_y, from_x) == original_piece.1 {
            return true;
        }
    }
    false
}
// Method returning the coordinates of the king of a certain color
pub fn get_king_coordinates(board: GameBoard, player_turn: PieceColor) -> Coords {
    for i in 0..8i8 {
        for j in 0..8i8 {
            if let Some((piece_type, piece_color)) = board[i as usize][j as usize] {
                if piece_type == PieceType::King && piece_color == player_turn {
                    return Coords::new(i, j);
                }
            }
        }
    }
    Coords::default()
}

// Is getting checked
pub fn is_getting_checked(
    board: GameBoard,
    player_turn: PieceColor,
    move_history: &[HistRec],
) -> bool {
    let coordinates = get_king_coordinates(board, player_turn);

    let checked_cells = get_all_protected_cells(board, player_turn, move_history);

    for position in checked_cells {
        if position == coordinates {
            return true;
        }
    }
    false
}

pub fn impossible_positions_king_checked(
    original_coordinates: &Coords,
    positions: Vec<Coords>,
    board: GameBoard,
    color: PieceColor,
    move_history: &[HistRec],
) -> Vec<Coords> {
    let mut cleaned_position: Vec<Coords> = vec![];
    for position in positions {
        // We create a new board
        let mut new_board = Board::new(board, color, move_history.to_owned().clone());

        // We simulate the move

        Board::move_piece(&mut new_board, original_coordinates, &position);

        // We check if the board is still checked with this move meaning it didn't resolve the problem
        if !is_getting_checked(
            new_board.board,
            new_board.player_turn,
            &new_board.move_history,
        ) {
            cleaned_position.push(position)
        };
    }
    cleaned_position
}

pub fn is_piece_opposite_king(piece: Piece, color: PieceColor) -> bool {
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
