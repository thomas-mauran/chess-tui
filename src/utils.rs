use crate::{
    board::Board,
    pieces::{PieceColor, PieceType},
};

pub fn get_piece_color(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    coordinates: [i32; 2],
) -> Option<PieceColor> {
    board[coordinates[0] as usize][coordinates[1] as usize].map(|(_, piece_color)| piece_color)
}

pub fn get_piece_type(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    coordinates: [i32; 2],
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
pub fn cleaned_positions(positions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cleaned_array: Vec<Vec<i32>> = vec![];
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
    coordinates: [i32; 2],
    color: PieceColor,
) -> bool {
    match get_piece_color(board, coordinates) {
        Some(cell_color) => cell_color == color,
        None => false, // Treat empty cell as ally
    }
}

pub fn is_valid(coordinates: [i32; 2]) -> bool {
    let (y, x) = (coordinates[0], coordinates[1]);

    (0..8).contains(&y) && (0..8).contains(&x)
}

pub fn is_vec_in_array(array: Vec<Vec<i32>>, element: [i32; 2]) -> bool {
    for position in array {
        if position == element {
            return true;
        }
    }
    false
}

// We get all the cells that are getting put in 'check'
pub fn get_all_checked_cells(
    board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    color: PieceColor,
) -> Vec<Vec<i32>> {
    let mut check_cells: Vec<Vec<i32>> = vec![];
    for i in 0..8i32 {
        for j in 0..8i32 {
            if get_piece_color(board, [i, j]) == Some(color) {
                continue;
            }
            if let Some(piece_color) = get_piece_color(board, [i, j]) {
                if let Some(piece_type) = get_piece_type(board, [i, j]) {
                    check_cells.extend(Board::protected_positions_enum(
                        [i, j],
                        piece_type,
                        piece_color,
                        board,
                    ));
                }
            }
        }
    }
    check_cells
}

pub fn col_to_letter(col: i32) -> String {
    match col {
        0 => "a".to_string(),
        1 => "b".to_string(),
        2 => "c".to_string(),
        3 => "d".to_string(),
        4 => "e".to_string(),
        5 => "f".to_string(),
        6 => "g".to_string(),
        7 => "h".to_string(),
        _ => panic!("Col out of bound {}", col),
    }
}

pub fn convert_position_into_notation(position: i32) -> String {
    let mut result: String = "".to_string();
    // println!("{}", position);
    let from_y: i32 = position / 1000 % 10;
    let from_x: i32 = position / 100 % 10;
    let to_y: i32 = position / 10 % 10;
    let to_x: i32 = position % 10;

    result += &col_to_letter(from_x);
    result += &format!("{}", (8 - from_y) % 8).to_string();
    result += "-";
    result += &col_to_letter(to_x);
    result += &format!("{}", (8 - to_y) % 8).to_string();

    result
}
