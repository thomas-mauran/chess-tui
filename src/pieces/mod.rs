use crate::{
    board::{GameBoard, MoveHistory},
    notations::Coords,
};

use self::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}
impl Piece {
    pub fn new(piece_kind: PieceKind, piece_color: PieceColor) -> Self {
        Piece {
            kind: piece_kind,
            color: piece_color,
        }
    }
    pub fn from_char(piece_char: char) -> Option<Piece> {
        let (color, kind) = match piece_char {
            'P' => (PieceColor::White, PieceKind::Pawn),
            'N' => (PieceColor::White, PieceKind::Knight),
            'B' => (PieceColor::White, PieceKind::Bishop),
            'R' => (PieceColor::White, PieceKind::Rook),
            'Q' => (PieceColor::White, PieceKind::Queen),
            'K' => (PieceColor::White, PieceKind::King),
            'p' => (PieceColor::Black, PieceKind::Pawn),
            'n' => (PieceColor::Black, PieceKind::Knight),
            'b' => (PieceColor::Black, PieceKind::Bishop),
            'r' => (PieceColor::Black, PieceKind::Rook),
            'q' => (PieceColor::Black, PieceKind::Queen),
            'k' => (PieceColor::Black, PieceKind::King),
            _ => return None,
        };

        Some(Piece { kind, color })
    }
    pub fn authorized_positions(
        &self,
        coordinates: &Coords,
        board: GameBoard,
        move_history: &MoveHistory,
        is_king_checked: bool,
    ) -> Vec<Coords> {
        match self.kind {
            PieceKind::Pawn => Pawn::authorized_positions(
                coordinates,
                self.color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceKind::Rook => Rook::authorized_positions(
                coordinates,
                self.color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceKind::Bishop => Bishop::authorized_positions(
                coordinates,
                self.color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceKind::Queen => Queen::authorized_positions(
                coordinates,
                self.color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceKind::King => King::authorized_positions(
                coordinates,
                self.color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceKind::Knight => Knight::authorized_positions(
                coordinates,
                self.color,
                board,
                move_history,
                is_king_checked,
            ),
        }
    }

    pub fn protected_positions(
        &self,
        selected_coordinates: &Coords,
        board: GameBoard,
        move_history: &MoveHistory,
    ) -> Vec<Coords> {
        match self.kind {
            PieceKind::Pawn => {
                Pawn::protected_positions(selected_coordinates, self.color, board, move_history)
            }
            PieceKind::Rook => {
                Rook::protected_positions(selected_coordinates, self.color, board, move_history)
            }
            PieceKind::Bishop => {
                Bishop::protected_positions(selected_coordinates, self.color, board, move_history)
            }
            PieceKind::Queen => {
                Queen::protected_positions(selected_coordinates, self.color, board, move_history)
            }
            PieceKind::King => {
                King::protected_positions(selected_coordinates, self.color, board, move_history)
            }
            PieceKind::Knight => {
                Knight::protected_positions(selected_coordinates, self.color, board, move_history)
            }
        }
    }

    pub fn piece_to_utf_enum(piece_kind: PieceKind, piece_color: PieceColor) -> &'static str {
        match (piece_kind, piece_color) {
            (PieceKind::Queen, PieceColor::Black) => "♕",
            (PieceKind::Queen, PieceColor::White) => "♛",
            (PieceKind::King, PieceColor::Black) => "♔",
            (PieceKind::King, PieceColor::White) => "♚",
            (PieceKind::Rook, PieceColor::Black) => "♖",
            (PieceKind::Rook, PieceColor::White) => "♜",
            (PieceKind::Bishop, PieceColor::Black) => "♗",
            (PieceKind::Bishop, PieceColor::White) => "♝",
            (PieceKind::Knight, PieceColor::Black) => "♘",
            (PieceKind::Knight, PieceColor::White) => "♞",
            (PieceKind::Pawn, PieceColor::Black) => "♙",
            (PieceKind::Pawn, PieceColor::White) => "♟",
        }
    }

    pub fn piece_to_fen_enum(
        piece_kind: Option<PieceKind>,
        piece_color: Option<PieceColor>,
    ) -> &'static str {
        match (piece_kind, piece_color) {
            (Some(PieceKind::Queen), Some(PieceColor::Black)) => "q",
            (Some(PieceKind::Queen), Some(PieceColor::White)) => "Q",
            (Some(PieceKind::King), Some(PieceColor::Black)) => "k",
            (Some(PieceKind::King), Some(PieceColor::White)) => "K",
            (Some(PieceKind::Rook), Some(PieceColor::Black)) => "r",
            (Some(PieceKind::Rook), Some(PieceColor::White)) => "R",
            (Some(PieceKind::Bishop), Some(PieceColor::Black)) => "b",
            (Some(PieceKind::Bishop), Some(PieceColor::White)) => "B",
            (Some(PieceKind::Knight), Some(PieceColor::Black)) => "n",
            (Some(PieceKind::Knight), Some(PieceColor::White)) => "N",
            (Some(PieceKind::Pawn), Some(PieceColor::Black)) => "p",
            (Some(PieceKind::Pawn), Some(PieceColor::White)) => "P",
            (None, None) => "",
            _ => unreachable!("Undefined piece and piece color tuple"),
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match (&self.color, &self.kind) {
            (&PieceColor::White, &PieceKind::Pawn) => "P",
            (&PieceColor::White, &PieceKind::Knight) => "N",
            (&PieceColor::White, &PieceKind::Bishop) => "B",
            (&PieceColor::White, &PieceKind::Rook) => "R",
            (&PieceColor::White, &PieceKind::Queen) => "Q",
            (&PieceColor::White, &PieceKind::King) => "K",
            (&PieceColor::Black, &PieceKind::Pawn) => "p",
            (&PieceColor::Black, &PieceKind::Knight) => "n",
            (&PieceColor::Black, &PieceKind::Bishop) => "b",
            (&PieceColor::Black, &PieceKind::Rook) => "r",
            (&PieceColor::Black, &PieceKind::Queen) => "q",
            (&PieceColor::Black, &PieceKind::King) => "k",
        }
        .to_owned()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

impl PieceKind {
    pub fn piece_kind_to_string_enum(piece_kind: Option<PieceKind>) -> &'static str {
        match piece_kind {
            Some(PieceKind::Queen) => Queen::to_string(),
            Some(PieceKind::King) => King::to_string(),
            Some(PieceKind::Rook) => Rook::to_string(),
            Some(PieceKind::Bishop) => Bishop::to_string(),
            Some(PieceKind::Knight) => Knight::to_string(),
            Some(PieceKind::Pawn) => Pawn::to_string(),
            None => " ",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White = 0,
    Black = 1,
}
impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
    pub fn is_white(&self) -> bool {
        match self {
            PieceColor::White => true,
            PieceColor::Black => false,
        }
    }
    pub fn is_black(&self) -> bool {
        match self {
            PieceColor::Black => true,
            PieceColor::White => false,
        }
    }
}

pub trait Movable {
    fn piece_move(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        move_history: &MoveHistory,
    ) -> Vec<Coords>;
}

pub trait Position {
    fn authorized_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        move_history: &MoveHistory,
        is_king_checked: bool,
    ) -> Vec<Coords>;

    fn protected_positions(
        coordinates: &Coords,
        color: PieceColor,
        board: GameBoard,
        move_history: &MoveHistory,
    ) -> Vec<Coords>;
}
