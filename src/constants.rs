//! Shared constants and navigation enums.

use core::fmt;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

use ratatui::style::Color;
use throbber_widgets_tui::Set;

pub const CHESS_SET: Set = Set {
    full: "♚",
    empty: " ",
    symbols: &["♚", "♛", "♜", "♝", "♞", "♟", "♔", "♕", "♖", "♗", "♘", "♙"],
};

/// Sentinel value meaning "no board position set" (used before a square is chosen).
pub const UNDEFINED_POSITION: u8 = u8::MAX;
/// Light square / piece color used throughout the board renderer.
pub const WHITE: Color = Color::Rgb(160, 160, 160);
/// Dark square / piece color used throughout the board renderer.
pub const BLACK: Color = Color::Rgb(128, 95, 69);

/// TCP port for peer-to-peer multiplayer connections.
pub const NETWORK_PORT: u16 = 2308;
/// Byte length of the move message exchanged over TCP.
pub const NETWORK_BUFFER_SIZE: usize = 5;
/// Short poll interval used when waiting for network events.
pub const SLEEP_DURATION_SHORT_MS: u64 = 50;
/// Longer poll interval used for less time-sensitive waits.
pub const SLEEP_DURATION_LONG_MS: u64 = 100;
/// Wait after sending a resign to Lichess before continuing.
pub const SLEEP_DURATION_RESIGN_MS: u64 = 500;
/// Pause after showing puzzle solution before proceeding.
pub const SLEEP_DURATION_PUZZLE_MS: u64 = 1500;

/// Seconds in one day, used for converting timestamps to day units.
pub const SECONDS_PER_DAY: f64 = 86400.0;
/// Number of days of rating history shown in the chart.
pub const RATING_HISTORY_DAYS: i64 = 90;

/// Name of the built-in default skin entry.
pub const SKIN_NAME_DEFAULT: &str = "Default";
/// Name of the built-in ASCII skin entry.
pub const SKIN_NAME_ASCII: &str = "ASCII";

/// Index into [`TIME_CONTROL_OPTIONS`] that selects the "Custom" entry.
pub const TIME_CONTROL_CUSTOM_INDEX: u8 = 6;

/// Display labels for the four bot difficulty presets.
pub const BOT_DIFFICULTY_NAMES: [&str; 4] =
    ["Easy (400)", "Medium (900)", "Hard (1500)", "Magnus (2700)"];
/// `UCI_Elo` values passed to the engine for each difficulty preset.
pub const BOT_DIFFICULTY_ELO: [u16; 4] = [400, 900, 1500, 2700];
/// Engine search depth for each difficulty preset.
pub const BOT_DIFFICULTY_DEPTH: [u8; 4] = [1, 4, 8, 20];
/// Engine think time in milliseconds for each difficulty preset.
pub const BOT_DIFFICULTY_MOVETIME_MS: [u64; 4] = [25, 120, 500, 12_000];
/// Number of bot difficulty presets.
pub const BOT_DIFFICULTY_COUNT: usize = 4;

/// Time control options displayed in the game-mode configuration form.
pub const TIME_CONTROL_OPTIONS: &[&str] = &[
    "UltraBullet",
    "Bullet",
    "Blitz",
    "Rapid",
    "Classical",
    "No clock",
    "Custom",
];

/// Default selected time control index (Rapid).
pub const DEFAULT_TIME_CONTROL_SELECTED: u8 = 3;
/// Default duration in minutes when "Custom" time control is selected.
pub const DEFAULT_CUSTOM_TIME_VALUE: u16 = 10;

/// ASCII-art banner rendered on the home screen.
pub const TITLE: &str = r"
 ██████╗██╗  ██╗███████╗███████╗███████╗   ████████╗██╗   ██╗██╗
██╔════╝██║  ██║██╔════╝██╔════╝██╔════╝   ╚══██╔══╝██║   ██║██║
██║     ███████║█████╗  ███████╗███████╗█████╗██║   ██║   ██║██║
██║     ██╔══██║██╔══╝  ╚════██║╚════██║╚════╝██║   ██║   ██║██║
╚██████╗██║  ██║███████╗███████║███████║      ██║   ╚██████╔╝██║
 ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝      ╚═╝    ╚═════╝ ╚═╝
";

/// Piece rendering style: built-in Unicode, plain ASCII, or a custom skin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    /// Built-in Unicode piece set.
    DEFAULT,
    /// Plain ASCII characters.
    ASCII,
    /// User-defined skin loaded from `skins.json`.
    CUSTOM,
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayMode::ASCII => write!(f, "ASCII"),
            DisplayMode::DEFAULT => write!(f, "DEFAULT"),
            DisplayMode::CUSTOM => write!(f, "CUSTOM"),
        }
    }
}

/// Returns the user's config directory path.
///
/// # Errors
///
/// Returns an error if the config directory cannot be determined.
pub fn config_dir() -> Result<PathBuf, &'static str> {
    match dirs::config_dir() {
        Some(dir) => Ok(dir),
        None => Err("Could not get config directory"),
    }
}

/// TUI navigation targets — the full-screen "page" currently being rendered.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pages {
    /// Main menu.
    Home,
    /// Local two-player game.
    Solo,
    /// TCP peer-to-peer game.
    Multiplayer,
    /// Active Lichess game board.
    Lichess,
    /// Lichess landing menu.
    LichessMenu,
    /// Ongoing Lichess games list.
    OngoingGames,
    /// Game against the chess engine.
    Bot,
    /// Credits / about page.
    Credit,
    /// Game-mode selection form.
    GameModeMenu,
    /// PGN replay viewer.
    PgnViewer,
}
impl Pages {
    #[must_use]
    pub fn variant_count() -> usize {
        9
    }
}

/// Overlay popup kinds rendered on top of the active page.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Popups {
    /// Text input for the remote peer's IP address.
    EnterHostIP,
    /// Shown while the host waits for a peer to connect.
    WaitingForOpponentToJoin,
    /// Engine binary could not be found or launched.
    EnginePathError,
    /// Key-bindings help overlay.
    Help,
    /// Game-over result screen.
    EndScreen,
    /// Puzzle-completion result screen.
    PuzzleEndScreen,
    /// Generic error message.
    Error,
    /// Generic success message.
    Success,
    /// Waiting for a Lichess game pairing.
    SeekingLichessGame,
    /// Text input for a Lichess game ID or URL.
    EnterGameCode,
    /// Masked text input for the Lichess API token.
    EnterLichessToken,
    /// Text input for the Lichess API base URL.
    EnterLichessApiUrl,
    /// Y/N confirmation before resigning.
    ResignConfirmation,
    /// SAN move text entry.
    MoveInputSelection,
    /// File path entry for loading a PGN file.
    LoadPgnPath,
    /// Spinner to make user wait
    Loading,
}

/// Default base URL for all Lichess REST API requests.
pub const DEFAULT_LICHESS_API_URL: &str = "https://lichess.org/api";

/// Environment variable used to override the Lichess API base URL.
pub const LICHESS_API_URL_ENV: &str = "CHESS_TUI_LICHESS_API_URL";

/// Base URL for all Lichess REST API requests.
///
/// Defaults to [`DEFAULT_LICHESS_API_URL`], and can be overridden by setting the
/// `CHESS_TUI_LICHESS_API_URL` environment variable. Any trailing slashes are
/// trimmed so callers can safely append `/some/path`.
static LICHESS_API_URL: LazyLock<RwLock<String>> = LazyLock::new(|| {
    RwLock::new(resolve_lichess_api_url(
        std::env::var(LICHESS_API_URL_ENV).ok(),
    ))
});

/// Returns the Lichess API base URL currently in effect, without a trailing slash.
pub fn lichess_api_url() -> String {
    LICHESS_API_URL
        .read()
        .map(|url| url.clone())
        .unwrap_or_else(|poisoned| poisoned.into_inner().clone())
}

/// Overrides the Lichess API base URL for the rest of the process.
///
/// The value is normalized the same way as the environment variable: surrounding
/// whitespace and trailing slashes are trimmed, and a blank value resets the URL
/// to [`DEFAULT_LICHESS_API_URL`]. Returns the value that was stored.
pub fn set_lichess_api_url(raw: &str) -> String {
    let resolved = resolve_lichess_api_url(Some(raw.to_string()));
    match LICHESS_API_URL.write() {
        Ok(mut url) => *url = resolved.clone(),
        Err(poisoned) => *poisoned.into_inner() = resolved.clone(),
    }
    resolved
}

/// Returns `true` when the API URL is pinned by the environment variable.
///
/// The variable takes precedence over the persisted config at startup, so the UI
/// uses this to tell the user why their saved value was not applied.
pub fn lichess_api_url_is_env_pinned() -> bool {
    std::env::var(LICHESS_API_URL_ENV)
        .ok()
        .is_some_and(|url| !url.trim().is_empty())
}

/// Normalizes a raw Lichess API base URL, falling back to the default when blank.
pub fn resolve_lichess_api_url(raw: Option<String>) -> String {
    raw.map(|url| url.trim().trim_end_matches('/').to_string())
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| DEFAULT_LICHESS_API_URL.to_string())
}
/// Base URL for the chess-tui documentation.
pub const DOCS_URL: &str = "https://thomas-mauran.github.io/chess-tui/docs";

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_LICHESS_API_URL, lichess_api_url, resolve_lichess_api_url, set_lichess_api_url,
    };

    #[test]
    fn unset_env_falls_back_to_default() {
        assert_eq!(resolve_lichess_api_url(None), DEFAULT_LICHESS_API_URL);
    }

    #[test]
    fn blank_env_falls_back_to_default() {
        assert_eq!(
            resolve_lichess_api_url(Some("   ".to_string())),
            DEFAULT_LICHESS_API_URL
        );
    }

    #[test]
    fn custom_url_is_used() {
        assert_eq!(
            resolve_lichess_api_url(Some("http://localhost:9663/api".to_string())),
            "http://localhost:9663/api"
        );
    }

    #[test]
    fn trailing_slashes_and_whitespace_are_trimmed() {
        assert_eq!(
            resolve_lichess_api_url(Some(" https://lichess.dev/api// ".to_string())),
            "https://lichess.dev/api"
        );
    }

    /// One test drives the whole global so parallel tests cannot race on it.
    #[test]
    fn setting_the_url_at_runtime_replaces_it_and_normalizes() {
        assert_eq!(
            set_lichess_api_url(" http://localhost:9663/api/ "),
            "http://localhost:9663/api"
        );
        assert_eq!(lichess_api_url(), "http://localhost:9663/api");

        // A blank value restores the default rather than emptying the URL.
        assert_eq!(set_lichess_api_url("   "), DEFAULT_LICHESS_API_URL);
        assert_eq!(lichess_api_url(), DEFAULT_LICHESS_API_URL);
    }
}
