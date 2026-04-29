//! Shared constants and navigation enums.

use core::fmt;
use std::path::PathBuf;

use ratatui::style::Color;

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
#[derive(Debug, PartialEq, Clone)]
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
    /// Y/N confirmation before resigning.
    ResignConfirmation,
    /// SAN move text entry.
    MoveInputSelection,
    /// File path entry for loading a PGN file.
    LoadPgnPath,
}

/// Base URL for all Lichess REST API requests.
pub const LICHESS_API_URL: &str = "https://lichess.org/api";
