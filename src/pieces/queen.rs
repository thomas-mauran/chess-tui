use crate::utils::cleaned_positions;
use crate::pieces::bishop::Bishop;
use super::rook::Rook;

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
  pub fn authorized_positions(coordinates: [i32; 2], color: char, board: [[&'static str; 8]; 8]) -> Vec<Vec<i32>> {
    let mut positions: Vec<Vec<i32>> = vec![];

    // Queen = bishop concat rook
    positions.extend(Bishop::authorized_positions(coordinates, color, board));
    positions.extend(Rook::authorized_positions(coordinates, color, board));

    cleaned_positions(positions)
  }
}