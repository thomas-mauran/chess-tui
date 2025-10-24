use super::coord::Coord;
use crate::pieces::{PieceMove, PieceType};
use shakmaty::{Chess, Color, Piece, Position};

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
    pub move_history: Vec<PieceMove>,
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
            match last_move.piece_type {
                PieceType::Pawn => "p".to_string(),
                PieceType::Knight => "n".to_string(),
                PieceType::Bishop => "b".to_string(),
                PieceType::Rook => "r".to_string(),
                PieceType::Queen => "q".to_string(),
                PieceType::King => "k".to_string(),
            }
        } else {
            String::new()
        }
    }

    pub fn increment_consecutive_non_pawn_or_capture(
        &mut self,
        role_from: PieceType,
        role_to: Option<PieceType>,
    ) {
        match (role_from, role_to) {
            (PieceType::Pawn, _) | (_, Some(_)) => {
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
    pub fn is_draw(&mut self, _player_turn: Color) -> bool {
        self.position().is_stalemate()
            || self.consecutive_non_pawn_or_capture == 50
            || self.is_draw_by_repetition()
            || self.position().is_insufficient_material()
    }

    pub fn is_latest_move_promotion(&self) -> bool {
        // Check if last move in our history was a pawn reaching the back rank
        if let Some(last_move) = self.move_history.last() {
            last_move.piece_type == PieceType::Pawn
                && (last_move.to.row == 0 || last_move.to.row == 7)
        } else {
            false
        }
    }

    /// Get piece type at a coordinate (handles flipped board)
    pub fn get_piece_type(&self, coord: &Coord) -> Option<PieceType> {
        if !coord.is_valid() {
            return None;
        }

        let actual_coord = if self.is_flipped {
            Coord::new(7 - coord.row, 7 - coord.col)
        } else {
            *coord
        };

        let square = actual_coord.to_square()?;
        let chess = self.position_history.last()?;
        chess
            .board()
            .piece_at(square)
            .map(|p| PieceType::from_role(p.role))
    }

    /// Get piece color at a coordinate (handles flipped board)
    pub fn get_piece_color(&self, coord: &Coord) -> Option<Color> {
        if !coord.is_valid() {
            return None;
        }

        let actual_coord = if self.is_flipped {
            Coord::new(7 - coord.row, 7 - coord.col)
        } else {
            *coord
        };

        let square = actual_coord.to_square()?;
        let chess = self.position_history.last()?;
        chess.board().piece_at(square).map(|p| p.color)
    }

    /// Get authorized positions for a piece at the given coordinate
    pub fn get_authorized_positions(&self, player_turn: Color, coord: Coord) -> Vec<Coord> {
        if !coord.is_valid() {
            return vec![];
        }

        // If board is flipped, we need to convert the coordinate back to standard orientation
        let actual_coord = if self.is_flipped {
            Coord::new(7 - coord.row, 7 - coord.col)
        } else {
            coord
        };

        let chess = self.position_history.last().unwrap();
        let from_square = match actual_coord.to_square() {
            Some(s) => s,
            None => return vec![],
        };

        // Check if there's a piece at this position and it's the right color
        if let Some(piece) = chess.board().piece_at(from_square) {
            if piece.color != player_turn {
                return vec![];
            }
        } else {
            return vec![];
        }

        // Get all legal moves
        let legal_moves = chess.legal_moves();
        let mut positions = Vec::new();

        for m in legal_moves {
            if m.from() == Some(from_square) {
                let mut target_coord = Coord::from_square(m.to());
                // If board is flipped, flip the target coordinates back
                if self.is_flipped {
                    target_coord = Coord::new(7 - target_coord.row, 7 - target_coord.col);
                }
                positions.push(target_coord);
            }
        }

        positions
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
    pub fn is_checkmate(&self, player_turn: Color) -> bool {
        let chess = self.position_history.last().unwrap();
        chess.is_checkmate() && chess.turn() == player_turn
    }

    /// Check if last move was castling
    pub fn is_latest_move_castling(&self, from: Coord, to: Coord) -> bool {
        if let Some(piece_type) = self.get_piece_type(&from) {
            if piece_type == PieceType::King {
                let distance = (from.col as i32 - to.col as i32).abs();
                return distance >= 2;
            }
        }
        false
    }

    /// Flip the board for alternating perspectives
    pub fn flip_the_board(&mut self) {
        // Just toggle the flipped flag - shakmaty board stays in standard orientation
        self.is_flipped = !self.is_flipped;
    }

    /// Get FEN position for UCI engine
    pub fn fen_position(&self, _is_bot_starting: bool, _player_turn: Color) -> String {
        let chess = self.position_history.last().unwrap();
        // Use FEN display from shakmaty
        use shakmaty::fen::Fen;
        let fen = Fen::from_position(chess.clone(), shakmaty::EnPassantMode::Legal);
        fen.to_string()
    }

    /// Get black taken pieces
    pub fn black_taken_pieces(&self) -> Vec<PieceType> {
        self.taken_pieces
            .iter()
            .filter(|p| p.color == Color::Black)
            .map(|p| PieceType::from_role(p.role))
            .collect()
    }

    /// Get white taken pieces
    pub fn white_taken_pieces(&self) -> Vec<PieceType> {
        self.taken_pieces
            .iter()
            .filter(|p| p.color == Color::White)
            .map(|p| PieceType::from_role(p.role))
            .collect()
    }

    /// Execute a move on the shakmaty Chess position and sync the visual board
    /// Optionally specify a promotion piece type
    pub fn execute_shakmaty_move(&mut self, from: Coord, to: Coord) -> bool {
        self.execute_shakmaty_move_with_promotion(from, to, None)
    }

    /// Execute a move from standard (non-flipped) coordinates
    /// Used for bot and opponent moves which come from external sources in standard notation
    pub fn execute_standard_move(
        &mut self,
        from: Coord,
        to: Coord,
        promotion: Option<PieceType>,
    ) -> bool {
        let from_square = match from.to_square() {
            Some(s) => s,
            None => return false,
        };

        let to_square = match to.to_square() {
            Some(s) => s,
            None => return false,
        };

        // Get current position
        let mut chess = self.position_history.last().unwrap().clone();

        // Check if there's a piece at the destination to track captures
        if let Some(captured_piece) = chess.board().piece_at(to_square) {
            self.taken_pieces.push(captured_piece);
        }

        // Find the legal move that matches
        let legal_moves = chess.legal_moves();
        let matching_move = legal_moves.iter().find(|m| {
            if m.from() == Some(from_square) && m.to() == to_square {
                // If a promotion is specified, make sure it matches
                if let Some(promo_type) = promotion {
                    if let Some(promo_role) = m.promotion() {
                        return promo_role == promo_type.to_role();
                    }
                    return false;
                }
                true
            } else {
                false
            }
        });

        if let Some(shakmaty_move) = matching_move {
            // Execute the move in shakmaty
            chess = chess.play(shakmaty_move).unwrap();

            // Update position history
            self.position_history.push(chess);

            true
        } else {
            false
        }
    }

    /// Execute a move with optional promotion
    pub fn execute_shakmaty_move_with_promotion(
        &mut self,
        from: Coord,
        to: Coord,
        promotion: Option<PieceType>,
    ) -> bool {
        // Convert coordinates to squares, accounting for flip
        let actual_from = if self.is_flipped {
            Coord::new(7 - from.row, 7 - from.col)
        } else {
            from
        };

        let actual_to = if self.is_flipped {
            Coord::new(7 - to.row, 7 - to.col)
        } else {
            to
        };

        let from_square = match actual_from.to_square() {
            Some(s) => s,
            None => return false,
        };

        let to_square = match actual_to.to_square() {
            Some(s) => s,
            None => return false,
        };

        // Get current position
        let mut chess = self.position_history.last().unwrap().clone();

        // Check if there's a piece at the destination to track captures
        if let Some(captured_piece) = chess.board().piece_at(to_square) {
            self.taken_pieces.push(captured_piece);
        }

        // Find the legal move that matches
        let legal_moves = chess.legal_moves();
        let matching_move = legal_moves.iter().find(|m| {
            if m.from() == Some(from_square) && m.to() == to_square {
                // If a promotion is specified, make sure it matches
                if let Some(promo_type) = promotion {
                    if let Some(promo_role) = m.promotion() {
                        return promo_role == promo_type.to_role();
                    }
                    // If we specified a promotion but move doesn't have one, skip
                    return false;
                }
                true
            } else {
                false
            }
        });

        if let Some(shakmaty_move) = matching_move {
            // Execute the move in shakmaty
            chess = chess.play(shakmaty_move).unwrap();

            // Update position history
            self.position_history.push(chess);

            true
        } else {
            false
        }
    }
}
