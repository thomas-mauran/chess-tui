use crate::{pieces::{PieceColor, PieceType}, board::Board};

pub fn get_piece_color(board: [[Option<(PieceType, PieceColor)>; 8]; 8], coordinates: [i32; 2]) -> Option<PieceColor> {
  match board[coordinates[0] as usize][coordinates[1] as usize] {
    Some((_, piece_color)) => Some(piece_color),
    None => None
  }
}

pub fn get_piece_type(board: [[Option<(PieceType, PieceColor)>; 8]; 8], coordinates: [i32; 2]) -> Option<PieceType> {
  match board[coordinates[0] as usize][coordinates[1] as usize] {
      Some((piece_type, _)) => Some(piece_type),
      None => None,
  }
}

pub fn get_opposite_color(color: PieceColor) -> PieceColor {
  match color {
      PieceColor::Black => PieceColor::White,
      PieceColor::White => PieceColor::Black
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

pub fn is_vec_in_array(array: Vec<Vec<i32>>, element: [i32; 2]) -> bool {
  for position in array{
    if position == element { return true;}
  }
  return false;
}

// We get all the cells that are getting put in 'check'
pub fn get_all_checked_cells(board: [[Option<(PieceType, PieceColor)>; 8]; 8], color: PieceColor) -> Vec<Vec<i32>>{
  let mut check_cells: Vec<Vec<i32>> = vec![];
    for i in 0..8i32{
      for j in 0..8i32{
        if get_piece_color(board, [i, j]) == Some(color) { continue; }
        if let Some(piece_color) = get_piece_color(board, [i, j]){
          if let Some(piece_type) = get_piece_type(board, [i, j]){
            check_cells.extend(Board::protected_positions_enum([i, j], piece_type, piece_color, board));
          }
      }
    }
  }
  return check_cells

}