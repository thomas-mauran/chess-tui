//! API types and HTTP client.

use reqwest::blocking::Client;
use serde::Deserialize;

pub enum LichessError {
    NoToken,
    NoClient,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "gameFull")]
    GameFull {
        id: String,
        white: Player,
        black: Player,
        state: GameState,
    },
    #[serde(rename = "gameState")]
    GameState(GameState),
    #[serde(rename = "chatLine")]
    ChatLine,
}

// Event stream types for /api/stream/event
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum EventStreamEvent {
    #[serde(rename = "gameStart")]
    GameStart { game: EventStreamGame },
    #[serde(rename = "gameFinish")]
    GameFinish { game: EventStreamGame },
    #[serde(rename = "challenge")]
    Challenge,
    #[serde(rename = "challengeCanceled")]
    ChallengeCanceled,
    #[serde(rename = "challengeDeclined")]
    ChallengeDeclined,
}

#[derive(Debug, Deserialize)]
pub struct EventStreamGame {
    #[serde(rename = "gameId")]
    pub game_id: String,
    pub color: String,
    /// chess-tui plays standard chess only, so the variant decides whether the
    /// game can be set up at all. See [`GameVariant::is_standard`].
    #[serde(default)]
    pub variant: Option<GameVariant>,
    // We only need game_id, color and variant, but keep minimal structure for deserialization
    #[serde(flatten)]
    _rest: serde_json::Value,
}

/// Rule set a game is played under, as reported by the API.
///
/// chess-tui models every position with [`shakmaty::Chess`], so anything other
/// than `standard` cannot be parsed, let alone played: a Horde FEN has no white
/// king and its moves are illegal in standard chess from the first ply.
#[derive(Debug, Deserialize, Clone)]
pub struct GameVariant {
    pub key: String,
    pub name: String,
}

impl GameVariant {
    /// Standard chess is the only rule set chess-tui can set up a board for.
    pub fn is_standard(&self) -> bool {
        self.key == "standard"
    }
}

/// Name of a game's variant when it is one chess-tui cannot play.
///
/// A missing `variant` is treated as standard, which is how the API omits it.
pub fn unsupported_variant_name(variant: Option<&GameVariant>) -> Option<String> {
    variant.filter(|v| !v.is_standard()).map(|v| v.name.clone())
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GameState {
    pub moves: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OngoingGame {
    #[serde(rename = "gameId")]
    pub game_id: String,
    #[serde(rename = "fullId")]
    pub full_id: String,
    pub color: String,
    pub fen: String,
    pub opponent: OpponentInfo,
    #[serde(rename = "isMyTurn")]
    pub is_my_turn: bool,
    /// See [`GameVariant`]: non-standard games cannot be set up.
    #[serde(default)]
    pub variant: Option<GameVariant>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OpponentInfo {
    pub id: Option<String>,
    pub username: String,
    pub rating: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct OngoingGamesResponse {
    #[serde(rename = "nowPlaying")]
    pub now_playing: Vec<OngoingGame>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Puzzle {
    pub game: PuzzleGame,
    pub puzzle: PuzzleInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleGame {
    pub id: String,
    /// Empty on instances whose puzzle database has no game replay for the puzzle.
    #[serde(default)]
    pub pgn: String,
    /// Absent on instances whose puzzles do not come from a clocked game.
    #[serde(default)]
    pub clock: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleInfo {
    pub id: String,
    pub rating: u16,
    pub plays: u32,
    #[serde(rename = "initialPly")]
    pub initial_ply: u16,
    pub solution: Vec<String>,
    pub themes: Vec<String>,
    /// Starting position of the puzzle. Sent by instances that have no game PGN
    /// to replay, and is then the only way to set up the board.
    #[serde(default)]
    pub fen: Option<String>,
    #[serde(default, rename = "lastMove")]
    pub last_move: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub perfs: Option<Perfs>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub online: Option<bool>,
    #[serde(default)]
    pub profile: Option<ProfileInfo>,
    #[serde(default, rename = "seenAt")]
    pub seen_at: Option<u64>,
    #[serde(default, rename = "createdAt")]
    pub created_at: Option<u64>,
    #[serde(default)]
    pub count: Option<UserCounts>,
    #[serde(default, rename = "playTime")]
    pub playtime: Option<Playtime>,
    #[serde(default)]
    pub streamer: Option<UserStreamer>,
}

impl UserProfile {
    /// Convert createdAt from milliseconds to seconds for easier date handling
    pub fn created_at_secs(&self) -> Option<u64> {
        self.created_at.map(|ms| ms / 1000)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileInfo {
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default, rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(default, rename = "lastName")]
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Playtime {
    #[serde(default)]
    pub total: Option<u32>,
    #[serde(default)]
    pub tv: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserStreamer {
    #[serde(default)]
    pub youtube: Option<String>,
    #[serde(default)]
    pub twitch: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserCounts {
    #[serde(default)]
    pub all: Option<u32>,
    #[serde(default)]
    pub rated: Option<u32>,
    #[serde(default)]
    pub ai: Option<u32>,
    #[serde(default)]
    pub draw: Option<u32>,
    #[serde(default, rename = "drawH")]
    pub draw_h: Option<u32>,
    #[serde(default)]
    pub loss: Option<u32>,
    #[serde(default, rename = "lossH")]
    pub loss_h: Option<u32>,
    #[serde(default)]
    pub win: Option<u32>,
    #[serde(default, rename = "winH")]
    pub win_h: Option<u32>,
    #[serde(default)]
    pub bookmark: Option<u32>,
    #[serde(default)]
    pub playing: Option<u32>,
    #[serde(default)]
    pub import: Option<u32>,
    #[serde(default)]
    pub me: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Perfs {
    #[serde(default)]
    pub bullet: Option<Perf>,
    #[serde(default)]
    pub blitz: Option<Perf>,
    #[serde(default)]
    pub rapid: Option<Perf>,
    #[serde(default)]
    pub classical: Option<Perf>,
    #[serde(default)]
    pub puzzle: Option<Perf>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Perf {
    pub rating: u16,
    #[serde(default)]
    pub rd: Option<u16>,
    #[serde(default)]
    pub prog: Option<i16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RatingHistoryEntry {
    pub name: String,
    pub points: Vec<[i16; 4]>, // [year, month, day, rating]
}

#[derive(Clone)]
pub struct LichessClient {
    pub token: String,
    pub client: Client,
}

impl LichessClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .http1_only()
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }
}
