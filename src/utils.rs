use crate::{
    board::{BoardState, GameBoard},
    notations::Coords,
    pieces::{Piece, PieceColor, PieceKind},
};
use ratatui::style::Color;

pub fn get_piece_color(board: &GameBoard, coordinates: &Coords) -> Option<PieceColor> {
    board[coordinates.row as usize][coordinates.col as usize].map(|Piece { kind: _, color }| color)
    // board.get(coordinates).map(|(_, piece_color)| piece_color)
}

pub fn get_piece_kind(board: &GameBoard, coordinates: &Coords) -> Option<PieceKind> {
    board[coordinates.row as usize][coordinates.col as usize].map(|Piece { kind, color: _ }| kind)
}

// method to clean the position array to remove impossible positions
pub fn cleaned_positions(positions: Vec<Coords>) -> Vec<Coords> {
    let mut cleaned_array: Vec<Coords> = vec![];
    for position in positions {
        if position.is_valid() {
            cleaned_array.push(position);
        }
    }
    cleaned_array
    // positions
    //     .iter()
    //     .filter(|coords| coords.is_valid())
    //     .fold(Vec::new(), |res, item| res + item.clone())
}

// Return true forally cell color; false for enemy
pub fn is_cell_color_ally(board: GameBoard, coordinates: Coords, color: PieceColor) -> bool {
    match get_piece_color(&board, &coordinates) {
        Some(cell_color) => cell_color == color,
        None => false, // Treat empty cell as ally
    }
}

// We get all the cells that are getting put in 'check'
pub fn get_all_protected_cells(
    board: GameBoard,
    player_turn: PieceColor,
    move_history: &[(Option<PieceKind>, String)],
) -> Vec<Coords> {
    let mut check_cells: Vec<Coords> = vec![];
    for i in 0..8i8 {
        for j in 0..8i8 {
            let coords = &Coords::new(i, j);
            if get_piece_color(&board, coords) == Some(player_turn) {
                continue;
            }
            // get the current cell piece color and type protecting positions
            if let Some(piece_color) = get_piece_color(&board, coords) {
                if let Some(piece_kind) = get_piece_kind(&board, coords) {
                    check_cells.extend(Piece::new(piece_kind, piece_color).protected_positions(
                        coords,
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
    let mut result = String::new();
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

pub fn get_int_from_char(ch: Option<char>) -> i8 {
    match ch {
        Some(ch) => ch.to_digit(10).unwrap() as i8,
        _ => unreachable!("a, b, c, d, e, f, g, h are covered, nothing else is used"),
    }
}

pub fn get_latest_move(
    move_history: &[(Option<PieceKind>, String)],
) -> (Option<PieceKind>, String) {
    if !move_history.is_empty() {
        return move_history[move_history.len() - 1].clone();
    }
    (None, "0000".to_string())
}

pub fn did_piece_already_move(
    move_history: &[(Option<PieceKind>, String)],
    original_piece: (Option<PieceKind>, &Coords),
) -> bool {
    for entry in move_history {
        let position = entry.1.clone();
        let from_y = get_int_from_char(position.chars().next());
        let from_x = get_int_from_char(position.chars().nth(1));
        // Here there is an entry with the same piece type and the same original position, meaning it moved at some point
        if entry.0 == original_piece.0 && &Coords::new(from_y, from_x) == original_piece.1 {
            return true;
        }
    }
    false
}
// Method returning the coordinates of the king of a certain color
pub fn get_king_coordinates(board: GameBoard, player_turn: PieceColor) -> Coords {
    for i in 0..8i8 {
        for j in 0..8i8 {
            if let Some(Piece { kind, color }) = board[i as usize][j as usize] {
                if kind == PieceKind::King && color == player_turn {
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
    move_history: &[(Option<PieceKind>, String)],
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
    move_history: &[(Option<PieceKind>, String)],
) -> Vec<Coords> {
    let mut cleaned_position: Vec<Coords> = vec![];
    for position in positions {
        // We create a new board
        let mut new_board = BoardState::new(&board, color, move_history.to_owned().clone());

        // We simulate the move

        BoardState::move_piece_on_the_board(&mut new_board, original_coordinates, &position);

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

pub fn is_piece_opposite_king(piece: Option<Piece>, orig_color: PieceColor) -> bool {
    match piece {
        Some(Piece { kind, color }) => kind == PieceKind::King && color == orig_color.opposite(),
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
