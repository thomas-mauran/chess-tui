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
    /// Current position index in history when navigating. None means viewing the latest position.
    pub history_position_index: Option<usize>,
    /// Original flip state before navigating history (used to restore when returning to latest)
    pub original_flip_state: Option<bool>,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self {
            move_history: Vec::new(),
            position_history: vec![Chess::default()],
            consecutive_non_pawn_or_capture: 0,
            taken_pieces: Vec::new(),
            is_flipped: false,
            history_position_index: None,
            original_flip_state: None,
        }
    }
}

impl GameBoard {
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
        self.history_position_index = None;
        self.original_flip_state = None;
    }

    /// Gets a read-only reference to the last position in the history.
    /// If navigating history, returns the position at history_position_index.
    pub fn position_ref(&self) -> &Chess {
        if let Some(index) = self.history_position_index {
            if index < self.position_history.len() {
                &self.position_history[index]
            } else {
                self.position_history.last().unwrap()
            }
        } else {
            self.position_history.last().unwrap()
        }
    }

    /// Gets a read-only reference to the current position, or None if history is empty
    /// If navigating history, returns the position at history_position_index.
    pub fn current_position(&self) -> Option<&Chess> {
        if let Some(index) = self.history_position_index {
            self.position_history.get(index)
        } else {
            self.position_history.last()
        }
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
    pub fn is_draw(&self) -> bool {
        let chess = self.position_ref();
        chess.is_stalemate()
            || self.consecutive_non_pawn_or_capture == 50
            || self.is_draw_by_repetition()
            || chess.is_insufficient_material()
    }

    // We check manually if the last move was a pawn to one of the promotion squares
    // AND the promotion hasn't been handled yet (no promotion piece set)
    pub fn is_latest_move_promotion(&self) -> bool {
        self.move_history
            .last()
            .map(|last_move| {
                // Check if it's a pawn move to promotion square
                let is_pawn_to_promotion_square = last_move.role() == Role::Pawn
                    && (last_move.to().rank() == Rank::First
                        || last_move.to().rank() == Rank::Eighth);

                // Only return true if it's a promotion move AND promotion hasn't been handled yet
                // If promotion() returns Some, the promotion was already handled
                is_pawn_to_promotion_square && last_move.promotion().is_none()
            })
            .unwrap_or(false)
    }

    /// Get piece type at a coordinate (handles flipped board)
    pub fn get_role_at_square(&self, square: &Square) -> Option<Role> {
        self.position_ref()
            .board()
            .piece_at(*square)
            .map(|p| p.role)
    }

    pub fn is_square_occupied(&self, square: &Square) -> bool {
        self.position_ref().board().piece_at(*square).is_some()
    }

    pub fn get_piece_color_at_square(&self, square: &Square) -> Option<Color> {
        let piece = self.position_ref().board().piece_at(*square);
        piece.map(|p| p.color)
    }

    /// Get authorized positions for a piece at the given coordinate
    pub fn get_authorized_positions(&self, player_turn: Color, square: &Square) -> Vec<Square> {
        // Check if there's a piece at this position and it's the right color
        if let Some(piece) = self.position_ref().board().piece_at(*square) {
            if piece.color != player_turn {
                return vec![];
            }
        } else {
            return vec![];
        }

        // Get all legal moves
        self.position_ref()
            .clone()
            .legal_moves()
            .iter()
            .filter(|m| m.from() == Some(*square))
            .map(|m| m.to())
            .collect()
    }

    /// Check if the king is in check
    pub fn is_getting_checked(&self, player_turn: Color) -> bool {
        let chess = self.position_ref();
        chess.is_check() && chess.turn() == player_turn
    }

    /// Get the king's coordinates
    pub fn get_king_coordinates(&self, color: Color) -> Coord {
        let king_square = match self.position_ref().board().king_of(color) {
            Some(sq) => sq,
            None => {
                log::error!("King not found for color {:?} (invalid board state)", color);
                // Return undefined coordinate as fallback
                return Coord::undefined();
            }
        };
        let mut coord = Coord::from_square(king_square);

        // If board is flipped, flip the coordinate for display
        if self.is_flipped {
            coord = Coord::new(7 - coord.row, 7 - coord.col);
        }

        coord
    }

    /// Check if game is checkmate
    pub fn is_checkmate(&self) -> bool {
        self.position_ref().clone().is_checkmate()
    }

    /// Flip the board for alternating perspectives
    pub fn flip_the_board(&mut self) {
        self.is_flipped = !self.is_flipped;
    }

    /// Get FEN position for UCI engine
    pub fn fen_position(&self) -> String {
        // Use FEN display from shakmaty
        use shakmaty::fen::Fen;
        let fen = Fen::from_position(self.position_ref().clone(), shakmaty::EnPassantMode::Legal);
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

    /// Reconstruct game history from a string of space-separated UCI moves
    /// Optionally verifies against an expected final FEN
    pub fn reconstruct_history(&mut self, moves_str: &str, expected_fen: Option<&str>) {
        if moves_str.trim().is_empty() {
            return;
        }

        log::info!(
            "Reconstructing history from {} moves",
            moves_str.split_whitespace().count()
        );

        // Reset board to initial state
        self.reset();

        // Start from the initial position
        let mut current_position = self.position_history[0].clone();
        let mut moves_applied = 0;
        let mut moves_failed = 0;

        // Parse moves string (space-separated UCI moves)
        let move_strings: Vec<&str> = moves_str.split_whitespace().collect();

        for (i, move_uci) in move_strings.iter().enumerate() {
            // Parse UCI move using shakmaty
            match move_uci.parse::<shakmaty::uci::UciMove>() {
                Ok(uci) => {
                    match uci.to_move(&current_position) {
                        Ok(chess_move) => {
                            // Track captures
                            if let Some(captured_piece) =
                                current_position.board().piece_at(chess_move.to())
                            {
                                self.taken_pieces.push(captured_piece);
                            }

                            // Apply the move
                            match current_position.play(&chess_move) {
                                Ok(new_pos) => {
                                    self.move_history.push(chess_move);
                                    self.position_history.push(new_pos.clone());
                                    current_position = new_pos;
                                    moves_applied += 1;
                                }
                                Err(e) => {
                                    log::warn!(
                                        "Failed to play move {}: {} - {}. Stopping history reconstruction.",
                                        i + 1, move_uci, e
                                    );
                                    moves_failed += 1;
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!(
                                "Illegal move {}: {} - {}. Stopping history reconstruction.",
                                i + 1,
                                move_uci,
                                e
                            );
                            moves_failed += 1;
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::warn!(
                        "Failed to parse UCI move {}: {} - {}. Skipping.",
                        i + 1,
                        move_uci,
                        e
                    );
                    moves_failed += 1;
                }
            }
        }

        if moves_failed > 0 {
            log::warn!(
                "Failed to apply {} out of {} moves. History may be incomplete.",
                moves_failed,
                move_strings.len()
            );
        }

        log::info!(
            "Successfully applied {} moves out of {} total",
            moves_applied,
            move_strings.len()
        );

        // Verify that the final position matches the expected FEN
        if let Some(fen_str) = expected_fen {
            if let Some(final_position) = self.position_history.last() {
                let final_fen = shakmaty::fen::Fen::from_position(
                    final_position.clone(),
                    shakmaty::EnPassantMode::Legal,
                )
                .to_string();

                // Compare FEN strings (ignoring move counters which might differ)
                let final_fen_parts: Vec<&str> = final_fen.split(' ').collect();
                let expected_fen_parts: Vec<&str> = fen_str.split(' ').collect();

                let positions_match = final_fen_parts.len() >= 4
                    && expected_fen_parts.len() >= 4
                    && final_fen_parts[0..4] == expected_fen_parts[0..4];

                if !positions_match {
                    log::warn!(
                        "Final position from moves doesn't match FEN. Final: {}, Expected: {}",
                        final_fen,
                        fen_str
                    );
                    log::warn!("Using FEN position and rebuilding history from it");

                    // If positions don't match, use the FEN position as the final position
                    // Try to parse the expected FEN
                    if let Ok(fen) = shakmaty::fen::Fen::from_ascii(fen_str.as_bytes()) {
                        if let Ok(position) =
                            fen.into_position::<shakmaty::Chess>(shakmaty::CastlingMode::Standard)
                        {
                            // Replace the last position in history with the FEN position
                            if let Some(last_pos) = self.position_history.last_mut() {
                                *last_pos = position;
                            }
                        }
                    }
                } else {
                    log::info!("Final position matches FEN - history is correct");
                }
            }
        }
    }

    /// Execute a move on the board
    /// Returns the executed Move if successful, None if illegal
    pub fn execute_move(
        &mut self,
        from: Square,
        to: Square,
        promotion: Option<Role>,
    ) -> Option<Move> {
        let chess = self.position_ref().clone();

        // Track captures before executing move
        if let Some(captured_piece) = chess.board().piece_at(to) {
            self.taken_pieces.push(captured_piece);
        }

        // Find matching legal move
        let legal_moves = chess.legal_moves();

        // Check if this looks like a castling move (king moving 2 squares horizontally)
        let is_potential_castling = {
            let piece_at_from = chess.board().piece_at(from);
            if let Some(piece) = piece_at_from {
                if piece.role == Role::King && piece.color == chess.turn() {
                    // Check if moving 2 squares horizontally (castling)
                    let from_file = from.file();
                    let to_file = to.file();
                    let from_rank = from.rank();
                    let to_rank = to.rank();
                    from_rank == to_rank && (from_file as i8 - to_file as i8).abs() == 2
                } else {
                    false
                }
            } else {
                false
            }
        };

        if is_potential_castling {
            log::debug!("Detected potential castling move: {} -> {}", from, to);
            // Log all castling moves available
            let castling_moves: Vec<_> = legal_moves
                .iter()
                .filter(|m| matches!(m, shakmaty::Move::Castle { .. }))
                .collect();
            if !castling_moves.is_empty() {
                for cm in &castling_moves {
                    log::debug!(
                        "  Available castling: {:?} (from: {:?}, to: {})",
                        cm,
                        cm.from(),
                        cm.to()
                    );
                }
            } else {
                log::warn!(
                    "Castling move {} -> {} requested but no castling moves available",
                    from,
                    to
                );
            }
        }

        let matching_move = legal_moves.iter().find(|m| {
            // Regular move matching (this will also match castling moves correctly)
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
            match chess.play(shakmaty_move) {
                Ok(new_chess) => {
                    // Update history
                    self.position_history.push(new_chess);
                    // Reset history navigation when a new move is made
                    self.history_position_index = None;
                    self.original_flip_state = None;
                    Some(shakmaty_move.clone())
                }
                Err(e) => {
                    log::error!("Failed to play move {} -> {}: {}", from, to, e);
                    None
                }
            }
        } else {
            // Move not found - check if it's a castling move that needs special handling
            if is_potential_castling {
                // Clone chess before using it in the play call
                let chess_clone = chess.clone();
                // Try to find castling move from the same square
                let castling_move = legal_moves
                    .iter()
                    .find(|m| matches!(m, shakmaty::Move::Castle { .. }) && m.from() == Some(from));

                if let Some(castle_move) = castling_move {
                    log::info!(
                        "Found castling move for {} -> {}: {:?} (actual destination: {})",
                        from,
                        to,
                        castle_move,
                        castle_move.to()
                    );
                    // Execute the castling move even though destination doesn't match exactly
                    match chess_clone.play(castle_move) {
                        Ok(new_chess) => {
                            self.position_history.push(new_chess);
                            self.history_position_index = None;
                            self.original_flip_state = None;
                            log::info!("Castling move executed successfully: {:?}", castle_move);
                            return Some(castle_move.clone());
                        }
                        Err(e) => {
                            log::error!("Failed to play castling move: {}", e);
                        }
                    }
                } else {
                    log::warn!(
                        "Castling move {} -> {} requested but no castling available from {}",
                        from,
                        to,
                        from
                    );
                }
            }

            // Log why move wasn't found
            let piece_at_from = chess.board().piece_at(from);
            let piece_at_to = chess.board().piece_at(to);
            log::debug!("Move {} -> {} not found in legal moves. Piece at from: {:?}, piece at to: {:?}, promotion: {:?}, current turn: {:?}", 
                from, to, piece_at_from, piece_at_to, promotion, chess.turn());
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

    /// Get the correct flip state for a given position index in solo mode
    /// In solo mode, the board flips after each move, so:
    /// - index 0 (initial): not flipped
    /// - index 1 (after move 1): flipped
    /// - index 2 (after move 2): not flipped
    /// - etc.
    fn get_flip_state_for_index(&self, index: usize, is_solo_mode: bool) -> bool {
        if !is_solo_mode {
            // In non-solo modes, use the current flip state
            return self.is_flipped;
        }
        // In solo mode, flip state alternates: odd indices are flipped
        (index % 2) == 1
    }

    /// Navigate to the next position in history (forward in time)
    /// Returns true if navigation was successful, false if already at the latest position
    /// is_solo_mode: whether we're in solo mode (affects board flipping)
    pub fn navigate_history_next(&mut self, is_solo_mode: bool) -> bool {
        if self.position_history.is_empty() {
            return false;
        }

        let max_index = self.position_history.len() - 1;

        match self.history_position_index {
            None => {
                // Already at the latest position
                false
            }
            Some(index) => {
                if index < max_index {
                    let new_index = index + 1;
                    self.history_position_index = Some(new_index);
                    // Update flip state based on position index
                    self.is_flipped = self.get_flip_state_for_index(new_index, is_solo_mode);
                    true
                } else {
                    // Reached the latest position, reset to None
                    // Restore the original flip state
                    self.history_position_index = None;
                    if let Some(original_flip) = self.original_flip_state.take() {
                        self.is_flipped = original_flip;
                    } else {
                        // Fallback: calculate based on latest index
                        self.is_flipped = self.get_flip_state_for_index(max_index, is_solo_mode);
                    }
                    true
                }
            }
        }
    }

    /// Navigate to the previous position in history (backward in time)
    /// Returns true if navigation was successful, false if already at the first position
    /// is_solo_mode: whether we're in solo mode (affects board flipping)
    pub fn navigate_history_previous(&mut self, is_solo_mode: bool) -> bool {
        if self.position_history.is_empty() {
            return false;
        }

        match self.history_position_index {
            None => {
                // Currently at latest position, go to previous
                // Store the current flip state before navigating
                if self.position_history.len() > 1 {
                    self.original_flip_state = Some(self.is_flipped);
                    let prev_index = self.position_history.len() - 2;
                    self.history_position_index = Some(prev_index);
                    // Update flip state based on position index
                    self.is_flipped = self.get_flip_state_for_index(prev_index, is_solo_mode);
                    true
                } else {
                    false
                }
            }
            Some(index) => {
                if index > 0 {
                    let new_index = index - 1;
                    self.history_position_index = Some(new_index);
                    // Update flip state based on position index
                    self.is_flipped = self.get_flip_state_for_index(new_index, is_solo_mode);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Reset history navigation to view the latest position
    /// is_solo_mode: whether we're in solo mode (affects board flipping)
    pub fn reset_history_navigation(&mut self, is_solo_mode: bool) {
        if self.history_position_index.is_some() {
            self.history_position_index = None;
            // Restore the original flip state
            if let Some(original_flip) = self.original_flip_state.take() {
                self.is_flipped = original_flip;
            } else {
                // Fallback: calculate based on latest index
                let max_index = self.position_history.len().saturating_sub(1);
                self.is_flipped = self.get_flip_state_for_index(max_index, is_solo_mode);
            }
        }
    }

    /// Truncate history at the given index, removing all moves and positions after it
    /// This is used when making a move from a historical position to create a new branch
    pub fn truncate_history_at(&mut self, index: usize) {
        if index >= self.position_history.len() {
            return;
        }

        // Truncate position history to index + 1 (keep the position at index)
        // We keep index + 1 because position_history[i] is the position AFTER move i
        let new_len = index + 1;
        if new_len < self.position_history.len() {
            self.position_history.truncate(new_len);
        }

        // Truncate move history to index (remove moves after this point)
        if index < self.move_history.len() {
            self.move_history.truncate(index);
        }

        // Reset history navigation since we're now at the latest position
        self.history_position_index = None;
        self.original_flip_state = None;
    }
}
