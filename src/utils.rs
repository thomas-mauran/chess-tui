use crate::{
    constants::{DisplayMode, UNDEFINED_POSITION},
    game::game::{Coord, Game, GameBoard},
    pieces::{PieceColor, PieceMove, PieceType},
};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph},
};

pub fn get_piece_color(board: GameBoard, coordinates: &Coord) -> Option<PieceColor> {
    if !coordinates.is_valid() {
        return None;
    }
    board[coordinates].map(|(_, piece_color)| piece_color)
}

pub fn get_piece_type(board: GameBoard, coordinates: &Coord) -> Option<PieceType> {
    if !coordinates.is_valid() {
        return None;
    }
    board[coordinates].map(|(piece_type, _)| piece_type)
}

/// method to clean `positions`: remove impossible positions
pub fn cleaned_positions(positions: &[Coord]) -> Vec<Coord> {
    positions
        .iter()
        .filter(|position| position.is_valid())
        .copied()
        .collect()
}

/// Return true forally cell color; false for enemy
pub fn is_cell_color_ally(board: GameBoard, coordinates: &Coord, color: PieceColor) -> bool {
    match get_piece_color(board, coordinates) {
        Some(cell_color) => cell_color == color,
        None => false, // Treat empty cell as ally
    }
}

/// We get all the cells that are getting put in 'check'
pub fn get_all_protected_cells(
    board: GameBoard,
    player_turn: PieceColor,
    move_history: &[PieceMove],
) -> Vec<Coord> {
    let mut check_cells: Vec<Coord> = vec![];
    for i in 0..8u8 {
        for j in 0..8u8 {
            if get_piece_color(board, &Coord::new(i, j)) == Some(player_turn) {
                continue;
            }
            // get the current cell piece color and type protecting positions
            if let Some(piece_color) = get_piece_color(board, &Coord::new(i, j)) {
                if let Some(piece_type) = get_piece_type(board, &Coord::new(i, j)) {
                    check_cells.extend(PieceType::protected_positions(
                        &Coord::new(i, j),
                        piece_type,
                        piece_color,
                        board,
                        move_history,
                    ));
                }
            }
        }
    }
    check_cells
}

pub fn col_to_letter(col: u8) -> String {
    match col {
        0 => "a".to_string(),
        1 => "b".to_string(),
        2 => "c".to_string(),
        3 => "d".to_string(),
        4 => "e".to_string(),
        5 => "f".to_string(),
        6 => "g".to_string(),
        7 => "h".to_string(),
        _ => unreachable!("Col out of bound {}", col),
    }
}

pub fn letter_to_col(col: Option<char>) -> i8 {
    match col {
        Some('a') => 0,
        Some('b') => 1,
        Some('c') => 2,
        Some('d') => 3,
        Some('e') => 4,
        Some('f') => 5,
        Some('g') => 6,
        Some('h') => 7,
        _ => unreachable!("Col out of bound"),
    }
}

pub fn convert_position_into_notation(position: &str) -> String {
    let from_y = get_int_from_char(position.chars().next());
    let from_x = get_int_from_char(position.chars().nth(1));
    let to_y = get_int_from_char(position.chars().nth(2));
    let to_x = get_int_from_char(position.chars().nth(3));

    let from_x = col_to_letter(from_x);
    let to_x = col_to_letter(to_x);

    format!("{from_x}{}-{to_x}{}", 8 - from_y, 8 - to_y)
}

pub fn convert_notation_into_position(notation: &str) -> String {
    let from_x = &letter_to_col(notation.chars().next());
    let from_y = (get_int_from_char(notation.chars().nth(1)) as i8 - 8).abs();

    let to_x = &letter_to_col(notation.chars().nth(2));
    let to_y = (get_int_from_char(notation.chars().nth(3)) as i8 - 8).abs();

    format!("{from_y}{from_x}{to_y}{to_x}")
}

pub fn get_int_from_char(ch: Option<char>) -> u8 {
    match ch {
        Some(ch) => ch.to_digit(10).unwrap() as u8,
        _ => UNDEFINED_POSITION,
    }
}

pub fn did_piece_already_move(
    move_history: &[PieceMove],
    original_piece: (Option<PieceType>, Option<PieceColor>, Coord),
) -> bool {
    for entry in move_history {
        if Some(entry.piece_type) == original_piece.0
            && Some(entry.piece_color) == original_piece.1
            && entry.from == original_piece.2
        {
            return true;
        }
    }
    false
}
/// Method returning the coordinates of the king of a certain color
pub fn get_king_coordinates(board: GameBoard, player_turn: PieceColor) -> Coord {
    for i in 0..8u8 {
        for j in 0..8u8 {
            if let Some((piece_type, piece_color)) = board[i as usize][j as usize] {
                if piece_type == PieceType::King && piece_color == player_turn {
                    return Coord::new(i, j);
                }
            }
        }
    }
    Coord::undefined()
}

/// Is getting checked
pub fn is_getting_checked(
    board: GameBoard,
    player_turn: PieceColor,
    move_history: &[PieceMove],
) -> bool {
    let coordinates = get_king_coordinates(board, player_turn);

    let checked_cells = get_all_protected_cells(board, player_turn, move_history);

    checked_cells.contains(&coordinates)
}

pub fn impossible_positions_king_checked(
    original_coordinates: &Coord,
    positions: Vec<Coord>,
    board: GameBoard,
    color: PieceColor,
    move_history: &[PieceMove],
) -> Vec<Coord> {
    let mut cleaned_position: Vec<Coord> = vec![];
    for position in positions {
        // We create a new board
        let mut new_board = Game::new(board, color, move_history.to_owned().clone());

        // We simulate the move

        Game::move_piece_on_the_board(&mut new_board, original_coordinates, &position);

        // We check if the board is still checked with this move meaning it didn't resolve the problem
        if !is_getting_checked(
            new_board.board,
            new_board.player_turn,
            &new_board.move_history,
        ) {
            cleaned_position.push(position);
        };
    }
    cleaned_position
}

pub fn is_piece_opposite_king(piece: Option<(PieceType, PieceColor)>, color: PieceColor) -> bool {
    match piece {
        Some((piece_type, piece_color)) => {
            piece_type == PieceType::King && piece_color == color.opposite()
        }
        _ => false,
    }
}

pub fn color_to_ratatui_enum(piece_color: Option<PieceColor>) -> Color {
    match piece_color {
        Some(PieceColor::Black) => Color::Black,
        Some(PieceColor::White) => Color::White,
        None => Color::Red,
    }
}

pub fn get_cell_paragraph<'a>(
    board: &'a Game,
    cell_coordinates: &'a Coord,
    bounding_rect: Rect,
) -> Paragraph<'a> {
    // Get piece and color
    let piece_color = get_piece_color(board.board, cell_coordinates);
    let piece_type = get_piece_type(board.board, cell_coordinates);
    let piece_enum = PieceType::piece_type_to_string_enum(piece_type, &board.display_mode);

    let paragraph = match board.display_mode {
        DisplayMode::DEFAULT => {
            let color_enum = color_to_ratatui_enum(piece_color);

            // Place the pieces on the board
            Paragraph::new(piece_enum).fg(color_enum)
        }
        DisplayMode::ASCII => {
            // Determine piece letter case
            let paragraph = match piece_color {
                // pieces belonging to the player on top will be lower case
                Some(PieceColor::Black) => Paragraph::new(piece_enum.to_lowercase()),
                // pieces belonging to the player on bottom will be upper case
                Some(PieceColor::White) => Paragraph::new(piece_enum.to_uppercase().underlined()),
                // Pass through original value
                None => Paragraph::new(piece_enum),
            };

            // Place the pieces on the board
            paragraph.block(Block::new().padding(Padding::vertical(bounding_rect.height / 2)))
        }
    };

    paragraph.alignment(Alignment::Center)
}

pub fn invert_position(coord: &Coord) -> Coord {
    Coord::new(7 - coord.row, 7 - coord.col)
}
