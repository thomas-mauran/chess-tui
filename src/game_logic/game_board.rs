use super::{
    board::{init_board, Board},
    coord::Coord,
    game::Game,
};
use crate::{
    pieces::{pawn::Pawn, PieceColor, PieceMove, PieceType},
    utils::col_to_letter,
};

/// ## visual representation
///
/// ### how it's stored:
///
/// . 0 1 2 3 4 5 6 7 .
/// 0 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 0
/// 1 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 1
/// 2 . . . . . . . . 2
/// 3 . . . . . . . . 3
/// 4 . . . . . . . . 4
/// 5 . . . . . . . . 5
/// 6 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 6
/// 7 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 7
/// . 0 1 2 3 4 5 6 7 .
///
/// ### how it's rendered:
///
/// . a b c d e f g h .
/// 8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 8
/// 7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 7
/// 6 . . . . . . . . 6
/// 5 . . . . . . . . 5
/// 4 . . . . . . . . 4
/// 3 . . . . . . . . 3
/// 2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 2
/// 1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 1
/// . a b c d e f g h .
/// only the pure gameboard, no additional information
///
#[derive(Debug, Clone)]
pub struct GameBoard {
    // the 8x8 board
    pub board: Board,
    // historic of the past Moves of the board
    pub move_history: Vec<PieceMove>,
    // historic of the past gameboards states
    pub board_history: Vec<Board>,
    // the number of consecutive non pawn or capture moves
    consecutive_non_pawn_or_capture: i32,
    // The white piece that got taken
    pub white_taken_pieces: Vec<PieceType>,
    // The black piece that got taken
    pub black_taken_pieces: Vec<PieceType>,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            board: init_board(),
            move_history: vec![],
            board_history: vec![init_board()],
            consecutive_non_pawn_or_capture: 0,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }
}

impl GameBoard {
    pub fn new(board: Board, move_history: Vec<PieceMove>, board_history: Vec<Board>) -> Self {
        Self {
            board,
            move_history,
            board_history,
            consecutive_non_pawn_or_capture: 0,
            white_taken_pieces: vec![],
            black_taken_pieces: vec![],
        }
    }

    pub fn get_last_move_piece_type_as_string(&self) -> String {
        if let Some(last_move) = self.move_history.last() {
            match last_move.piece_type {
                PieceType::Pawn => return String::from("p"),
                PieceType::Rook => return String::from("r"),
                PieceType::Knight => return String::from("n"),
                PieceType::Bishop => return String::from("b"),
                PieceType::Queen => return String::from("q"),
                PieceType::King => return String::from("k"),
            }
        }
        String::from("")
    }

    pub fn increment_consecutive_non_pawn_or_capture(
        &mut self,
        piece_type_from: PieceType,
        piece_type_to: Option<PieceType>,
    ) {
        match (piece_type_from, piece_type_to) {
            (PieceType::Pawn, _) | (_, Some(_)) => {
                self.set_consecutive_non_pawn_or_capture(0);
            }
            _ => {
                let value = self.get_consecutive_non_pawn_or_capture() + 1;
                self.set_consecutive_non_pawn_or_capture(value);
            }
        }
    }

    pub fn add_piece_to_taken_pieces(&mut self, from: &Coord, to: &Coord, player_turn: PieceColor) {
        if self.is_latest_move_en_passant(from, to) {
            self.push_to_taken_piece(PieceType::Pawn, player_turn.opposite());
        }

        let piece_type_to = self.get_piece_type(to);
        let piece_color = self.get_piece_color(to);
        // We check if there is a piece and we are not doing a castle
        if piece_color.is_some()
            && piece_type_to.is_some()
            && (piece_type_to != Some(PieceType::Rook) && piece_color != Some(player_turn))
        {
            if let Some(piece_type) = piece_type_to {
                self.push_to_taken_piece(piece_type, piece_color.unwrap())
            }
        }
    }

    pub fn push_to_taken_piece(&mut self, piece_type: PieceType, piece_color: PieceColor) {
        match piece_color {
            PieceColor::Black => {
                self.white_taken_pieces.push(piece_type);
                self.white_taken_pieces.sort();
            }
            PieceColor::White => {
                self.black_taken_pieces.push(piece_type);
                self.black_taken_pieces.sort();
            }
        }
    }

    pub fn reset(&mut self) {
        self.board = init_board();
        self.move_history.clear();
        self.board_history.clear();
        self.board_history.push(init_board());
        self.consecutive_non_pawn_or_capture = 0;
    }

    // Method to get the authorized positions for a piece
    pub fn get_authorized_positions(
        &self,
        player_turn: PieceColor,
        coordinates: Coord,
    ) -> Vec<Coord> {
        if let (Some(piece_type), Some(piece_color)) = (
            self.get_piece_type(&coordinates),
            self.get_piece_color(&coordinates),
        ) {
            // If the piece color is not the same as the player turn we return an empty vector it's not his turn
            if player_turn != piece_color {
                return vec![];
            }

            piece_type.authorized_positions(
                &coordinates,
                piece_color,
                self,
                self.is_getting_checked(self.board, player_turn),
            )
        } else {
            vec![]
        }
    }

    // Method use to flip the board pieces (for the black player)
    pub fn flip_the_board(&mut self) {
        let mut flipped_board = [[None; 8]; 8]; // Create a new empty board of the same type

        for (i, row) in self.board.iter().enumerate() {
            for (j, &square) in row.iter().enumerate() {
                // Place each square in the mirrored position
                flipped_board[7 - i][7 - j] = square;
            }
        }
        self.board = flipped_board;
    }

    // Check if the latest move is en passant
    pub fn is_latest_move_en_passant(&self, from: &Coord, to: &Coord) -> bool {
        let piece_type_from = self.get_piece_type(from);
        let piece_type_to = self.get_piece_type(to);

        let from_y: i32 = from.row as i32;
        let from_x: i32 = from.col as i32;
        let to_y: i32 = to.row as i32;
        let to_x: i32 = to.col as i32;
        match (piece_type_from, piece_type_to) {
            (Some(PieceType::Pawn), _) => {
                // Check if it's a diagonal move, and the destination is an empty cell
                from_y != to_y && from_x != to_x && self.board[to].is_none()
            }
            _ => false,
        }
    }

    // Check if the latest move is castling
    pub fn is_latest_move_castling(&self, from: Coord, to: Coord) -> bool {
        let piece_type_from = self.get_piece_type(&from);
        let piece_type_to = self.get_piece_type(&to);

        let from_x: i32 = from.col as i32;
        let to_x: i32 = to.col as i32;
        let distance = (from_x - to_x).abs();

        match (piece_type_from, piece_type_to) {
            (Some(PieceType::King), _) => distance > 1,
            _ => false,
        }
    }

    // Check if the latest move is a promotion
    pub fn is_latest_move_promotion(&self) -> bool {
        if let Some(last_move) = self.move_history.last() {
            if let Some(piece_type_to) =
                self.get_piece_type(&Coord::new(last_move.to.row, last_move.to.col))
            {
                let last_row = 0;
                if last_move.to.row == last_row && piece_type_to == PieceType::Pawn {
                    return true;
                }
            }
        }
        false
    }

    // Method to get the number of authorized positions for the current player (used for the end condition)
    pub fn number_of_authorized_positions(&self, player_turn: PieceColor) -> usize {
        let mut possible_moves: Vec<Coord> = vec![];

        for i in 0..8 {
            for j in 0..8 {
                let coord = Coord::new(i, j);
                if let Some((_piece_type, piece_color)) = self.board[&coord] {
                    if piece_color == player_turn {
                        possible_moves.extend(self.get_authorized_positions(player_turn, coord));
                    }
                }
            }
        }
        possible_moves.len()
    }

    // Check if the game is checkmate
    pub fn is_checkmate(&self, player_turn: PieceColor) -> bool {
        if !self.is_getting_checked(self.board, player_turn) {
            return false;
        }

        self.number_of_authorized_positions(player_turn) == 0
    }

    // Check if the game is a draw
    pub fn is_draw_by_repetition(&mut self) -> bool {
        // A new game has started
        if self.move_history.is_empty() {
            self.board_history.clear();
            self.board_history.push(self.board);
            return false;
        }

        // Index mapping
        let mut position_counts = std::collections::HashMap::new();
        for board in self.board_history.iter() {
            let count = position_counts.entry(board).or_insert(0);
            *count += 1;

            if *count >= 3 {
                return true;
            }
        }

        false
    }

    // Check if the game is a draw
    pub fn is_draw(&mut self, player_turn: PieceColor) -> bool {
        self.number_of_authorized_positions(player_turn) == 0
            || self.consecutive_non_pawn_or_capture == 50
            || self.is_draw_by_repetition()
    }

    pub fn set_consecutive_non_pawn_or_capture(&mut self, value: i32) {
        self.consecutive_non_pawn_or_capture = value;
    }

    pub fn get_consecutive_non_pawn_or_capture(&self) -> i32 {
        self.consecutive_non_pawn_or_capture
    }

    /// We get all the cells that are getting put in 'check'
    pub fn get_all_protected_cells(&self, player_turn: PieceColor) -> Vec<Coord> {
        let mut check_cells: Vec<Coord> = vec![];
        for i in 0..8u8 {
            for j in 0..8u8 {
                if self.get_piece_color(&Coord::new(i, j)) == Some(player_turn) {
                    continue;
                }
                // get the current cell piece color and type protecting positions
                if let Some(piece_color) = self.get_piece_color(&Coord::new(i, j)) {
                    if let Some(piece_type) = self.get_piece_type(&Coord::new(i, j)) {
                        check_cells.extend(PieceType::protected_positions(
                            &Coord::new(i, j),
                            piece_type,
                            piece_color,
                            self,
                        ));
                    }
                }
            }
        }
        check_cells
    }

    /// Method returning the coordinates of the king of a certain color
    pub fn get_king_coordinates(&self, board: Board, player_turn: PieceColor) -> Coord {
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
    /// Here we keep the board as one of the parameters because for the king position we need to simulate the board if he moves
    /// to make sure he will not be checked after the move
    pub fn is_getting_checked(&self, board: Board, player_turn: PieceColor) -> bool {
        let coordinates = self.get_king_coordinates(board, player_turn);

        let fake_game_board = GameBoard {
            board,
            move_history: self.move_history.clone(),
            board_history: self.board_history.clone(),
            consecutive_non_pawn_or_capture: self.consecutive_non_pawn_or_capture,
            white_taken_pieces: self.white_taken_pieces.clone(),
            black_taken_pieces: self.black_taken_pieces.clone(),
        };

        let checked_cells = fake_game_board.get_all_protected_cells(player_turn);

        checked_cells.contains(&coordinates)
    }

    /// Check if a piece already moved on the board
    pub fn did_piece_already_move(
        &self,
        original_piece: (Option<PieceType>, Option<PieceColor>, Coord),
    ) -> bool {
        for entry in &self.move_history {
            if Some(entry.piece_type) == original_piece.0
                && Some(entry.piece_color) == original_piece.1
                && entry.from == original_piece.2
            {
                return true;
            }
        }
        false
    }

    // Get all the positions where the king can't go because it's checked
    pub fn impossible_positions_king_checked(
        &self,
        original_coordinates: &Coord,
        positions: Vec<Coord>,
        color: PieceColor,
    ) -> Vec<Coord> {
        let mut cleaned_position: Vec<Coord> = vec![];
        for position in positions {
            let game = GameBoard::new(self.board, self.move_history.to_vec(), vec![]);

            // We create a new board
            let mut new_board = Game::new(game, color);

            // We simulate the move

            Game::execute_move(&mut new_board, original_coordinates, &position);

            // We check if the board is still checked with this move meaning it didn't resolve the problem
            if !self.is_getting_checked(new_board.game_board.board, new_board.player_turn) {
                cleaned_position.push(position);
            };
        }
        cleaned_position
    }

    // Return the color of the piece at a certain position
    pub fn get_piece_color(&self, coordinates: &Coord) -> Option<PieceColor> {
        if !coordinates.is_valid() {
            return None;
        }
        self.board[coordinates].map(|(_, piece_color)| piece_color)
    }

    pub fn get_piece_type(&self, coordinates: &Coord) -> Option<PieceType> {
        if !coordinates.is_valid() {
            return None;
        }
        self.board[coordinates].map(|(piece_type, _)| piece_type)
    }

    // Convert the history and game status to a FEN string
    pub fn fen_position(&mut self, is_bot_starting: bool, player_turn: PieceColor) -> String {
        let mut result = String::new();
        let bot_color = if is_bot_starting {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        let king_col = if bot_color == PieceColor::White { 4 } else { 3 };

        // We loop through the board and convert it to a FEN string
        for i in 0..8u8 {
            for j in 0..8u8 {
                // We get the piece type and color
                let (piece_type, piece_color) = (
                    self.get_piece_type(&Coord::new(i, j)),
                    self.get_piece_color(&Coord::new(i, j)),
                );
                let letter = PieceType::piece_to_fen_enum(piece_type, piece_color);
                // Pattern match directly on the result of piece_to_fen_enum
                match letter {
                    "" => {
                        // Check if the string is not empty before using chars().last()
                        if let Some(last_char) = result.chars().last() {
                            if last_char.is_ascii_digit() {
                                let incremented_char =
                                    char::from_digit(last_char.to_digit(10).unwrap_or(0) + 1, 10)
                                        .unwrap_or_default();
                                // Remove the old number and add the new incremented one
                                result.pop();
                                result.push_str(incremented_char.to_string().as_str());
                            } else {
                                result.push('1');
                            }
                        } else {
                            result.push('1');
                        }
                    }
                    letter => {
                        // If the result is not an empty string, push '1'
                        result.push_str(letter);
                    }
                };
            }
            result.push('/');
        }

        // we remove the last / and specify the player turn (black)
        result.pop();

        // We say it is blacks turn to play
        result.push_str(if bot_color == PieceColor::Black {
            " b"
        } else {
            " w"
        });

        // We add the castles availabilities for black
        if !self.did_piece_already_move((
            Some(PieceType::King),
            Some(player_turn),
            Coord::new(7, king_col),
        )) && !self.is_getting_checked(self.board, PieceColor::Black)
        {
            // king side black castle availability
            if !self.did_piece_already_move((
                Some(PieceType::Rook),
                Some(player_turn),
                Coord::new(7, 7),
            )) {
                result.push_str(" k");
            }
            // queen side black castle availability
            if !self.did_piece_already_move((
                Some(PieceType::Rook),
                Some(player_turn),
                Coord::new(7, 0),
            )) {
                result.push('q');
            }
        } else {
            result.push_str(" -");
        }

        // We check if the latest move is a pawn moving 2 cells, meaning the next move can be en passant
        if Pawn::did_pawn_move_two_cells(self.move_history.last()) {
            // Use an if-let pattern for better readability
            if let Some(last_move) = self.move_history.last() {
                let mut converted_move = String::new();

                converted_move += &col_to_letter(last_move.from.col);
                // FEN starts counting from 1 not 0
                converted_move += &format!("{}", 8 - last_move.from.row + 1).to_string();

                result.push(' ');
                result.push_str(&converted_move);
            }
        } else {
            result.push_str(" -");
        }

        result.push(' ');

        result.push_str(&self.get_consecutive_non_pawn_or_capture().to_string());
        result.push(' ');

        result.push_str(&(self.move_history.len() / 2).to_string());

        result
    }
}
