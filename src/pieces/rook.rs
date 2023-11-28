use super::{PieceColor, PieceType, Movable, Position};
use crate::utils::{cleaned_positions, get_piece_color, is_cell_color_ally, is_valid};

pub struct Rook;

impl Movable for Rook {
    fn piece_move(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        _latest_move: Option<(Option<PieceType>, i32)>,
    ) -> Vec<Vec<i8>> {
        // Pawns can only move in one direction depending on their color
        let mut positions: Vec<Vec<i8>> = vec![];

        let (y, x) = (coordinates[0], coordinates[1]);

        // RIGHT ROW
        for i in 1..8i8 {
            let new_x = x + i;
            let new_y = y;
            let new_coordinates = [new_y, new_x];

            // Invalid coords
            if !is_valid(new_coordinates) {
                break;
            }

            // Empty cell
            if get_piece_color(board, new_coordinates).is_none() {
                positions.push(new_coordinates.to_vec());
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates.to_vec());
                    break;
                }
            }
            // Enemy cell
            positions.push(new_coordinates.to_vec());
            break;
        }

        // LEFT ROW
        for i in 1..8i8 {
            let new_x = x - i;
            let new_y = y;
            let new_coordinates = [new_y, new_x];

            // Invalid coords
            if !is_valid(new_coordinates) {
                break;
            }

            // Empty cell
            if get_piece_color(board, new_coordinates).is_none() {
                positions.push(new_coordinates.to_vec());
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates.to_vec());
                    break;
                }
            }
            // Enemy cell
            positions.push(new_coordinates.to_vec());
            break;
        }

        // BOTTOM ROW
        for i in 1..8i8 {
            let new_x = x;
            let new_y = y + i;
            let new_coordinates = [new_y, new_x];

            // Invalid coords
            if !is_valid(new_coordinates) {
                break;
            }

            // Empty cell
            if get_piece_color(board, new_coordinates).is_none() {
                positions.push(new_coordinates.to_vec());
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates.to_vec());
                    break;
                }
            }
            // Enemy cell
            positions.push(new_coordinates.to_vec());
            break;
        }

        // UP ROW
        for i in 1..8i8 {
            let new_x = x;
            let new_y = y - i;
            let new_coordinates = [new_y, new_x];

            // Invalid coords
            if !is_valid(new_coordinates) {
                break;
            }

            // Empty cell
            if get_piece_color(board, new_coordinates).is_none() {
                positions.push(new_coordinates.to_vec());
                continue;
            }
            // Ally cell
            if is_cell_color_ally(board, new_coordinates, color) {
                if !allow_move_on_ally_positions {
                    break;
                } else {
                    positions.push(new_coordinates.to_vec());
                    break;
                }
            }
            // Enemy cell
            positions.push(new_coordinates.to_vec());
            break;
        }

        cleaned_positions(positions)
    }
}

impl Position for Rook{
    fn authorized_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        _latest_move: Option<(Option<PieceType>, i32)>,
    ) -> Vec<Vec<i8>> {
        Self::piece_move(coordinates, color, board, false, None)
    }

    fn protected_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        _latest_move: Option<(Option<PieceType>, i32)>,
    ) -> Vec<Vec<i8>> {
        Self::piece_move(coordinates, color, board, true, None)
    }
}

impl Rook {
    pub fn to_string() -> &'static str {
        "\
    \n\
    █▟█▙█\n\
    ▜███▛\n\
    ▐███▌\n\
   ▗█████▖\n\
    "
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{rook::Rook, PieceColor, PieceType, Position},
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
            vec![7, 4],
            vec![6, 4],
            vec![5, 4],
            vec![3, 4],
            vec![2, 4],
            vec![1, 4],
            vec![0, 4],
            vec![4, 0],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
            vec![4, 5],
            vec![4, 6],
            vec![4, 7],
        ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions([4, 4], PieceColor::White, board.board, None);
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
            vec![7, 4],
            vec![6, 4],
            vec![5, 4],
            vec![3, 4],
            vec![4, 0],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
            vec![4, 5],
            vec![4, 6],
            vec![4, 7],
        ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions([4, 4], PieceColor::White, board.board, None);
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
            vec![4, 0],
            vec![4, 1],
            vec![4, 2],
            vec![4, 3],
            vec![4, 5],
            vec![4, 6],
            vec![3, 4],
            vec![5, 4],
        ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions([4, 4], PieceColor::White, board.board, None);
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}
