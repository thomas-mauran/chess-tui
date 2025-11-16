use crate::game_logic::coord::Coord;
use crate::game_logic::game_board::GameBoard;
use crate::pieces::{PieceColor, PieceMove, PieceType};
use shakmaty::san::San;
use shakmaty::{Chess, Color, Position, Role, Square};
use std::fs;
use std::str::FromStr;

/// Convert shakmaty's Color to our PieceColor
fn convert_color(color: Color) -> PieceColor {
    match color {
        Color::White => PieceColor::White,
        Color::Black => PieceColor::Black,
    }
}

/// Convert shakmaty's Role to our PieceType
fn convert_role(role: Role) -> PieceType {
    match role {
        Role::Pawn => PieceType::Pawn,
        Role::Knight => PieceType::Knight,
        Role::Bishop => PieceType::Bishop,
        Role::Rook => PieceType::Rook,
        Role::Queen => PieceType::Queen,
        Role::King => PieceType::King,
    }
}

/// Convert shakmaty's Square to our Coord
fn convert_square(square: Square) -> Coord {
    let row = 7 - square.rank() as u8; // Invert row (shakmaty uses 0=a1, we use 0=a8)
    let col = square.file() as u8;
    Coord::new(row, col)
}

/// Load a PGN file and apply moves to the game board
pub fn load_pgn_file(file_path: &str) -> Result<(GameBoard, PieceColor), String> {
    // Read the file
    let pgn_content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read PGN file: {}", e))?;

    parse_pgn(&pgn_content)
}

/// Parse PGN content and apply moves to the game board
pub fn parse_pgn(pgn_content: &str) -> Result<(GameBoard, PieceColor), String> {
    let mut game_board = GameBoard::default();
    let mut pos = Chess::default();

    // Parse the PGN content - find the moves section
    // PGN format has headers (lines starting with [) first, then moves
    // Skip all header lines (lines starting with [)
    let lines: Vec<&str> = pgn_content.lines().collect();
    let mut move_start_idx = 0;
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('[') {
            move_start_idx = idx;
            break;
        }
    }
    
    let moves_section = if move_start_idx < lines.len() {
        lines[move_start_idx..].join(" ")
    } else {
        return Err("No moves found in PGN".to_string());
    };
    
    if moves_section.trim().is_empty() {
        return Err("No moves found in PGN".to_string());
    }

    // Extract individual moves from the text
    // Remove move numbers, result indicators, and comments
    let cleaned_moves: Vec<&str> = moves_section
        .split_whitespace()
        .filter(|token| {
            // Filter out move numbers (e.g., "1.", "2.", etc.)
            !token.ends_with('.')
                && !token.chars().all(|c| c.is_ascii_digit() || c == '.')
                // Filter out game results
                && *token != "1-0"
                && *token != "0-1"
                && *token != "1/2-1/2"
                && *token != "*"
                // Filter out comments (basic filtering)
                && !token.starts_with('{')
                && !token.starts_with('(')
                && !token.starts_with('[')
        })
        .collect();

    // Apply each move
    for move_str in cleaned_moves {
        // Skip empty strings
        if move_str.is_empty() {
            continue;
        }

        // Parse the SAN move
        let san = San::from_str(move_str)
            .map_err(|e| format!("Failed to parse move '{}': {}", move_str, e))?;

        // Convert to a legal move
        let m = san
            .to_move(&pos)
            .map_err(|e| format!("Illegal move '{}': {}", move_str, e))?;

        // Get the move details before applying it
        let from_square = m.from().ok_or_else(|| "Invalid move: no from square".to_string())?;
        let to_square = m.to();
        let from_coord = convert_square(from_square);
        let to_coord = convert_square(to_square);

        // Get piece type at from position
        let piece_type = pos
            .board()
            .piece_at(from_square)
            .map(|p| convert_role(p.role))
            .ok_or_else(|| "No piece at from square".to_string())?;

        // Apply the move to shakmaty position
        pos = pos.play(&m).map_err(|e| format!("Failed to apply move: {}", e))?;

        // Apply the move to our game board
        apply_move_to_game_board(&mut game_board, from_coord, to_coord, piece_type)?;
    }

    // Determine whose turn it is
    let player_turn = convert_color(pos.turn());

    Ok((game_board, player_turn))
}

/// Apply a move to our game board
fn apply_move_to_game_board(
    game_board: &mut GameBoard,
    from: Coord,
    to: Coord,
    _piece_type: PieceType,
) -> Result<(), String> {
    // Get the piece at the from position
    let piece = game_board.board[&from]
        .ok_or_else(|| format!("No piece at position {:?}", from))?;

    let (from_piece_type, from_piece_color) = piece;

    // Check if there's a piece being captured
    let captured_piece = game_board.board[&to];

    // Move the piece
    game_board.board[&to] = Some((from_piece_type, from_piece_color));
    game_board.board[&from] = None;

    // Handle captures
    if let Some((captured_type, captured_color)) = captured_piece {
        match captured_color {
            PieceColor::White => game_board.white_taken_pieces.push(captured_type),
            PieceColor::Black => game_board.black_taken_pieces.push(captured_type),
        }
    }

    // Handle castling by moving the rook
    if from_piece_type == PieceType::King {
        let col_diff = (to.col as i8) - (from.col as i8);
        if col_diff.abs() == 2 {
            // This is a castling move
            let rook_from_col = if col_diff > 0 { 7 } else { 0 };
            let rook_to_col = if col_diff > 0 { to.col - 1 } else { to.col + 1 };
            let rook_row = from.row;

            let rook_from = Coord::new(rook_row, rook_from_col);
            let rook_to = Coord::new(rook_row, rook_to_col);

            if let Some((PieceType::Rook, color)) = game_board.board[&rook_from] {
                game_board.board[&rook_to] = Some((PieceType::Rook, color));
                game_board.board[&rook_from] = None;
            }
        }
    }

    // Handle en passant
    if from_piece_type == PieceType::Pawn {
        let col_diff = (to.col as i8) - (from.col as i8);
        // If pawn moved diagonally but there was no piece at destination (before the move),
        // it's en passant
        if col_diff.abs() == 1 && captured_piece.is_none() {
            // Remove the captured pawn
            let captured_pawn_row = from.row;
            let captured_pawn_coord = Coord::new(captured_pawn_row, to.col);
            if let Some((_, color)) = game_board.board[&captured_pawn_coord] {
                match color {
                    PieceColor::White => game_board.white_taken_pieces.push(PieceType::Pawn),
                    PieceColor::Black => game_board.black_taken_pieces.push(PieceType::Pawn),
                }
                game_board.board[&captured_pawn_coord] = None;
            }
        }
    }

    // Record the move
    let piece_move = PieceMove {
        from,
        to,
        piece_type: from_piece_type,
        piece_color: from_piece_color,
    };
    game_board.move_history.push(piece_move);
    game_board.board_history.push(game_board.board);

    // Update consecutive non-pawn or capture counter
    game_board.increment_consecutive_non_pawn_or_capture(
        from_piece_type,
        captured_piece.map(|(t, _)| t),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pgn() {
        let pgn = "1. e4 e5 2. Nf3 Nc6";
        let result = parse_pgn(pgn);
        assert!(result.is_ok());
        let (game_board, player_turn) = result.unwrap();
        // After 4 moves, it should be white's turn
        assert_eq!(player_turn, PieceColor::White);
        assert_eq!(game_board.move_history.len(), 4);
    }

    #[test]
    fn test_load_pgn_file() {
        // Create a temporary PGN file
        use std::io::Write;
        let temp_file = "/tmp/test_chess.pgn";
        let mut file = std::fs::File::create(temp_file).unwrap();
        writeln!(file, "[Event \"Test\"]").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "1. e4 e5 2. Nf3 Nc6 3. Bb5").unwrap();
        drop(file);

        let result = load_pgn_file(temp_file);
        assert!(result.is_ok(), "Failed to load PGN file: {:?}", result.err());
        
        let (game_board, player_turn) = result.unwrap();
        assert_eq!(game_board.move_history.len(), 5);
        assert_eq!(player_turn, PieceColor::Black); // Black's turn after 5 moves
        
        // Clean up
        std::fs::remove_file(temp_file).ok();
    }
}
