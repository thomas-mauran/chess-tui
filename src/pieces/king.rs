use crate::utils::{cleaned_positions, is_cell_color_ally, get_all_checked_cells, is_vec_in_array, is_valid};
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

  pub fn king_moves(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8], allow_move_on_ally_positions: bool) -> Vec<Vec<i32>>{
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

          let new_coordinates = [new_y, new_x];
          if is_valid(new_coordinates) && (!is_cell_color_ally(board, [new_y, new_x], color) || allow_move_on_ally_positions) {
            positions.push(vec![y + dy, x + dx]);
          }
      }
    }
  
    cleaned_positions(positions)
  }

  pub fn authorized_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
    let mut positions: Vec<Vec<i32>> = vec![];
    let impossible_cells = get_all_checked_cells(board, color);
    let king_cells = King::king_moves(coordinates, color, board, false);

    // Here we only want king positions that are not in impossible
    for king_position in king_cells.clone(){
      if !is_vec_in_array(impossible_cells.clone(), [king_position[0], king_position[1]]) {
        positions.push(king_position);
      }
    }

    return positions
  }


  // This method is used to calculated the cells the king is actually covering and is used when the other king authorized position is called
  pub fn protecting_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
    Self::king_moves(coordinates, color, board, true)
  }

}




#[cfg(test)]
mod tests {
  use crate::{board::Board, pieces::{PieceType, PieceColor, king::King}};

  #[test]
  fn king_moves_no_enemies() {
      let custom_board = [
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, Some((PieceType::Bishop, PieceColor::Black)), None, None, Some((PieceType::Bishop, PieceColor::Black)), None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, Some((PieceType::King, PieceColor::White)), None, None, None],
        [None, None, None, None, None, Some((PieceType::Bishop, PieceColor::Black)), None, None],
        [None, None, Some((PieceType::Bishop, PieceColor::Black)), None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
      ];
      let mut board = Board::default();
      board.set_board(custom_board);

      let mut right_positions = vec![
        vec![4, 5],
        vec![5, 4],
      ];
      right_positions.sort();

      let mut positions = King::authorized_positions([4, 4], PieceColor::White, board.board);
      positions.sort();

      assert_eq!(right_positions, positions);
  }

}