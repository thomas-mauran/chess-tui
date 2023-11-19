pub mod rook;
pub mod pawn;
pub mod bishop;
pub mod king;
pub mod knight;
pub mod queen;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

