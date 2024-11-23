use super::coord::Coord;
use crate::pieces::{PieceColor, PieceType};

/// only the pure gameboard, no additional information
pub type GameBoard = [[Option<(PieceType, PieceColor)>; 8]; 8];

impl std::ops::Index<&Coord> for GameBoard {
    type Output = Option<(PieceType, PieceColor)>;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self[index.row as usize][index.col as usize]
    }
}

impl std::ops::IndexMut<&Coord> for GameBoard {
    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        &mut self[index.row as usize][index.col as usize]
    }
}
