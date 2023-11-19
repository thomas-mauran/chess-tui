use crate::utils::{is_valid, get_piece_color, is_cell_color_ally, cleaned_positions};
pub struct Knight{}
impl Knight{
  pub fn to_string() -> &'static str{
    "\
    \n\
     ██\n\
    ██░██\n\
    ███  \n\
    █████\n\
    "
  }

  pub fn authorized_positions(coordinates: [i32; 2], color: char, board: [[&'static str; 8]; 8]) -> Vec<Vec<i32>>{
    let mut positions: Vec<Vec<i32>> = Vec::new();

    let (y, x) = (coordinates[0], coordinates[1]);

    // Generate knight positions in all eight possible L-shaped moves
    let knight_moves = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

    for &(dy, dx) in &knight_moves {
      let new_coordinates = [y + dy, x + dx];

      if is_valid(new_coordinates) && !is_cell_color_ally(board, new_coordinates, color){
        positions.push(new_coordinates.to_vec());
      }
    }

    cleaned_positions(positions)
  }
}