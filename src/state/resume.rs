//! On-disk persistence of an in-progress local or bot game so the user can
//! quit and pick up where they left off.
//!
//! The save file lives next to `config.toml` in the user's config dir and is
//! a small JSON document. We deliberately keep the schema minimal — only what
//! is required to reconstruct a playable position. Move history and clock
//! state are not preserved; on resume the game continues without history
//! navigation or a time control.

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use shakmaty::{Color, Piece, Role};

use crate::constants::config_dir;

/// Which game mode a saved game belongs to. Each mode has its own file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResumeMode {
    /// Two-player local game (Pages::Solo with no bot/opponent).
    Local,
    /// Game against the bot engine (Pages::Bot).
    Bot,
}

impl ResumeMode {
    fn filename(self) -> &'static str {
        match self {
            ResumeMode::Local => "resume_local.json",
            ResumeMode::Bot => "resume_bot.json",
        }
    }
}

/// Serializable form of a captured piece — shakmaty's `Color` and `Role`
/// don't derive `Serialize`, so we store them as short string tags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TakenPieceRecord {
    pub color: String,
    pub role: String,
}

impl TakenPieceRecord {
    pub fn from_piece(p: Piece) -> Self {
        Self {
            color: color_tag(p.color).to_string(),
            role: role_tag(p.role).to_string(),
        }
    }

    pub fn to_piece(&self) -> Option<Piece> {
        Some(Piece {
            color: color_from_tag(&self.color)?,
            role: role_from_tag(&self.role)?,
        })
    }
}

fn color_tag(c: Color) -> &'static str {
    match c {
        Color::White => "white",
        Color::Black => "black",
    }
}

fn color_from_tag(s: &str) -> Option<Color> {
    match s {
        "white" => Some(Color::White),
        "black" => Some(Color::Black),
        _ => None,
    }
}

fn role_tag(r: Role) -> &'static str {
    match r {
        Role::Pawn => "pawn",
        Role::Knight => "knight",
        Role::Bishop => "bishop",
        Role::Rook => "rook",
        Role::Queen => "queen",
        Role::King => "king",
    }
}

fn role_from_tag(s: &str) -> Option<Role> {
    match s {
        "pawn" => Some(Role::Pawn),
        "knight" => Some(Role::Knight),
        "bishop" => Some(Role::Bishop),
        "rook" => Some(Role::Rook),
        "queen" => Some(Role::Queen),
        "king" => Some(Role::King),
        _ => None,
    }
}

/// Bot configuration captured alongside a bot-mode save so the engine can be
/// re-launched with the same settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotConfig {
    pub path: String,
    pub depth: u8,
    pub difficulty: Option<u8>,
    /// "white" / "black" — the colour the human player chose.
    pub player_color: String,
    pub is_random_color: bool,
}

/// Clock state captured so each side resumes with the time they had on disk.
/// `cursor` and `custom_minutes` mirror `GameModeState` so the menu form is
/// pre-populated identically when starting follow-up games.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockState {
    pub white_ms: u64,
    pub black_ms: u64,
    pub clock_cursor: u8,
    pub custom_minutes: u16,
}

/// One on-disk save. Created after every move that mutates the position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedGame {
    pub mode: ResumeMode,
    /// FEN of the current position. Used as a sanity check after replaying
    /// `moves_uci`; if the move list is empty we fall back to using this FEN
    /// directly as the resumed position.
    pub fen: String,
    /// Whose perspective the board was being shown from.
    pub is_flipped: bool,
    pub taken_pieces: Vec<TakenPieceRecord>,
    /// Bot-only — `None` for `ResumeMode::Local`.
    pub bot: Option<BotConfig>,
    /// Present whenever the game was started with a time control.
    #[serde(default)]
    pub clock: Option<ClockState>,
    /// Every move played so far, encoded as UCI. Restoring these lets the
    /// resumed game replay history (P/N navigation, full move log).
    #[serde(default)]
    pub moves_uci: Vec<String>,
}

/// Returns the path the save file for `mode` would live at, or `None` if the
/// platform's config directory cannot be determined. We do not create the
/// directory here — callers should rely on the chess-tui subdirectory already
/// existing (it is created on startup in `main.rs`).
///
/// The `CHESS_TUI_RESUME_DIR` env var overrides the location entirely. It
/// exists so the integration tests can redirect persistence into a tempdir
/// without polluting the user's real config directory.
pub fn save_path(mode: ResumeMode) -> Option<PathBuf> {
    if let Ok(custom) = std::env::var("CHESS_TUI_RESUME_DIR") {
        let dir = PathBuf::from(custom);
        return Some(dir.join(mode.filename()));
    }
    let dir = config_dir().ok()?;
    Some(dir.join("chess-tui").join(mode.filename()))
}

/// Returns `true` if a save file exists on disk for `mode`. Errors (missing
/// config dir, IO problems) are treated as "no save".
pub fn has_save(mode: ResumeMode) -> bool {
    save_path(mode).map(|p| p.is_file()).unwrap_or(false)
}

/// Reads and parses the save for `mode`. Any failure (missing file, malformed
/// JSON) returns `None`; we never want a corrupt save to prevent the app from
/// starting a new game.
pub fn load(mode: ResumeMode) -> Option<SavedGame> {
    let path = save_path(mode)?;
    let raw = fs::read_to_string(&path).ok()?;
    match serde_json::from_str::<SavedGame>(&raw) {
        Ok(saved) if saved.mode == mode => Some(saved),
        Ok(_) => {
            log::warn!(
                "Resume file {} contains a different mode than expected; ignoring",
                path.display()
            );
            None
        }
        Err(e) => {
            log::warn!("Failed to parse resume file {}: {}", path.display(), e);
            None
        }
    }
}

/// Best-effort write. IO/serialisation failures are logged but never bubble
/// up — losing a save is annoying, but never worth interrupting gameplay.
pub fn save(mode: ResumeMode, game: &SavedGame) {
    let Some(path) = save_path(mode) else {
        log::warn!("Cannot persist resume save: no config directory");
        return;
    };
    let json = match serde_json::to_string_pretty(game) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("Failed to serialise resume save: {}", e);
            return;
        }
    };
    if let Err(e) = fs::write(&path, json) {
        log::warn!("Failed to write resume save {}: {}", path.display(), e);
    }
}

/// Best-effort delete. Missing-file is treated as success.
pub fn delete(mode: ResumeMode) {
    let Some(path) = save_path(mode) else {
        return;
    };
    match fs::remove_file(&path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => log::warn!("Failed to delete resume save {}: {}", path.display(), e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taken_piece_roundtrip() {
        for color in [Color::White, Color::Black] {
            for role in [
                Role::Pawn,
                Role::Knight,
                Role::Bishop,
                Role::Rook,
                Role::Queen,
                Role::King,
            ] {
                let p = Piece { color, role };
                let record = TakenPieceRecord::from_piece(p);
                assert_eq!(record.to_piece(), Some(p));
            }
        }
    }

    #[test]
    fn taken_piece_invalid_tag_returns_none() {
        let record = TakenPieceRecord {
            color: "purple".into(),
            role: "dragon".into(),
        };
        assert_eq!(record.to_piece(), None);
    }

    #[test]
    fn saved_game_json_roundtrip_local() {
        let original = SavedGame {
            mode: ResumeMode::Local,
            fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".into(),
            is_flipped: false,
            taken_pieces: vec![],
            bot: None,
            clock: Some(ClockState {
                white_ms: 600_000,
                black_ms: 594_500,
                clock_cursor: 3,
                custom_minutes: 10,
            }),
            moves_uci: vec!["e2e4".into()],
        };
        let json = serde_json::to_string(&original).expect("serialise");
        let parsed: SavedGame = serde_json::from_str(&json).expect("parse");
        assert_eq!(parsed, original);
    }

    #[test]
    fn saved_game_json_roundtrip_bot() {
        let original = SavedGame {
            mode: ResumeMode::Bot,
            fen: "8/8/8/8/8/8/8/4k2K w - - 0 1".into(),
            is_flipped: true,
            taken_pieces: vec![TakenPieceRecord {
                color: "white".into(),
                role: "pawn".into(),
            }],
            bot: Some(BotConfig {
                path: "/usr/local/bin/stockfish".into(),
                depth: 12,
                difficulty: Some(3),
                player_color: "black".into(),
                is_random_color: false,
            }),
            clock: None,
            moves_uci: vec![],
        };
        let json = serde_json::to_string(&original).expect("serialise");
        let parsed: SavedGame = serde_json::from_str(&json).expect("parse");
        assert_eq!(parsed, original);
    }

    #[test]
    fn saved_game_clock_field_is_backward_compatible() {
        // Older save files predate the `clock` field. They must still load
        // cleanly with `clock: None` so users coming from an earlier version
        // don't lose their resume state.
        let legacy_json = r#"{
            "mode": "Local",
            "fen": "8/8/8/8/8/8/8/4k2K w - - 0 1",
            "is_flipped": false,
            "taken_pieces": [],
            "bot": null
        }"#;
        let parsed: SavedGame = serde_json::from_str(legacy_json).expect("parse legacy");
        assert!(parsed.clock.is_none());
    }

    #[test]
    fn save_path_uses_per_mode_filename() {
        // Don't assert the leading path (depends on the host) — just confirm
        // the two modes do not collide and that the filenames are stable.
        let local = save_path(ResumeMode::Local).expect("config dir resolves on this platform");
        let bot = save_path(ResumeMode::Bot).expect("config dir resolves on this platform");
        assert_ne!(local, bot);
        assert!(local.ends_with("resume_local.json"));
        assert!(bot.ends_with("resume_bot.json"));
    }
}
