use super::coord::Coord;
use crate::pieces::{PieceColor, PieceType};

pub type Board = [[Option<(PieceType, PieceColor)>; 8]; 8];

impl std::ops::Index<&Coord> for Board {
    type Output = Option<(PieceType, PieceColor)>;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self[index.row as usize][index.col as usize]
    }
}

impl std::ops::IndexMut<&Coord> for Board {
    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        &mut self[index.row as usize][index.col as usize]
    }
}

pub fn init_board() -> Board {
    [
        [
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, Some((PieceType::Queen, PieceColor::White)), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ]
}
