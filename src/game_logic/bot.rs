use crate::constants::{BOT_DIFFICULTY_DEPTH, BOT_DIFFICULTY_ELO, BOT_DIFFICULTY_MOVETIME_MS};
use ruci::{Engine, Go};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use std::borrow::Cow;
use std::process::Command;
use std::str::FromStr;

#[derive(Clone)]
pub struct Bot {
    pub engine_path: String,
    /// Used to indicate if a bot move is following
    pub bot_will_move: bool,
    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
    /// Base depth when difficulty is Off (full strength)
    pub depth: u8,
    /// Difficulty preset index: None = Off (full strength), Some(0..=3) = Easy/Medium/Hard/Magnus
    pub difficulty: Option<u8>,
}

impl Bot {
    pub fn new(engine_path: &str, is_bot_starting: bool, depth: u8, difficulty: Option<u8>) -> Bot {
        Self {
            engine_path: engine_path.to_string(),
            bot_will_move: false,
            is_bot_starting,
            depth,
            difficulty,
        }
    }

    /// Effective depth: preset depth when difficulty is set, else base depth.
    fn effective_depth(&self) -> u8 {
        self.difficulty
            .and_then(|i| {
                let idx = i as usize;
                if idx < BOT_DIFFICULTY_DEPTH.len() {
                    Some(BOT_DIFFICULTY_DEPTH[idx])
                } else {
                    None
                }
            })
            .unwrap_or(self.depth)
    }

    /// Movetime in ms when difficulty is set; None = no limit (full strength).
    fn movetime_ms(&self) -> Option<u64> {
        self.difficulty.and_then(|i| {
            let idx = i as usize;
            if idx < BOT_DIFFICULTY_MOVETIME_MS.len() {
                Some(BOT_DIFFICULTY_MOVETIME_MS[idx])
            } else {
                None
            }
        })
    }

    /// ELO for UCI_LimitStrength when difficulty is set.
    fn elo(&self) -> Option<u16> {
        self.difficulty.and_then(|i| {
            let idx = i as usize;
            if idx < BOT_DIFFICULTY_ELO.len() {
                Some(BOT_DIFFICULTY_ELO[idx])
            } else {
                None
            }
        })
    }

    /// Get the best move from the chess engine.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The engine process fails to spawn
    /// - The engine fails to initialize
    /// - The FEN string is invalid
    /// - The engine fails to return a move
    pub fn get_move(&self, fen: &str) -> UciMove {
        // Parse engine_path to support command-line arguments
        // Split by spaces, treating first part as command and rest as args
        let parts: Vec<&str> = self.engine_path.split_whitespace().collect();
        let (command, args) = if parts.is_empty() {
            (self.engine_path.as_str(), &[] as &[&str])
        } else {
            (parts[0], &parts[1..])
        };

        let mut cmd = Command::new(command);
        if !args.is_empty() {
            cmd.args(args);
        }

        let mut process = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap_or_else(|e| panic!("Failed to spawn engine process: {e}"));

        let mut engine = Engine::from_process(&mut process, false)
            .unwrap_or_else(|e| panic!("Failed to initialize engine: {e}"));

        // UCI handshake (required before setoption/position). Discard options.
        engine
            .use_uci(|_| {})
            .unwrap_or_else(|e| panic!("Failed UCI handshake: {e}"));

        // Optional ELO limit via UCI options (UCI_LimitStrength + UCI_Elo) when difficulty is set
        if let Some(elo) = self.elo() {
            engine
                .send(ruci::gui::SetOption {
                    name: Cow::Borrowed("UCI_LimitStrength"),
                    value: Some(Cow::Borrowed("true")),
                })
                .unwrap_or_else(|e| panic!("Failed to set UCI_LimitStrength: {e}"));
            engine
                .send(ruci::gui::SetOption {
                    name: Cow::Borrowed("UCI_Elo"),
                    value: Some(Cow::Owned(elo.to_string())),
                })
                .unwrap_or_else(|e| panic!("Failed to set UCI_Elo: {e}"));
        }

        let fen_parsed =
            Fen::from_str(fen).unwrap_or_else(|e| panic!("Failed to parse FEN '{fen}': {e}"));

        engine
            .send(ruci::Position::Fen {
                fen: Cow::Owned(fen_parsed),
                moves: Cow::Borrowed(&[]),
            })
            .unwrap_or_else(|e| panic!("Failed to send position to engine: {e}"));

        let depth = self.effective_depth();
        let move_time_ms = self.movetime_ms();

        engine
            .go(
                &Go {
                    depth: Some(depth as usize),
                    move_time: move_time_ms.map(|ms| ms as usize),
                    ..Default::default()
                },
                |_| {},
            )
            .unwrap_or_else(|e| panic!("Engine failed to compute move: {e}"))
            .take_normal()
            .unwrap_or_else(|| panic!("Engine returned non-normal move"))
            .r#move
    }
}
