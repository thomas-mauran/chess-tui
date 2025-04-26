use super::coord::Coord;
use crate::pieces::PieceType;
use shakmaty::Color;

pub type Board = [[Option<(PieceType, Color)>; 8]; 8];

impl std::ops::Index<&Coord> for Board {
    type Output = Option<(PieceType, Color)>;

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
            Some((PieceType::Rook, Color::Black)),
            Some((PieceType::Knight, Color::Black)),
            Some((PieceType::Bishop, Color::Black)),
            Some((PieceType::Queen, Color::Black)),
            Some((PieceType::King, Color::Black)),
            Some((PieceType::Bishop, Color::Black)),
            Some((PieceType::Knight, Color::Black)),
            Some((PieceType::Rook, Color::Black)),
        ],
        [
            Some((PieceType::Pawn, Color::Black)),
            Some((PieceType::Pawn, Color::Black)),
            Some((PieceType::Pawn, Color::Black)),
            Some((PieceType::Pawn, Color::Black)),
            Some((PieceType::Pawn, Color::Black)),
            Some((PieceType::Pawn, Color::Black)),
            Some((PieceType::Pawn, Color::Black)),
            Some((PieceType::Pawn, Color::Black)),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            Some((PieceType::Pawn, Color::White)),
            Some((PieceType::Pawn, Color::White)),
            Some((PieceType::Pawn, Color::White)),
            Some((PieceType::Pawn, Color::White)),
            Some((PieceType::Pawn, Color::White)),
            Some((PieceType::Pawn, Color::White)),
            Some((PieceType::Pawn, Color::White)),
            Some((PieceType::Pawn, Color::White)),
        ],
        [
            Some((PieceType::Rook, Color::White)),
            Some((PieceType::Knight, Color::White)),
            Some((PieceType::Bishop, Color::White)),
            Some((PieceType::Queen, Color::White)),
            Some((PieceType::King, Color::White)),
            Some((PieceType::Bishop, Color::White)),
            Some((PieceType::Knight, Color::White)),
            Some((PieceType::Rook, Color::White)),
        ],
    ]
}
