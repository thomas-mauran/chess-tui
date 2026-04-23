//! PGN file parsing and playback.

use shakmaty::san::San;
use shakmaty::{Chess, Move, Position};

/// Auto-play rate multipliers (1x ≈ one move per second).
///
/// The event loop ticks every 250ms, so at each tick we advance the
/// accumulator by `rate * 0.25`; when it reaches 1.0 the next ply plays.
pub const SPEEDS: &[(f32, &str)] = &[
    (0.5, "0.5x"),
    (1.0, "1x"),
    (1.5, "1.5x"),
    (2.0, "2x"),
    (2.5, "2.5x"),
    (3.0, "3x"),
    (4.0, "4x"),
];
pub const DEFAULT_SPEED_IDX: usize = 1; // 1x

pub struct PgnViewer {
    /// positions[0] = starting position, positions[i] = position after moves[i-1]
    pub positions: Vec<Chess>,
    /// shakmaty Move objects - for board highlighting
    pub moves: Vec<Move>,
    /// SAN strings - for display in the history panel
    pub sans: Vec<String>,
    /// Which ply is currently shown (0 = start)
    pub current_ply: usize,
    /// Auto-play state
    pub auto_play: bool,
    pub speed_idx: usize,
    pub auto_play_accum: f32,
    /// Set to true when the user presses `h` on the end-of-game banner.
    /// Reset when the viewer moves off the final ply.
    pub end_banner_dismissed: bool,
    /// Metadata from PGN headers
    pub title: String,
    pub white: String,
    pub black: String,
    pub result: String,
    /// Termination reason (e.g. "Normal", "Time forfeit", "Player A won by checkmate")
    pub termination: String,
}

impl PgnViewer {
    /// Load all games from a PGN file.
    pub fn from_file(path: &str) -> Result<Vec<Self>, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Cannot read '{}': {}", path, e))?;
        Self::from_pgn_str(&content)
    }

    /// Parse a PGN string that may contain one or more games.
    pub fn from_pgn_str(pgn: &str) -> Result<Vec<Self>, String> {
        let mut viewers = Vec::new();
        for game_text in split_pgn_games(pgn) {
            if let Ok(v) = parse_single_game(&game_text) {
                viewers.push(v);
            }
        }
        if viewers.is_empty() {
            Err("No valid games found in PGN".to_string())
        } else {
            Ok(viewers)
        }
    }

    // ── Navigation ──────────────────────────────────────────────────────────

    /// Returns the board position at the current ply.
    pub fn current_position(&self) -> &Chess {
        &self.positions[self.current_ply]
    }

    /// The move that led to `current_ply` (used for board highlighting).
    pub fn last_move(&self) -> Option<&Move> {
        if self.current_ply > 0 {
            self.moves.get(self.current_ply - 1)
        } else {
            None
        }
    }

    /// Advances one ply forward, clamped at the final position.
    pub fn next(&mut self) {
        if self.current_ply < self.moves.len() {
            self.current_ply += 1;
        }
    }

    /// Steps one ply back and clears `end_banner_dismissed`.
    pub fn prev(&mut self) {
        if self.current_ply > 0 {
            self.current_ply -= 1;
            self.end_banner_dismissed = false;
        }
    }

    /// Jumps to the starting position (ply 0).
    pub fn goto_start(&mut self) {
        self.current_ply = 0;
        self.end_banner_dismissed = false;
    }

    /// Jumps to the final position (last ply).
    pub fn goto_end(&mut self) {
        self.current_ply = self.moves.len();
    }

    /// Returns the total number of plies (half-moves) in the game.
    pub fn total_plies(&self) -> usize {
        self.moves.len()
    }

    /// Returns `true` when the viewer is on the final position.
    pub fn is_at_end(&self) -> bool {
        !self.moves.is_empty() && self.current_ply == self.moves.len()
    }

    /// Human-readable result string for display (e.g. "White won", "Draw").
    pub fn result_summary(&self) -> String {
        match self.result.as_str() {
            "1-0" => format!("{} (White) won", self.white),
            "0-1" => format!("{} (Black) won", self.black),
            "1/2-1/2" => "Draw".to_string(),
            _ => self.result.clone(),
        }
    }

    /// Called every app tick (250ms) - advances ply when auto-play is on.
    pub fn tick(&mut self) {
        if !self.auto_play {
            return;
        }
        let rate = SPEEDS[self.speed_idx].0;
        self.auto_play_accum += rate * 0.25;
        while self.auto_play_accum >= 1.0 {
            self.auto_play_accum -= 1.0;
            if self.current_ply < self.moves.len() {
                self.current_ply += 1;
            } else {
                self.auto_play = false;
                self.auto_play_accum = 0.0;
                break;
            }
        }
    }

    /// Returns the human-readable label for the current auto-play speed (e.g. `"1x"`).
    pub fn speed_label(&self) -> &'static str {
        SPEEDS[self.speed_idx].1
    }

    /// Increases auto-play speed by one step, clamped at the maximum.
    pub fn speed_up(&mut self) {
        if self.speed_idx + 1 < SPEEDS.len() {
            self.speed_idx += 1;
        }
    }

    /// Decreases auto-play speed by one step, clamped at the minimum.
    pub fn speed_down(&mut self) {
        if self.speed_idx > 0 {
            self.speed_idx -= 1;
        }
    }
}

// ── PGN parsing ──────────────────────────────────────────────────────────────

/// Split a multi-game PGN string into individual game strings.
/// A new game starts when we see a header line (`[...`) after movetext.
fn split_pgn_games(pgn: &str) -> Vec<String> {
    let mut games: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut has_movetext = false;

    for line in pgn.lines() {
        let trimmed = line.trim();
        // New game starts when a header arrives after we've seen movetext
        if trimmed.starts_with('[') && has_movetext {
            if !current.trim().is_empty() {
                games.push(std::mem::take(&mut current));
            }
            has_movetext = false;
        }
        if !trimmed.is_empty() && !trimmed.starts_with('[') {
            has_movetext = true;
        }
        current.push_str(line);
        current.push('\n');
    }
    if !current.trim().is_empty() {
        games.push(current);
    }
    if games.is_empty() && !pgn.trim().is_empty() {
        games.push(pgn.to_string());
    }
    games
}

fn parse_single_game(pgn: &str) -> Result<PgnViewer, String> {
    // Extract header values
    let mut title = String::from("?");
    let mut white = String::from("?");
    let mut black = String::from("?");
    let mut result = String::from("*");
    let mut termination = String::new();

    for line in pgn.lines() {
        let t = line.trim();
        if !t.starts_with('[') {
            continue;
        }
        if let Some(val) = extract_header_value(t) {
            let lower = t.to_lowercase();
            if lower.starts_with("[event") {
                title = val;
            } else if lower.starts_with("[white ") || lower.starts_with("[white\"") {
                white = val;
            } else if lower.starts_with("[black ") || lower.starts_with("[black\"") {
                black = val;
            } else if lower.starts_with("[result") {
                result = val;
            } else if lower.starts_with("[termination") {
                termination = val;
            }
        }
    }

    // Build movetext from non-header lines
    let mut movetext = String::new();
    let mut past_headers = false;
    for line in pgn.lines() {
        let t = line.trim();
        if !t.starts_with('[') {
            past_headers = true;
        }
        if past_headers {
            movetext.push_str(line);
            movetext.push(' ');
        }
    }

    // Parse SAN tokens from movetext
    let san_strings = extract_san_tokens(&movetext);

    // Apply moves to build position chain
    let mut positions: Vec<Chess> = vec![Chess::default()];
    let mut moves: Vec<Move> = Vec::new();
    let mut sans: Vec<String> = Vec::new();
    let mut pos = Chess::default();

    for san_str in &san_strings {
        // Strip check/checkmate annotations - shakmaty's San parser handles them
        // but being explicit avoids issues with non-standard suffixes
        let clean: &str = san_str.trim_end_matches(['+', '#']);
        match clean.parse::<San>() {
            Ok(san) => match san.to_move(&pos) {
                Ok(mv) => match pos.clone().play(&mv) {
                    Ok(new_pos) => {
                        moves.push(mv);
                        sans.push(san_str.clone());
                        pos = new_pos;
                        positions.push(pos.clone());
                    }
                    Err(_) => break,
                },
                Err(_) => break,
            },
            Err(_) => break,
        }
    }

    if moves.is_empty() {
        return Err(format!("No moves parsed from game '{}'", title));
    }

    Ok(PgnViewer {
        positions,
        moves,
        sans,
        current_ply: 0,
        auto_play: false,
        speed_idx: DEFAULT_SPEED_IDX,
        auto_play_accum: 0.0,
        end_banner_dismissed: false,
        title,
        white,
        black,
        result,
        termination,
    })
}

/// Extract a header value from a PGN tag like `[White "Magnus Carlsen"]`.
fn extract_header_value(line: &str) -> Option<String> {
    let start = line.find('"')?;
    let end = line.rfind('"')?;
    if end > start {
        Some(line[start + 1..end].to_string())
    } else {
        None
    }
}

/// Strip comments, variations and move numbers from movetext, return SAN tokens.
fn extract_san_tokens(movetext: &str) -> Vec<String> {
    let mut text = movetext.to_string();

    // Remove block comments {…}  (may span multiple words)
    loop {
        match (text.find('{'), text.find('}')) {
            (Some(s), Some(e)) if e > s => {
                text.replace_range(s..=e, " ");
            }
            _ => break,
        }
    }

    // Remove variations (…) - handle nesting
    loop {
        let mut found = false;
        let chars: Vec<char> = text.chars().collect();
        let mut depth = 0usize;
        let mut open_idx: Option<usize> = None;
        for (ci, &c) in chars.iter().enumerate() {
            match c {
                '(' => {
                    if depth == 0 {
                        open_idx = Some(ci);
                    }
                    depth += 1;
                }
                ')' if depth > 0 => {
                    depth -= 1;
                    if depth == 0 {
                        if let Some(s) = open_idx {
                            // Convert char indices to byte indices
                            let byte_s = text.char_indices().nth(s).map(|(i, _)| i).unwrap_or(0);
                            let byte_e = text
                                .char_indices()
                                .nth(ci)
                                .map(|(i, _)| i + 1)
                                .unwrap_or(text.len());
                            text.replace_range(byte_s..byte_e, " ");
                            found = true;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        if !found {
            break;
        }
    }

    let mut result = Vec::new();
    for token in text.split_whitespace() {
        // Move numbers: "1.", "2.", "1...", "10.", etc.
        let is_move_number = token
            .trim_end_matches('.')
            .chars()
            .all(|c| c.is_ascii_digit());
        if is_move_number {
            continue;
        }
        // NAG annotations: $1, $2, …
        if token.starts_with('$') {
            continue;
        }
        // Result tokens
        if matches!(token, "1-0" | "0-1" | "1/2-1/2" | "*") {
            continue;
        }
        // Strip trailing annotation glyphs (!, ?, !?, ?!, !!, ??)
        let clean = token.trim_end_matches(['!', '?']);
        if !clean.is_empty() {
            result.push(clean.to_string());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHECKMATE_PGN: &str = r#"[Event "Live"]
[White "A"]
[Black "B"]
[Result "0-1"]
[Termination "B won by checkmate"]

1. e4 e5 2. Qh5 Nc6 3. Bc4 Nf6 4. Qxf7# 0-1
"#;

    #[test]
    fn parses_headers_and_termination() {
        let games = PgnViewer::from_pgn_str(CHECKMATE_PGN).expect("parse PGN");
        let v = &games[0];
        assert_eq!(v.white, "A");
        assert_eq!(v.black, "B");
        assert_eq!(v.result, "0-1");
        assert_eq!(v.termination, "B won by checkmate");
        assert_eq!(v.result_summary(), "B (Black) won");
    }

    #[test]
    fn navigates_back_from_final_position() {
        let mut games = PgnViewer::from_pgn_str(CHECKMATE_PGN).expect("parse PGN");
        let v = &mut games[0];
        v.goto_end();
        assert!(v.is_at_end());
        let end_ply = v.current_ply;
        v.prev();
        assert_eq!(v.current_ply, end_ply - 1);
        assert!(!v.is_at_end());
    }

    #[test]
    fn speed_is_clamped_at_bounds() {
        let mut games = PgnViewer::from_pgn_str(CHECKMATE_PGN).expect("parse PGN");
        let v = &mut games[0];
        assert_eq!(v.speed_label(), "1x");
        for _ in 0..20 {
            v.speed_up();
        }
        assert_eq!(v.speed_idx, SPEEDS.len() - 1);
        assert_eq!(v.speed_label(), "4x");
        for _ in 0..20 {
            v.speed_down();
        }
        assert_eq!(v.speed_idx, 0);
        assert_eq!(v.speed_label(), "0.5x");
    }

    #[test]
    fn prev_clears_end_banner_dismissal() {
        let mut games = PgnViewer::from_pgn_str(CHECKMATE_PGN).expect("parse PGN");
        let v = &mut games[0];
        v.goto_end();
        v.end_banner_dismissed = true;
        v.prev();
        assert!(!v.end_banner_dismissed);
    }
}
