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

  pub fn authorized_positions(coordinates: [i32; 2], color: char) -> Vec<Vec<i32>>{
    // Pawns can only move in one direction depending of their color
    // -1 if they are white (go up) +1 if they are black (go down)
    let direction = if color == 'w' {-1} else { 1 };

    let mut positions: Vec<Vec<i32>> = vec![];

    let (y, x) = (coordinates[0], coordinates[1]);

    // can move one cell
    positions.push(vec![y + direction, x]);

    // can move one cell
    positions.push(vec![y + direction * 2, x]);

    //TODO: En passant
    
    cleaned_positions(positions, coordinates)
  }
}

pub struct Rook{}

impl Rook{
  pub fn to_string() -> &'static str{
    "\
    \n\
    █ █ █\n\
    █████\n\
     ███\n\
    █████\n\
    "
  }

  pub fn authorized_positions(coordinates: [i32; 2], _color: char) -> Vec<Vec<i32>>{
    // Pawns can only move in one direction depending of their color
    let mut positions: Vec<Vec<i32>> = vec![];

    let (y, x) = (coordinates[0], coordinates[1]);

    // can move on a complete row
    for i in 0..8i32 {
      if i != x{
        positions.push(vec![y, i]);
      }
      if i != y{
        positions.push(vec![i, x]);
      }
    }

    cleaned_positions(positions, coordinates)
  }
}
pub struct Queen{}

impl Queen{
  pub fn to_string() -> &'static str{
    "\
    \n\
    █ ░ █\n\
    █ █ █\n\
     ███\n\
    █████\n\
    "
  }
  pub fn authorized_positions(coordinates: [i32; 2], _color: char) -> Vec<Vec<i32>>{
    // Pawns can only move in one direction depending of their color
    let mut positions: Vec<Vec<i32>> = vec![];

    let (y, x) = (coordinates[0], coordinates[1]);

    // can move on a complete row
    for i in 0..16i32 {
      if i != x{
        positions.push(vec![y, i]);
      }
      if i != y{
        positions.push(vec![i, x]);
      }
      positions.push(vec![y - 8 + i, x - 8 + i]);
      positions.push(vec![i + y - 8, x + 8 - i]);
    }

    cleaned_positions(positions, coordinates)
  }
}

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

  pub fn authorized_positions(coordinates: [i32; 2], _color: char) -> Vec<Vec<i32>>{
    // Pawns can only move in one direction depending of their color
    let mut positions: Vec<Vec<i32>> = vec![];
    
    let y = coordinates[0];
    let x = coordinates[1]; 

    // can move on a complete row
    // Generate positions in all eight possible directions
    for &dy in &[-1, 0, 1] {
      for &dx in &[-1, 0, 1] {
          // Skip the case where both dx and dy are zero (the current position)
          positions.push(vec![y + dy, x + dx]);
      }
  }

    cleaned_positions(positions, coordinates)
  }
}
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
  pub fn authorized_positions(coordinates: [i32; 2], _color: char) -> Vec<Vec<i32>>{
    // Pawns can only move in one direction depending of their color
    let mut positions: Vec<Vec<i32>> = vec![];
    
    let y = coordinates[0];
    let x = coordinates[1]; 

    // can move on a complete row
    for i in 0..16i32 {
      positions.push(vec![y - 8 + i, x - 8 + i]);
      positions.push(vec![i + y - 8, x + 8 - i]);
    }

    cleaned_positions(positions, coordinates)
  }
}


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

  pub fn authorized_positions(coordinates: [i32; 2], _color: char) -> Vec<Vec<i32>> {
    let mut positions: Vec<Vec<i32>> = Vec::new();

    let (y, x) = (coordinates[0], coordinates[1]);

    // Generate knight positions in all eight possible L-shaped moves
    let knight_moves = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

    for &(dy, dx) in &knight_moves {
        positions.push(vec![y + dy, x + dx]);
    }

    cleaned_positions(positions, coordinates)
}

}

// method to clean the position array to remove impossible positions
fn cleaned_positions(positions: Vec<Vec<i32>>, coordinates: [i32; 2]) -> Vec<Vec<i32>> {
  let mut cleaned_array: Vec<Vec<i32>> = vec![];
  for position in positions {
      if (position[0] >= 0 && position[0] <= 7)
          && (position[1] >= 0 && position[1] <= 7)
          && (position[0] != coordinates[0] || position[1] != coordinates[1])
      {
          cleaned_array.push(position);
      }
  }
  cleaned_array
}
