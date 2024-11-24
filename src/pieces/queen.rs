use super::rook::Rook;
use super::{Movable, PieceColor, Position};
use crate::constants::DisplayMode;
use crate::game::coord::Coord;
use crate::game::game_board::GameBoard;
use crate::pieces::bishop::Bishop;
use crate::utils::cleaned_positions;

pub struct Queen;

impl Movable for Queen {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        allow_move_on_ally_positions: bool,
    ) -> Vec<Coord> {
        let mut positions = vec![];

        // Queen = bishop concat rook
        positions.extend(Bishop::piece_move(
            coordinates,
            color,
            game_board,
            allow_move_on_ally_positions,
        ));
        positions.extend(Rook::piece_move(
            coordinates,
            color,
            game_board,
            allow_move_on_ally_positions,
        ));

        cleaned_positions(&positions)
    }
}

impl Position for Queen {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
        _is_king_checked: bool,
    ) -> Vec<Coord> {
        game_board.impossible_positions_king_checked(
            coordinates,
            Self::piece_move(coordinates, color, game_board, false),
            color,
        )
    }
    fn protected_positions(
        coordinates: &Coord,
        color: PieceColor,
        game_board: &GameBoard,
    ) -> Vec<Coord> {
        Self::piece_move(coordinates, color, game_board, true)
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
