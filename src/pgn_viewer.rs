//! PGN viewer state — parses a PGN file into a sequence of board positions
//! and provides navigation + auto-play controls.
//!
//! Supports multi-game PGN files (e.g. multiple training games saved by watch.py).
//! Games are stored as `Vec<PgnViewer>` and the caller cycles between them.

use shakmaty::san::San;
use shakmaty::{Chess, Move, Position};

// Auto-play tick speeds (event loop fires ~4 ticks/sec)
pub const SPEED_FAST: u64 = 1;
pub const SPEED_NORMAL: u64 = 4;
pub const SPEED_SLOW: u64 = 8;

pub struct PgnViewer {
    /// positions[0] = starting position, positions[i] = position after moves[i-1]
    pub positions: Vec<Chess>,
    /// shakmaty Move objects — for board highlighting
    pub moves: Vec<Move>,
    /// SAN strings — for display in the history panel
    pub sans: Vec<String>,
    /// Which ply is currently shown (0 = start)
    pub current_ply: usize,
    /// Auto-play state
    pub auto_play: bool,
    pub auto_play_speed: u64,
    pub auto_play_tick: u64,
    /// Metadata from PGN headers
    pub title: String,
    pub white: String,
    pub black: String,
    pub result: String,
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
            match parse_single_game(&game_text) {
                Ok(v) => viewers.push(v),
                Err(_) => {} // skip unparseable games silently
            }
        }
        if viewers.is_empty() {
            Err("No valid games found in PGN".to_string())
        } else {
            Ok(viewers)
        }
    }

    // ── Navigation ──────────────────────────────────────────────────────────

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

    pub fn next(&mut self) {
        if self.current_ply < self.moves.len() {
            self.current_ply += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.current_ply > 0 {
            self.current_ply -= 1;
        }
    }

    pub fn goto_start(&mut self) {
        self.current_ply = 0;
    }

    pub fn goto_end(&mut self) {
        self.current_ply = self.moves.len();
    }

    pub fn total_plies(&self) -> usize {
        self.moves.len()
    }

    /// Called every app tick — advances ply when auto-play is on.
    pub fn tick(&mut self) {
        if !self.auto_play {
            return;
        }
        self.auto_play_tick += 1;
        if self.auto_play_tick >= self.auto_play_speed {
            self.auto_play_tick = 0;
            if self.current_ply < self.moves.len() {
                self.current_ply += 1;
            } else {
                self.auto_play = false; // stop at end
            }
        }
    }

    pub fn speed_up(&mut self) {
        self.auto_play_speed = self.auto_play_speed.saturating_sub(1).max(SPEED_FAST);
    }

    pub fn speed_down(&mut self) {
        self.auto_play_speed = (self.auto_play_speed + 1).min(16);
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

    for line in pgn.lines() {
        let t = line.trim();
        if !t.starts_with('[') {
            continue;
        }
        if let Some(val) = extract_header_value(t) {
            if t.to_lowercase().starts_with("[event") {
                title = val;
            } else if t.to_lowercase().starts_with("[white") {
                white = val;
            } else if t.to_lowercase().starts_with("[black") {
                black = val;
            } else if t.to_lowercase().starts_with("[result") {
                result = val;
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
        // Strip check/checkmate annotations — shakmaty's San parser handles them
        // but being explicit avoids issues with non-standard suffixes
        let clean: &str = san_str.trim_end_matches(|c: char| c == '+' || c == '#');
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
        auto_play_speed: SPEED_NORMAL,
        auto_play_tick: 0,
        title,
        white,
        black,
        result,
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

    // Remove variations (…) — handle nesting
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
        let clean = token.trim_end_matches(|c: char| matches!(c, '!' | '?'));
        if !clean.is_empty() {
            result.push(clean.to_string());
        }
    }
    result
}
