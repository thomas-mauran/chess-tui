use super::{Movable, PieceColor, PieceMove, Position};
use crate::board::{Coord, GameBoard};
use crate::constants::DisplayMode;
use crate::utils::{
    cleaned_positions, get_piece_color, impossible_positions_king_checked, is_cell_color_ally,
    is_piece_opposite_king,
};
pub struct Rook;

impl Movable for Rook {
    fn piece_move(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        allow_move_on_ally_positions: bool,
        _move_history: &[PieceMove],
    ) -> Vec<Coord> {
        // Pawns can only move in one direction depending on their color
        let mut positions = vec![];

        let (y, x) = (coordinates.row, coordinates.col);

        // RIGHT ROW
        for i in 1..8u8 {
            let new_x = x + i;
            let new_y = y;
            let new_coordinates = Coord::new(new_y, new_x);

            // Invalid coords
            if !new_coordinates.is_valid() {
                break;
            }

            // Empty cell
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates);
                    break;
                }
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // LEFT ROW
        for i in 1..=8 {
            let new_x: i8 = x as i8 - i as i8;
            let new_y: i8 = y as i8;
            let Some(new_coordinates) = Coord::opt_new(new_y, new_x) else {
                break;
            };

            // Empty piece
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }

            // Ally piece
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates);
                    break;
                }
            }

            // Enemy cell
            positions.push(new_coordinates);
            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // BOTTOM ROW
        for i in 1..8u8 {
            let new_x = x;
            let new_y = y + i;
            let new_coordinates = Coord::new(new_y, new_x);

            // Invalid coords
            if !new_coordinates.is_valid() {
                break;
            }

            // Empty cell
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates);
                    break;
                }
            }

            // Enemy cell
            positions.push(new_coordinates);

            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        // UP ROW
        for i in 1..8u8 {
            let new_x = x as i8;
            let new_y = y as i8 - i as i8;
            let Some(new_coordinates) = Coord::opt_new(new_y, new_x) else {
                break;
            };

            // Empty cell
            if get_piece_color(board, &new_coordinates).is_none() {
                positions.push(new_coordinates);
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, &new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates);
                    break;
                }
            }
            // Enemy cell
            positions.push(new_coordinates);

            if !allow_move_on_ally_positions
                || !is_piece_opposite_king(board[new_y as usize][new_x as usize], color)
            {
                break;
            }
        }

        cleaned_positions(&positions)
    }
}

impl Position for Rook {
    fn authorized_positions(
        coordinates: &Coord,
        color: PieceColor,
        board: GameBoard,
        move_history: &[PieceMove],
        _is_king_checked: bool,
    ) -> Vec<Coord> {
        // If the king is not checked we get then normal moves
        // if the king is checked we clean all the position not resolving the check
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

impl Rook {
    pub fn to_string(display_mode: &DisplayMode) -> &'static str {
        match display_mode {
            DisplayMode::DEFAULT => {
                "\
    \n\
    █▟█▙█\n\
    ▜███▛\n\
    ▐███▌\n\
   ▗█████▖\n\
    "
            }
            DisplayMode::ASCII => "R",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, Coord},
        pieces::{rook::Rook, PieceColor, PieceType, Position},
        utils::is_getting_checked,
    };

    #[test]
    fn piece_move_no_enemies() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
            Coord::new(7, 4),
            Coord::new(6, 4),
            Coord::new(5, 4),
            Coord::new(3, 4),
            Coord::new(2, 4),
            Coord::new(1, 4),
            Coord::new(0, 4),
            Coord::new(4, 0),
            Coord::new(4, 1),
            Coord::new(4, 2),
            Coord::new(4, 3),
            Coord::new(4, 5),
            Coord::new(4, 6),
            Coord::new(4, 7),
        ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_one_enemies_front() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
            Coord::new(7, 4),
            Coord::new(6, 4),
            Coord::new(5, 4),
            Coord::new(3, 4),
            Coord::new(4, 0),
            Coord::new(4, 1),
            Coord::new(4, 2),
            Coord::new(4, 3),
            Coord::new(4, 5),
            Coord::new(4, 6),
            Coord::new(4, 7),
        ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn piece_move_multiple_enemies_and_ally_front() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
            Coord::new(4, 0),
            Coord::new(4, 1),
            Coord::new(4, 2),
            Coord::new(4, 3),
            Coord::new(4, 5),
            Coord::new(4, 6),
            Coord::new(3, 4),
            Coord::new(5, 4),
        ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions(
            &Coord::new(4, 4),
            PieceColor::White,
            board.board,
            &[],
            false,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn king_checked_can_resolve() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
                None,
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions = vec![Coord::new(4, 2)];
        right_positions.sort();

        let mut positions = Rook::authorized_positions(
            &Coord::new(5, 2),
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }

    #[test]
    fn king_checked_cant_resolve() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Rook, PieceColor::White)),
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
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions: Vec<Coord> = vec![];
        right_positions.sort();

        let mut positions = Rook::authorized_positions(
            &Coord::new(5, 3),
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
    #[test]
    fn nailing() {
        let custom_board = [
            [
                None,
                None,
                None,
                None,
                Some((PieceType::King, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Rook, PieceColor::Black)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Queen, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::new(custom_board, PieceColor::Black, vec![]);
        board.set_board(custom_board);

        let is_king_checked =
            is_getting_checked(board.board, board.player_turn, &board.move_history);

        let mut right_positions: Vec<Coord> = vec![Coord::new(2, 4), Coord::new(3, 4)];
        right_positions.sort();

        let mut positions = Rook::authorized_positions(
            &Coord::new(1, 4),
            PieceColor::Black,
            board.board,
            &[],
            is_king_checked,
        );
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
