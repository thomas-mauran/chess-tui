use crate::utils::{cleaned_positions, is_cell_color_ally};
use super::{PieceType, PieceColor};

pub struct King{}
impl King{
  pub fn to_string() -> &'static str{
    "\
    \n\
      █\n\
    ██░██\n\
      █\n\
    █████\n\
    "
  }

  pub fn authorized_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
    // Pawns can only move in one direction depending of their color
    let mut positions: Vec<Vec<i32>> = vec![];
    
    let y = coordinates[0];
    let x = coordinates[1]; 

    // can move on a complete row
    // Generate positions in all eight possible directions
    // TODO: Calculate the cells where it can't go because of check
    for &dy in &[-1, 0, 1] {
      for &dx in &[-1, 0, 1] {
          // Skip the case where both dx and dy are zero (the current position)
          let new_x = x + dx;
          let new_y = y + dy;
          if new_x > 0 && new_y > 0 && new_x <= 7 && new_y <= 7 && !is_cell_color_ally(board, [new_y, new_x], color) {
            positions.push(vec![y + dy, x + dx]);
          }
      }
  }

    cleaned_positions(positions)
  }
}


