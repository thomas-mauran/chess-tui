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
            Some((PieceType::Rook, PieceColor::Black)),
            Some((PieceType::Knight, PieceColor::Black)),
            Some((PieceType::Bishop, PieceColor::Black)),
            Some((PieceType::Queen, PieceColor::Black)),
            Some((PieceType::King, PieceColor::Black)),
            Some((PieceType::Bishop, PieceColor::Black)),
            Some((PieceType::Knight, PieceColor::Black)),
            Some((PieceType::Rook, PieceColor::Black)),
        ],
        [
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
            Some((PieceType::Pawn, PieceColor::Black)),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, Some((PieceType::Pawn, PieceColor::White)), None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
            Some((PieceType::Pawn, PieceColor::White)),
        ],
        [
            Some((PieceType::Rook, PieceColor::White)),
            Some((PieceType::Knight, PieceColor::White)),
            Some((PieceType::Bishop, PieceColor::White)),
            Some((PieceType::Queen, PieceColor::White)),
            Some((PieceType::King, PieceColor::White)),
            Some((PieceType::Bishop, PieceColor::White)),
            Some((PieceType::Knight, PieceColor::White)),
            Some((PieceType::Rook, PieceColor::White)),
        ],
    ]
}
