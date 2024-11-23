use super::rook::Rook;
use super::{Movable, PieceColor, PieceMove, Position};
use crate::game::game::{Coord, GameBoard};
use crate::constants::DisplayMode;
use crate::pieces::bishop::Bishop;
use crate::utils::{cleaned_positions, impossible_positions_king_checked};

pub struct Queen;

impl Movable for Queen {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        move_history: &[PieceMove],
    ) -> Vec<Coord> {
        let mut positions = vec![];

        // Queen = bishop concat rook
        positions.extend(Bishop::piece_move(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
            move_history,
        ));
        positions.extend(Rook::piece_move(
            coordinates,
            color,
            board,
            allow_move_on_ally_positions,
            move_history,
        ));

        cleaned_positions(&positions)
    }
}

impl Position for Queen {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
        _is_king_checked: bool,
    ) -> Vec<Coord> {
        impossible_positions_king_checked(
            coordinates,
            Self::piece_move(coordinates, color, board, false, move_history),
            board,
            color,
            move_history,
        )
    }
    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, board, true, move_history)
    }
}

impl Queen {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
    \n\
◀█▟█▙█▶\n\
  ◥█◈█◤\n\
  ███\n\
▗█████▖\n\
    "
            }
            DisplayMode::ASCII => "Q",
        }
    }
}
