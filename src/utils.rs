use crate::pieces::{PieceColor, PieceType};

pub fn get_piece_color(board: [[Option<(PieceType, PieceColor)>; 8]; 8], coordinates: [i32; 2]) -> Option<PieceColor> {
  match board[coordinates[0] as usize][coordinates[1] as usize] {
    Some((_, piece_color)) => Some(piece_color),
    None => None
  }
}


// method to clean the position array to remove impossible positions
pub fn cleaned_positions(positions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cleaned_array: Vec<Vec<i32>> = vec![];
    for position in positions {
        if is_valid([position[0], position[1]])
        {
            cleaned_array.push(position);
        }
    }
    cleaned_array
  }
  
// Return true forally cell color; false for enemy
pub fn is_cell_color_ally(board: [[Option<(PieceType, PieceColor)>; 8]; 8], coordinates: [i32; 2], color: PieceColor) -> bool {
  match get_piece_color(board, coordinates) {
      Some(cell_color) => cell_color == color,
      None => false, // Treat empty cell as ally
  }
}
  
pub fn is_valid(coordinates: [i32; 2]) -> bool{
    let (y, x) = (coordinates[0], coordinates[1]);
  
    return y < 8 && y >= 0 && x < 8 && x >= 0
  }