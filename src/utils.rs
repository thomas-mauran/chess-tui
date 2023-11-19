pub fn get_piece_color(board: [[&'static str; 8]; 8], coordinates: [i32; 2]) -> char {
    return board[coordinates[0] as usize][coordinates[1] as usize].chars().next().unwrap();
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
  
  // Return true for empty cell and ally cell color; false for enemy
pub fn is_cell_color_ally(board: [[&'static str; 8]; 8], coordinates: [i32; 2], color: char) -> bool{
    let cell_color = get_piece_color(board, coordinates);
    if cell_color == ' ' { return false;}
  
    return color == cell_color 
  }
  
pub fn is_valid(coordinates: [i32; 2]) -> bool{
    let (y, x) = (coordinates[0], coordinates[1]);
  
    return y < 8 && y >= 0 && x < 8 && x >= 0
  }