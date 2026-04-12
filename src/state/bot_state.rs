use std::sync::mpsc::{channel, Receiver};
use shakmaty::Move;
use crate::game_logic::bot::Bot;

/// Define every variable related to the bot in the app
pub struct BotState {
    /// Bot thinking depth for chess engine (used when difficulty is Off)
    pub bot_depth: u8,
    /// Bot difficulty preset: None = Off (full strength), Some(0..=3) = Easy/Medium/Hard/Magnus
    pub bot_difficulty: Option<u8>,
    /// Bot thinking channel receiver
    pub bot_move_receiver: Option<Receiver<Move>>,
    /// path of the chess engine
    pub chess_engine_path: Option<String>,
}

impl Default for BotState {
    fn default() -> Self {
        Self {
            bot_depth: 10,
            bot_difficulty: None,
            bot_move_receiver: None,
            chess_engine_path: None,
        }
    }
}

impl BotState {


    /// Start bot thinking in a separate thread
    pub fn start_bot_thinking(&mut self, fen: String, depth: u8, bot_difficulty: Option<u8>) {
        // Don't start if already thinking
        if self.bot_move_receiver.is_some() {
            return;
        }

        let engine_path = self.chess_engine_path.clone().unwrap_or_default();

        // Create channel for communication
        let (tx, rx) = channel();
        self.bot_move_receiver = Some(rx);

        // Spawn thread to compute bot move
        std::thread::spawn(move || {
            // Create bot instance in thread
            let bot = Bot::new(&engine_path, false, depth, bot_difficulty);
            let uci_move = bot.get_move(&fen);

            // Convert UCI move to shakmaty Move
            let position: Option<shakmaty::Chess> = shakmaty::fen::Fen::from_ascii(fen.as_bytes())
                .ok()
                .and_then(|fen| fen.into_position(shakmaty::CastlingMode::Standard).ok());

            if let Some(pos) = position {
                if let Ok(chess_move) = uci_move.to_move(&pos) {
                    let _ = tx.send(chess_move);
                }
            }
        });
    }

    /// Check if bot is currently thinking
    pub fn is_bot_thinking(&self) -> bool {
        self.bot_move_receiver.is_some()
    }
}