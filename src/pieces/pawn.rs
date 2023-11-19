use crate::utils::{get_piece_color, cleaned_positions, is_valid, is_cell_color_ally};
use super::{PieceType, PieceColor};

pub struct Pawn{}

impl Pawn{
  pub fn to_string() -> &'static str{
    "\
    \n\
    \n\
      █\n\
     ███\n\
     ███\n\
    "
  }

  pub fn authorized_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>>{
    // Pawns can only move in one direction depending of their color
    // -1 if they are white (go up) +1 if they are black (go down)
    let direction = if color == PieceColor::White {-1} else { 1 };

    let mut positions: Vec<Vec<i32>> = vec![];

    let (y, x) = (coordinates[0], coordinates[1]);

    // move one in front
    let new_x_front_one = x;
    let new_y_front_one = y + direction * 1;
    let new_coordinates_front_one = [new_y_front_one, new_x_front_one];

    if is_valid(new_coordinates_front_one) {
      if get_piece_color(board, new_coordinates_front_one).is_none() {
          // Empty cell
          positions.push(new_coordinates_front_one.to_vec());
  
          // move front a second cell
          let new_x_front_two = x;
          let new_y_front_two = y + direction * 2;
          let new_coordinates_front_two = [new_y_front_two, new_x_front_two];
  
          if is_valid(new_coordinates_front_two)
              && get_piece_color(board, new_coordinates_front_two).is_none()
              && ((color == PieceColor::White && y == 6) || (color == PieceColor::Black && y == 1))
          {
              positions.push(new_coordinates_front_two.to_vec());
          }
      } else if !is_cell_color_ally(board, new_coordinates_front_one, color) {
          // Enemy cell
          positions.push(new_coordinates_front_one.to_vec());
      }
  }



      // check for enemy piece on the right
      let new_x_right = x + 1;
      let new_y_right = y + direction;
      let new_coordinates_right = [new_y_right, new_x_right];

      if is_valid(new_coordinates_right) && get_piece_color(board, new_coordinates_right).is_some() && !is_cell_color_ally(board, new_coordinates_right, color){
        positions.push(new_coordinates_right.to_vec());
      }

      // check for enemy piece on the left
      let new_x_left = x - 1;
      let new_y_left = y + direction;
      let new_coordinates_left = [new_y_left, new_x_left];

      if is_valid(new_coordinates_left) && get_piece_color(board, new_coordinates_left).is_some() && !is_cell_color_ally(board, new_coordinates_left, color){
        positions.push(new_coordinates_left.to_vec());
      }
      

    //TODO: En passant
    
    cleaned_positions(positions)
  }
}


#[cfg(test)]
mod tests {
    use crate::{board::Board, pieces::{pawn::Pawn, PieceType, PieceColor}};

    #[test]
    fn pawn_moves_one_cell_forward() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, Some((PieceType::Pawn, PieceColor::White)), None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let positions = Pawn::authorized_positions([4, 4], PieceColor::White, board.board);

        assert_eq!(vec![vec![3, 4]], positions);
    }

    #[test]
    fn pawn_moves_one_cell_forward_two() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, Some((PieceType::Pawn, PieceColor::White)), None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let positions = Pawn::authorized_positions([6, 4], PieceColor::White, board.board);

        assert_eq!(vec![vec![5, 4], vec![4, 4]], positions);
    }

    #[test]
    fn pawn_moves_one_cell_enemy_left_right() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, Some((PieceType::Pawn, PieceColor::Black)), None, None, None, None],
            [None, None, Some((PieceType::Pawn, PieceColor::White)), None, Some((PieceType::Pawn, PieceColor::White)), None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let positions = Pawn::authorized_positions([1, 3], PieceColor::Black, board.board);

        let right_positions = vec![
          vec![2, 3], 
          vec![3, 3],
          vec![2, 4],
          vec![2, 2],
        ];
        assert_eq!(right_positions, positions);
    }
}
