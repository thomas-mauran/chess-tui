use shakmaty::{ByColor, Chess, Move, Piece, Position, Role};

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
    // historic of the past Moves of the board
    pub move_history: Vec<Move>,
    /// historic of the past gameboards states.
    /// The last position is the current position.
    pub position_history: Vec<Chess>,
    // the number of consecutive non pawn or capture moves
    pub consecutive_non_pawn_or_capture: i32,
    pub taken_pieces: Vec<Piece>,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            move_history: Vec::new(),
            position_history: vec![Chess::new()],
            consecutive_non_pawn_or_capture: 0,
            taken_pieces: Vec::new(),
        }
    }
}

impl GameBoard {
    pub fn new(move_history: Vec<Move>, position_history: Vec<Chess>) -> Self {
        Self {
            move_history,
            position_history,
            consecutive_non_pawn_or_capture: 0,
            taken_pieces: Vec::new(),
        }
    }

    pub fn get_last_move_role_as_string(&self) -> &'static str {
        if let Some(last_move) = self.move_history.last() {
            match last_move.role() {
                Role::Pawn => "p",
                Role::Knight => "n",
                Role::Bishop => "b",
                Role::Rook => "r",
                Role::Queen => "q",
                Role::King => "k",
            }
        } else {
            ""
        }
    }

    pub fn increment_consecutive_non_pawn_or_capture(
        &mut self,
        role_from: Role,
        role_to: Option<Role>,
    ) {
        match (role_from, role_to) {
            (Role::Pawn, _) | (_, Some(_)) => {
                self.consecutive_non_pawn_or_capture = 0;
            }
            _ => {
                self.consecutive_non_pawn_or_capture += 1;
            }
        }
    }

    pub fn reset(&mut self) {
        self.move_history.clear();
        self.position_history.clear();
        self.position_history.push(Chess::new());
        self.consecutive_non_pawn_or_capture = 0;
    }

    /// Gets the last position in the history.
    pub fn position(&mut self) -> &mut Chess {
        self.position_history.last_mut().unwrap()
    }

    // Check if the game is a draw
    pub fn is_draw_by_repetition(&mut self) -> bool {
        // Index mapping
        let mut position_counts = std::collections::HashMap::new();
        for board in self.position_history.iter() {
            let count = position_counts.entry(board).or_insert(0);
            *count += 1;

            if *count >= 3 {
                return true;
            }
        }

        false
    }

    // Check if the game is a draw
    pub fn is_draw(&mut self) -> bool {
        self.position().is_stalemate()
            || self.consecutive_non_pawn_or_capture == 50
            || self.is_draw_by_repetition()
    }

    pub fn is_last_move_promotion(&self) -> bool {
        if let Some(last_move) = self.move_history.last() {
            last_move.is_promotion()
        } else {
            false
        }
    }
}
