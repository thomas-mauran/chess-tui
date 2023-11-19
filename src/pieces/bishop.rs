use crate::utils::{is_valid, get_piece_color, is_cell_color_ally, cleaned_positions};
pub struct Bishop{}
impl Bishop{
  pub fn to_string() -> &'static str{
    "\
    \n\
     ███\n\
    ██ ██\n\
     ███\n\
    █████\n\
    "
  }
  pub fn authorized_positions(coordinates: [i32; 2], color: char, board: [[&'static str; 8]; 8]) -> Vec<Vec<i32>> {
    let mut positions: Vec<Vec<i32>> = vec![];

    let y = coordinates[0];
    let x = coordinates[1];

    // for diagonal from piece to top left
    for i in 1..8i32 {
        let new_x = x - i;
        let new_y = y - i;
        let new_coordinates = [new_y, new_x];

      // Invalid coords
      if !is_valid(new_coordinates) {
        break;
      }
  
      // Empty cell 
      if get_piece_color(board, new_coordinates) == ' ' {
          positions.push(new_coordinates.to_vec());
          continue;
      }
      // Ally cell
      if is_cell_color_ally(board, new_coordinates, color) {
          break;
      }
      // Enemy cell
      positions.push(new_coordinates.to_vec());
      break;
    }

    // for diagonal from piece to bottom left
    for i in 1..8i32 {
      let new_x = x + i;
      let new_y = y + i;
      let new_coordinates = [new_y, new_x];

      // Invalid coords
      if !is_valid(new_coordinates) {
        break;
      }
  
      // Empty cell 
      if get_piece_color(board, new_coordinates) == ' ' {
          positions.push(new_coordinates.to_vec());
          continue;
      }
      // Ally cell
      if is_cell_color_ally(board, new_coordinates, color) {
          break;
      }
      // Enemy cell
      positions.push(new_coordinates.to_vec());
      break;
    }

     // for diagonal from piece to bottom right
    for i in 1..8i32 {
      let new_x = x - i;
      let new_y = y + i;
      let new_coordinates = [new_y, new_x];

    // Invalid coords
    if !is_valid(new_coordinates) {
      break;
    }

    // Empty cell 
    if get_piece_color(board, new_coordinates) == ' ' {
        positions.push(new_coordinates.to_vec());
        continue;
    }
    // Ally cell
    if is_cell_color_ally(board, new_coordinates, color) {
        break;
    }
    // Enemy cell
    positions.push(new_coordinates.to_vec());
    break;
  }

  // for diagonal from piece to top right
  for i in 1..8i32 {
    let new_x = x + i;
    let new_y = y - i;
    let new_coordinates = [new_y, new_x];

    // Invalid coords
    if !is_valid(new_coordinates) {
      break;
    }

    // Empty cell 
    if get_piece_color(board, new_coordinates) == ' ' {
        positions.push(new_coordinates.to_vec());
        continue;
    }
    // Ally cell
    if is_cell_color_ally(board, new_coordinates, color) {
        break;
    }
    // Enemy cell
    positions.push(new_coordinates.to_vec());
    break;
  }

    cleaned_positions(positions)
  }

}
