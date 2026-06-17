//! Autosave / resume orchestration for in-progress local and bot games.
//!
//! Persistence lives in [`crate::state::resume`]. This file owns the policy
//! decisions: when to write, when to clear, and how to rebuild the in-memory
//! game from a saved snapshot.

use std::time::Duration;

use shakmaty::{CastlingMode, Chess, Color, fen::Fen, uci::UciMove};

use crate::app::App;
use crate::constants::{Pages, Popups};
use crate::game_logic::bot::Bot;
use crate::game_logic::clock::Clock;
use crate::game_logic::game::{Game, GameState};
use crate::state::resume::{
    BotConfig, ClockState, ResumeMode, SavedGame, TakenPieceRecord, delete, has_save, load, save,
};

impl App {
    /// Returns the resume mode that matches the active game, if any.
    /// Multiplayer and Lichess games are not eligible — their state lives on
    /// the wire / on lichess.org.
    pub fn current_resume_mode(&self) -> Option<ResumeMode> {
        if self.game.logic.opponent.is_some() {
            return None;
        }
        match self.ui_state.current_page {
            Pages::Solo if self.game.logic.bot.is_none() => Some(ResumeMode::Local),
            Pages::Bot if self.game.logic.bot.is_some() => Some(ResumeMode::Bot),
            _ => None,
        }
    }

    /// Writes the current position to disk if the game is in a state worth
    /// saving and at least one move has been played since the last save.
    /// Use this when only a move advanced the state — clock-only ticks should
    /// go through `tick_resume_state` so they're rate-limited.
    pub fn autosave_resume_state(&mut self) {
        self.write_resume_state(false);
    }

    /// Per-tick hook from the main loop. Honours the move-count guard like
    /// `autosave_resume_state`, but additionally throttles clock-only writes
    /// to at most once per second so we don't hammer the disk while the user
    /// just stares at the board.
    pub fn tick_resume_state(&mut self) {
        let now = std::time::Instant::now();
        let recent = self
            .last_resume_flush_at
            .is_some_and(|t| now.duration_since(t) < std::time::Duration::from_secs(1));
        if recent {
            // Allow the move-guarded write to slip through immediately; only
            // the throttle-less force path is gated.
            self.write_resume_state(false);
            return;
        }
        self.write_resume_state(true);
        self.last_resume_flush_at = Some(now);
    }

    /// Bypasses the move-count guard so the clock's current value is
    /// captured. Used by `quit()` and `reset_home()` — paths that are
    /// guaranteed to wipe in-memory state next, so a stale save can't be
    /// updated later.
    pub fn flush_resume_state(&mut self) {
        self.write_resume_state(true);
        self.last_resume_flush_at = Some(std::time::Instant::now());
    }

    fn write_resume_state(&mut self, force: bool) {
        let Some(mode) = self.current_resume_mode() else {
            return;
        };

        // Mid-promotion is a half-move — wait for the user to pick a piece.
        // Terminal states get cleared by `clear_resume_state_on_end`, so we
        // intentionally skip writing them here too.
        if self.game.logic.game_state != GameState::Playing {
            return;
        }

        let count = self.game.logic.game_board.move_history.len();
        if count == 0 {
            return;
        }
        if !force && self.last_saved_move_count == Some(count) {
            return;
        }

        let saved = SavedGame {
            mode,
            fen: self.game.logic.game_board.fen_position(),
            is_flipped: self.game.logic.game_board.is_flipped,
            taken_pieces: self
                .game
                .logic
                .game_board
                .taken_pieces
                .iter()
                .map(|p| TakenPieceRecord::from_piece(*p))
                .collect(),
            bot: match mode {
                ResumeMode::Bot => self.bot_config_for_save(),
                ResumeMode::Local => None,
            },
            clock: self.clock_state_for_save(),
            moves_uci: self
                .game
                .logic
                .game_board
                .move_history
                .iter()
                .map(|m| UciMove::from_standard(m).to_string())
                .collect(),
        };

        save(mode, &saved);
        self.last_saved_move_count = Some(count);
    }

    fn clock_state_for_save(&self) -> Option<ClockState> {
        let clock = self.game.logic.clock.as_ref()?;
        let white = clock.get_time(Color::White);
        let black = clock.get_time(Color::Black);
        Some(ClockState {
            white_ms: white.as_millis() as u64,
            black_ms: black.as_millis() as u64,
            clock_cursor: self.game_mode_state.clock_cursor,
            custom_minutes: self.game_mode_state.custom_time_minutes,
        })
    }

    fn bot_config_for_save(&self) -> Option<BotConfig> {
        let bot = self.game.logic.bot.as_ref()?;
        let path = self.bot_state.chess_engine_path.clone()?;
        let player_color = match self.game_mode_state.selected_color {
            Some(Color::White) => "white",
            Some(Color::Black) => "black",
            None => "white",
        };
        Some(BotConfig {
            path,
            depth: bot.depth,
            difficulty: bot.difficulty,
            player_color: player_color.to_string(),
            is_random_color: self.game_mode_state.is_random_color,
        })
    }

    /// Removes the save file for the currently active game (if it lives in a
    /// resumable mode). Used when the game ends naturally and when the user
    /// explicitly starts a fresh game in the same mode.
    pub fn clear_resume_state(&mut self, mode: ResumeMode) {
        delete(mode);
        self.last_saved_move_count = None;
    }

    /// Convenience used by end-of-game hooks: clears the save for whichever
    /// resumable mode is currently active. No-op for Multiplayer/Lichess.
    pub fn clear_resume_state_for_current_mode(&mut self) {
        if let Some(mode) = self.current_resume_mode() {
            self.clear_resume_state(mode);
        }
    }

    /// Returns whether a saved game is on disk for `mode`. Used to gate the
    /// "Press R to resume" UI hint and key binding.
    pub fn has_resume_save(&self, mode: ResumeMode) -> bool {
        has_save(mode)
    }

    /// Loads the save for `mode` and rebuilds the in-memory game so play can
    /// continue. Returns `true` on success. Failures (no save, malformed
    /// FEN) are surfaced via an error popup; the menu state is left intact
    /// so the user can try again or start a new game.
    pub fn resume_from_saved(&mut self, mode: ResumeMode) -> bool {
        let Some(saved) = load(mode) else {
            self.ui_state
                .show_message_popup("No saved game to resume.".to_string(), Popups::Error);
            return false;
        };

        // Validate the saved FEN up-front; a malformed save is unrecoverable
        // and worth deleting so the user isn't stuck retrying the same error.
        if Fen::from_ascii(saved.fen.as_bytes())
            .ok()
            .and_then(|f| f.into_position::<Chess>(CastlingMode::Standard).ok())
            .is_none()
        {
            log::warn!("Resume failed: could not parse saved FEN '{}'", saved.fen);
            self.ui_state.show_message_popup(
                "Saved game is corrupted and cannot be resumed.".to_string(),
                Popups::Error,
            );
            delete(mode);
            return false;
        }

        // Reset to a fresh game and preserve theme like reset_home does.
        let display_mode = self.game.ui.display_mode;
        let current_skin = self.game.ui.skin.clone();
        self.game = Game::default();
        self.game.ui.display_mode = display_mode;
        self.game.ui.skin = current_skin;
        self.game.ui.available_piece_styles = self.theme_state.available_piece_styles.clone();

        if saved.moves_uci.is_empty() {
            // Old-format save (or zero-move snapshot): jump straight to the
            // FEN. The move history will be empty — accepted trade-off.
            // The FEN was validated above, so the chained parse cannot fail
            // here; we still match defensively rather than unwrap.
            if let Some(position) = Fen::from_ascii(saved.fen.as_bytes())
                .ok()
                .and_then(|f| f.into_position::<Chess>(CastlingMode::Standard).ok())
            {
                self.game.logic.game_board.position_history = vec![position];
            }
            self.game.logic.game_board.move_history = vec![];
            self.game.logic.game_board.taken_pieces = saved
                .taken_pieces
                .iter()
                .filter_map(|r| r.to_piece())
                .collect();
        } else {
            // Replay the UCI moves from the default initial position so the
            // resumed game has full history (P/N navigation, move log).
            // `reconstruct_history` resets the board, applies each move, and
            // sanity-checks the final position against `expected_fen`.
            let moves_joined = saved.moves_uci.join(" ");
            self.game
                .logic
                .game_board
                .reconstruct_history(&moves_joined, Some(&saved.fen));
        }

        self.game.logic.game_board.history_position_index = None;
        self.game.logic.game_board.is_flipped = saved.is_flipped;
        self.game.logic.sync_player_turn_with_position();
        self.game.logic.game_state = GameState::Playing;

        // Restore any saved clock + the form selection used to create it so a
        // follow-up "new game" presents the same time control by default.
        if let Some(cs) = saved.clock.as_ref() {
            let mut clock = Clock::with_remaining(
                Duration::from_millis(cs.white_ms),
                Duration::from_millis(cs.black_ms),
            );
            // Game was mid-flight before the quit; start the side-to-move
            // clock immediately so resume feels continuous instead of frozen.
            clock.start(self.game.logic.player_turn);
            self.game.logic.clock = Some(clock);
            self.game_mode_state.clock_cursor = cs.clock_cursor;
            self.game_mode_state.custom_time_minutes = cs.custom_minutes;
        } else {
            self.game.logic.clock = None;
        }

        match mode {
            ResumeMode::Local => {
                self.ui_state.current_page = Pages::Solo;
                self.last_saved_move_count = Some(0);
            }
            ResumeMode::Bot => {
                let Some(bot_cfg) = saved.bot.as_ref() else {
                    self.ui_state.show_message_popup(
                        "Saved bot game is missing its engine config.".to_string(),
                        Popups::Error,
                    );
                    return false;
                };
                let player_color = match bot_cfg.player_color.as_str() {
                    "black" => Color::Black,
                    _ => Color::White,
                };
                self.bot_state.chess_engine_path = Some(bot_cfg.path.clone());
                self.bot_state.bot_depth = bot_cfg.depth;
                self.bot_state.bot_difficulty = bot_cfg.difficulty;
                self.game_mode_state.selected_color = Some(player_color);
                self.game_mode_state.is_random_color = bot_cfg.is_random_color;

                // Re-launch the engine. `Bot::new` matches the `bot_setup`
                // call path so the resumed game behaves like a fresh one.
                let is_bot_starting = self.game.logic.player_turn != player_color
                    && player_color == Color::White
                    || (player_color == Color::Black
                        && self.game.logic.player_turn == Color::White);
                self.game.logic.bot = Some(Bot::new(
                    &bot_cfg.path,
                    is_bot_starting,
                    bot_cfg.depth,
                    bot_cfg.difficulty,
                ));

                self.ui_state.current_page = Pages::Bot;
                self.last_saved_move_count = Some(0);

                // If it's the bot's turn in the saved position, kick it off.
                if self.game.logic.player_turn != player_color
                    && let Some(bot) = &self.game.logic.bot
                {
                    self.bot_state.start_bot_thinking(
                        self.game.logic.game_board.fen_position(),
                        bot.depth,
                        bot.difficulty,
                    );
                }
            }
        }

        self.ui_state.close_popup();
        true
    }
}
