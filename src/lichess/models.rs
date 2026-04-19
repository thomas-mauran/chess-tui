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
    // We only need game_id and color, but keep minimal structure for deserialization
    #[serde(flatten)]
    _rest: serde_json::Value,
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct OpponentInfo {
    pub id: Option<String>,
    pub username: String,
    pub rating: Option<u32>,
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
    pub pgn: String,
    pub clock: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PuzzleInfo {
    pub id: String,
    pub rating: u32,
    pub plays: u32,
    #[serde(rename = "initialPly")]
    pub initial_ply: u32,
    pub solution: Vec<String>,
    pub themes: Vec<String>,
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
    #[serde(default)]
    pub seen_at: Option<u64>,
    #[serde(default)]
    pub created_at: Option<u64>,
    #[serde(default)]
    pub count: Option<UserCounts>,
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
    pub rating: u32,
    #[serde(default)]
    pub rd: Option<u32>,
    #[serde(default)]
    pub prog: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RatingHistoryEntry {
    pub name: String,
    pub points: Vec<[i32; 4]>, // [year, month, day, rating]
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
                .timeout(None)
                .http1_only()
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }
}