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
    /// Settings menu.
    SettingsMenu,
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
    /// Text input for chess engine path.
    EnterEnginePath,
}

/// Default base URL for all Lichess REST API requests.
pub const DEFAULT_LICHESS_API_URL: &str = "https://lichess.org/api";

/// Path suffix a Lichess API base URL is expected to end with.
pub const LICHESS_API_URL_SUFFIX: &str = "/api";

/// OAuth scopes chess-tui needs on a Lichess personal access token.
///
/// Pre-selected on the token creation form linked from the token popup, so the
/// user only has to press submit.
pub const LICHESS_TOKEN_SCOPES: [&str; 4] = [
    "preference:read",
    "board:play",
    "challenge:write",
    "puzzle:read",
];

/// Description pre-filled on the Lichess token creation form.
pub const LICHESS_TOKEN_DESCRIPTION: &str = "chess-tui";

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

/// Resolves the API URL to install at startup, or `None` to leave the current one.
///
/// Precedence is the command-line flag, then `CHESS_TUI_LICHESS_API_URL`, then the
/// persisted config value. `None` means the environment variable is pinning the URL
/// (it is already loaded into the global), so the config value must not overwrite it.
pub fn startup_lichess_api_url(cli: Option<&str>, config: Option<&str>) -> Option<String> {
    if let Some(cli) = cli {
        return Some(resolve_lichess_api_url(Some(cli.to_string())));
    }
    if lichess_api_url_is_env_pinned() {
        return None;
    }
    config.map(|url| resolve_lichess_api_url(Some(url.to_string())))
}

/// Normalizes a raw Lichess API base URL, falling back to the default when blank.
pub fn resolve_lichess_api_url(raw: Option<String>) -> String {
    raw.map(|url| url.trim().trim_end_matches('/').to_string())
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| DEFAULT_LICHESS_API_URL.to_string())
}
/// Returns `true` when `raw` is a usable API base URL, or blank (meaning "default").
///
/// Only the scheme is checked: anything reachable over HTTP is a candidate, and a
/// wrong host can only be found out by talking to it.
pub fn is_valid_lichess_api_url(raw: &str) -> bool {
    let url = raw.trim();
    url.is_empty() || url.starts_with("http://") || url.starts_with("https://")
}

/// Returns `true` when `raw` already ends with the `/api` path segment.
///
/// A blank value counts as having it, since it resolves to
/// [`DEFAULT_LICHESS_API_URL`].
pub fn lichess_api_url_has_suffix(raw: &str) -> bool {
    let url = raw.trim().trim_end_matches('/');
    url.is_empty() || url.ends_with(LICHESS_API_URL_SUFFIX)
}

/// Returns `raw` with `/api` appended, leaving it alone if it is already there.
pub fn append_lichess_api_suffix(raw: &str) -> String {
    let url = raw.trim().trim_end_matches('/');
    if url.is_empty() {
        return DEFAULT_LICHESS_API_URL.to_string();
    }
    if url.ends_with(LICHESS_API_URL_SUFFIX) {
        url.to_string()
    } else {
        format!("{}{}", url, LICHESS_API_URL_SUFFIX)
    }
}

/// Builds the token creation URL for the instance serving `api_url`.
///
/// The `/api` suffix is stripped to get back to the web root, and chess-tui's
/// scopes are pre-selected so the user only has to submit the form.
pub fn lichess_token_create_url(api_url: &str) -> String {
    let base = api_url.trim().trim_end_matches('/');
    let base = base
        .strip_suffix(LICHESS_API_URL_SUFFIX)
        .unwrap_or(base)
        .trim_end_matches('/');
    let scopes = LICHESS_TOKEN_SCOPES
        .iter()
        .map(|scope| format!("scopes[]={}", scope))
        .collect::<Vec<_>>()
        .join("&");
    format!(
        "{}/account/oauth/token/create?{}&description={}",
        base, scopes, LICHESS_TOKEN_DESCRIPTION
    )
}

/// Base URL for the chess-tui documentation.
pub const DOCS_URL: &str = "https://thomas-mauran.github.io/chess-tui/docs";

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_LICHESS_API_URL, append_lichess_api_suffix, is_valid_lichess_api_url,
        lichess_api_url, lichess_api_url_has_suffix, lichess_token_create_url,
        resolve_lichess_api_url, set_lichess_api_url, startup_lichess_api_url,
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

    #[test]
    fn the_cli_flag_beats_the_config_and_is_normalized() {
        assert_eq!(
            startup_lichess_api_url(Some(" https://cli.example/api/ "), Some("https://cfg/api")),
            Some("https://cli.example/api".to_string())
        );
    }

    #[test]
    fn the_config_applies_when_no_flag_is_given() {
        assert_eq!(
            startup_lichess_api_url(None, Some("https://cfg.example/api")),
            Some("https://cfg.example/api".to_string())
        );
    }

    #[test]
    fn nothing_is_applied_without_a_flag_or_config() {
        assert_eq!(startup_lichess_api_url(None, None), None);
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

    #[test]
    fn only_http_urls_are_valid_and_blank_means_default() {
        assert!(is_valid_lichess_api_url("https://lichess.org/api"));
        assert!(is_valid_lichess_api_url("http://localhost:9663/api"));
        assert!(is_valid_lichess_api_url("  "));
        assert!(!is_valid_lichess_api_url("lichess.org/api"));
        assert!(!is_valid_lichess_api_url("ftp://lichess.org/api"));
    }

    #[test]
    fn the_api_suffix_is_detected_through_whitespace_and_slashes() {
        assert!(lichess_api_url_has_suffix(" https://lichess.dev/api/ "));
        assert!(lichess_api_url_has_suffix(""));
        assert!(!lichess_api_url_has_suffix("https://lichess.dev"));
        assert!(!lichess_api_url_has_suffix("https://lichess.dev/apis"));
    }

    #[test]
    fn appending_the_suffix_is_idempotent() {
        assert_eq!(
            append_lichess_api_suffix("https://lichess.dev/"),
            "https://lichess.dev/api"
        );
        assert_eq!(
            append_lichess_api_suffix("https://lichess.dev/api"),
            "https://lichess.dev/api"
        );
        assert_eq!(append_lichess_api_suffix("  "), DEFAULT_LICHESS_API_URL);
    }

    #[test]
    fn the_token_url_points_at_the_instance_with_scopes_preselected() {
        assert_eq!(
            lichess_token_create_url("https://lichess.verde.zoe/api"),
            "https://lichess.verde.zoe/account/oauth/token/create?scopes[]=preference:read&scopes[]=board:play&scopes[]=challenge:write&scopes[]=puzzle:read&description=chess-tui"
        );
        // A URL saved without the /api suffix still yields the same web root.
        assert!(
            lichess_token_create_url("https://lichess.verde.zoe/")
                .starts_with("https://lichess.verde.zoe/account/oauth/token/create?")
        );
    }
}
