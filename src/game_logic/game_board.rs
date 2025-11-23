use super::coord::Coord;
use shakmaty::{san::San, Chess, Color, Move, Piece, Position, Rank, Role, Square};

/// ## visual representation
///
/// ### how it's stored:
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
    /// Track if the board is currently flipped (for coordinate conversion)
    pub is_flipped: bool,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            move_history: Vec::new(),
            position_history: vec![Chess::default()],
            consecutive_non_pawn_or_capture: 0,
            taken_pieces: Vec::new(),
            is_flipped: false,
        }
    }
}

impl GameBoard {
    pub fn get_last_move_piece_type_as_string(&self) -> String {
        if let Some(last_move) = self.move_history.last() {
            match last_move.role() {
                Role::Pawn => "p".to_string(),
                Role::Knight => "n".to_string(),
                Role::Bishop => "b".to_string(),
                Role::Rook => "r".to_string(),
                Role::Queen => "q".to_string(),
                Role::King => "k".to_string(),
            }
        } else {
            String::new()
        }
    }

    /// Convert a move to Standard Algebraic Notation (e.g., "e4", "Nf3", "O-O", "Qxd5+")
    pub fn move_to_san(&self, move_index: usize) -> String {
        if move_index >= self.move_history.len() || move_index >= self.position_history.len() {
            return String::new();
        }

        // Get the position before this move was made
        let position = &self.position_history[move_index];
        let chess_move = &self.move_history[move_index];

        // Convert to SAN using shakmaty
        let san = San::from_move(position, chess_move);
        san.to_string()
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
        self.position_history.push(Chess::default());
        self.consecutive_non_pawn_or_capture = 0;
        self.is_flipped = false;
    }

    /// Gets the last position in the history.
    pub fn position(&mut self) -> &mut Chess {
        self.position_history.last_mut().unwrap()
    }

    /// Gets a read-only reference to the last position in the history.
    pub fn position_ref(&self) -> &Chess {
        self.position_history.last().unwrap()
    }

    /// Gets a read-only reference to the current position, or None if history is empty
    pub fn current_position(&self) -> Option<&Chess> {
        self.position_history.last()
    }

    /// Gets a mutable reference to the current position, or None if history is empty
    pub fn current_position_mut(&mut self) -> Option<&mut Chess> {
        self.position_history.last_mut()
    }

    // Check if the game is a draw by repetition
    pub fn is_draw_by_repetition(&self) -> bool {
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
    pub fn is_draw(&self, _player_turn: Color) -> bool {
        self.position_ref().is_stalemate()
            || self.consecutive_non_pawn_or_capture == 50
            || self.is_draw_by_repetition()
            || self.position_ref().is_insufficient_material()
    }

    // We check manually if the last move was a pawn to one of the promotion squares
    pub fn is_latest_move_promotion(&self) -> bool {
        if let Some(last_move) = self.move_history.last() {
            // we check manually if the last move was a pawn to one of the promotion squares
            if last_move.role() == Role::Pawn {
                let rank = last_move.to().rank();
                rank == Rank::First || rank == Rank::Eighth
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get piece type at a coordinate (handles flipped board)
    pub fn get_role_at_square(&self, square: &Square) -> Option<Role> {
        let piece = self
            .position_history
            .last()
            .unwrap()
            .board()
            .piece_at(*square);
        piece.map(|piece| piece.role)
    }

    pub fn get_current_chess(&self) -> Chess {
        let chess = self.position_history.last().unwrap().clone();
        chess
    }

    pub fn is_square_occupied(&self, square: &Square) -> bool {
        let board = self.position_history.last().unwrap().board().clone();
        board.piece_at(*square).is_some()
    }

    pub fn get_piece_color_at_square(&self, square: &Square) -> Option<Color> {
        let piece = self.position_ref().board().piece_at(*square);
        piece.map(|p| p.color)
    }

    /// Get authorized positions for a piece at the given coordinate
    pub fn get_authorized_positions(&self, player_turn: Color, square: &Square) -> Vec<Square> {
        let board = self.position_history.last().unwrap().board().clone();

        // Check if there's a piece at this position and it's the right color
        if let Some(piece) = board.piece_at(*square) {
            if piece.color != player_turn {
                return vec![];
            }
        } else {
            return vec![];
        }

        // Get all legal moves
        self.get_current_chess()
            .legal_moves()
            .iter()
            .filter(|m| m.from() == Some(*square))
            .map(|m| m.to())
            .collect()
    }

    /// Check if the king is in check
    pub fn is_getting_checked(&self, player_turn: Color) -> bool {
        let chess = self.position_history.last().unwrap();
        chess.is_check() && chess.turn() == player_turn
    }

    /// Get the king's coordinates
    pub fn get_king_coordinates(&self, color: Color) -> Coord {
        let chess = self.position_history.last().unwrap();
        let king_square = chess.board().king_of(color).unwrap();
        let mut coord = Coord::from_square(king_square);

        // If board is flipped, flip the coordinate for display
        if self.is_flipped {
            coord = Coord::new(7 - coord.row, 7 - coord.col);
        }

        coord
    }

    /// Check if game is checkmate
    pub fn is_checkmate(&self) -> bool {
        let chess = self.get_current_chess();
        chess.is_checkmate()
    }

    /// Check if last move was castling
    pub fn is_latest_move_castling(&self) -> bool {
        let chess = self.move_history.last().unwrap();
        chess.is_castle()
    }

    /// Flip the board for alternating perspectives
    pub fn flip_the_board(&mut self) {
        self.is_flipped = !self.is_flipped;
    }

    /// Get FEN position for UCI engine
    pub fn fen_position(&self, _is_bot_starting: bool, _player_turn: Color) -> String {
        let chess = self.get_current_chess();
        // Use FEN display from shakmaty
        use shakmaty::fen::Fen;
        let fen = Fen::from_position(chess, shakmaty::EnPassantMode::Legal);
        fen.to_string()
    }

    /// Get black taken pieces
    pub fn black_taken_pieces(&self) -> Vec<Role> {
        self.taken_pieces
            .iter()
            .filter(|p| p.color == Color::Black)
            .map(|p| p.role)
            .collect()
    }

    /// Get white taken pieces
    pub fn white_taken_pieces(&self) -> Vec<Role> {
        self.taken_pieces
            .iter()
            .filter(|p| p.color == Color::White)
            .map(|p| p.role)
            .collect()
    }

    /// Execute a move on the board
    /// Returns the executed Move if successful, None if illegal
    pub fn execute_move(
        &mut self,
        from: Square,
        to: Square,
        promotion: Option<Role>,
    ) -> Option<Move> {
        let mut chess = self.get_current_chess();

        // Track captures before executing move
        if let Some(captured_piece) = chess.board().piece_at(to) {
            self.taken_pieces.push(captured_piece);
        }

        // Find matching legal move
        let legal_moves = chess.legal_moves();
        let matching_move = legal_moves.iter().find(|m| {
            m.from() == Some(from) && m.to() == to && {
                match (promotion, m.promotion()) {
                    (Some(promo), Some(move_promo)) => promo == move_promo,
                    (None, None) => true,
                    (None, Some(_)) => true, // Allow promotion moves without specifying
                    (Some(_), None) => false, // Reject if we expect promotion but move has none
                }
            }
        });

        if let Some(shakmaty_move) = matching_move {
            // Execute move
            chess = chess.play(shakmaty_move).unwrap();

            // Update history
            self.position_history.push(chess);

            Some(shakmaty_move.clone())
        } else {
            None
        }
    }

    // Execute a move on the shakmaty Chess position and sync the visual board
    // Optionally specify a promotion piece type
    pub fn execute_shakmaty_move(&mut self, from: Square, to: Square) -> Option<Move> {
        self.execute_move(from, to, None)
    }

    /// Execute a move from standard (non-flipped) coordinates
    /// Used for bot and opponent moves which come from external sources in standard notation
    pub fn execute_standard_move(
        &mut self,
        from: Square,
        to: Square,
        promotion: Option<Role>,
    ) -> Option<Move> {
        self.execute_move(from, to, promotion)
    }

    /// Execute a move with optional promotion
    pub fn execute_shakmaty_move_with_promotion(
        &mut self,
        from: Square,
        to: Square,
        promotion: Option<Role>,
    ) -> bool {
        self.execute_move(from, to, promotion).is_some()
    }
}
