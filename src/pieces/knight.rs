use crate::utils::{is_valid, is_cell_color_ally, cleaned_positions};
use super::{PieceType, PieceColor};
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

  pub fn knight_moves(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8], allow_move_on_ally_positions: bool) -> Vec<Vec<i32>>{
    let mut positions: Vec<Vec<i32>> = Vec::new();

    let (y, x) = (coordinates[0], coordinates[1]);

    // Generate knight positions in all eight possible L-shaped moves
    let knight_moves = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

    for &(dy, dx) in &knight_moves {
      let new_coordinates = [y + dy, x + dx];

      if !is_valid(new_coordinates){
        continue;
      }

      if is_cell_color_ally(board, new_coordinates, color) && !allow_move_on_ally_positions {
        continue;
      }
      
      positions.push(new_coordinates.to_vec());
    }

    cleaned_positions(positions)
  }

  pub fn authorized_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
    Self::knight_moves(coordinates, color, board, false)
  }

  pub fn protecting_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
    Self::knight_moves(coordinates, color, board, true)
  }
}

#[cfg(test)]
mod tests {
  use crate::{board::Board, pieces::{PieceType, PieceColor, knight::Knight}};

  #[test]
  fn knight_moves_no_enemies() {
      let custom_board = [
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, Some((PieceType::Knight, PieceColor::White)), None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
      ];
      let mut board = Board::default();
      board.set_board(custom_board);

      let mut right_positions = vec![
        vec![2, 3],
        vec![2, 5],

        vec![3, 2],
        vec![3, 6],

        vec![5, 2],
        vec![5, 6],

        vec![6, 3],
        vec![6, 5],

      ];
      right_positions.sort();

      let mut positions = Knight::authorized_positions([4, 4], PieceColor::White, board.board);
      positions.sort();

      assert_eq!(right_positions, positions);
  }

  #[test]
  fn knight_moves_enemy_and_ally() {
      let custom_board = [
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, Some((PieceType::Pawn, PieceColor::White)), None],
          [None, None, None, None, None, Some((PieceType::Pawn, PieceColor::Black)), None, None],
          [None, None, None, None, None, None, None, Some((PieceType::Knight, PieceColor::White))],
      ];
      let mut board = Board::default();
      board.set_board(custom_board);

      let mut right_positions = vec![
        vec![6, 5],

      ];
      right_positions.sort();

      let mut positions = Knight::authorized_positions([7, 7], PieceColor::White, board.board);
      positions.sort();

      assert_eq!(right_positions, positions);
  }
}