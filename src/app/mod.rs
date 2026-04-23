//! Single mutable root passed to every handler; owns all game logic, state structs, and global settings.

use crate::constants::Popups;
use crate::game_logic::game::Game;
use crate::game_logic::game::GameState;
use crate::state::bot_state::BotState;
use crate::state::game_mode_state::GameModeState;
use crate::state::lichess_state::LichessState;
use crate::state::multiplayer_state::MultiplayerState;
use crate::state::theme_state::ThemeState;
use crate::state::ui_state::UIState;
use log::LevelFilter;
use std::error;

pub mod bot;
pub mod config;
pub mod game;
pub mod input;
pub mod lichess;
pub mod menu;
pub mod multiplayer;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Top-level application context that owns all runtime state.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Game
    pub game: Game,
    /// The log level of the app
    pub log_level: LevelFilter,
    /// Whether sound effects are enabled
    pub sound_enabled: bool,
    /// Everything related to the skin handling through the app
    pub theme_state: ThemeState,
    /// Bot engine state (path, depth, difficulty, move channel)
    pub bot_state: BotState,
    /// UI navigation state (current page, popup, menu cursor)
    pub ui_state: UIState,
    /// Everything related to multiplayer networking
    pub multiplayer_state: MultiplayerState,
    /// Everything related to game mode setup
    pub game_mode_state: GameModeState,
    /// Everything related to Lichess
    pub lichess_state: LichessState,
    /// PGN viewer: list of games loaded from a PGN file
    pub pgn_viewer_state: Option<Vec<crate::pgn_viewer::PgnViewer>>,
    /// PGN viewer: which game is currently shown
    pub pgn_viewer_game_idx: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            game: Game::default(),
            log_level: LevelFilter::Off,
            // TODO: Make a skin::default implem
            theme_state: ThemeState::default(),
            bot_state: BotState::default(),
            multiplayer_state: MultiplayerState::default(),
            game_mode_state: GameModeState::default(),
            lichess_state: LichessState::default(),
            ui_state: UIState::default(),
            sound_enabled: true,
            pgn_viewer_state: None,
            pgn_viewer_game_idx: 0,
        }
    }
}

impl App {
    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // Advance PGN viewer auto-play
        if let Some(ref mut games) = self.pgn_viewer_state {
            if let Some(viewer) = games.get_mut(self.pgn_viewer_game_idx) {
                viewer.tick();
            }
        }

        // Update cursor blink state (used to flicker the cursor cell when a piece is selected)
        self.game.ui.update_cursor_blink();

        // Handle puzzle logic
        if let Some(mut puzzle_game) = self.lichess_state.puzzle_game.take() {
            puzzle_game.check_elo_update();

            if let Some(success_message) =
                puzzle_game.check_pending_move(&mut self.game, self.lichess_state.token.clone())
            {
                self.ui_state
                    .show_message_popup(success_message, Popups::PuzzleEndScreen);
            }

            self.lichess_state.puzzle_game = Some(puzzle_game);
        }

        // Check clock for time up (for local games and bot games with clock)
        if let Some(ref mut clock) = self.game.logic.clock {
            if clock.any_time_up() {
                if let Some(time_up_color) = clock.get_time_up_color() {
                    // Time is up - end the game
                    let winner = time_up_color.other();
                    // Stop the clock (it should already be stopped, but ensure it)
                    if clock.is_running {
                        clock.stop();
                    }
                    self.game.logic.game_state = GameState::Checkmate;
                    // Set player_turn to the winner so check_and_show_game_end shows correct winner
                    self.game.logic.player_turn = winner;
                    // Mark that the game ended due to time
                    self.game.logic.game_ended_by_time = true;
                    self.check_and_show_game_end();
                }
            }
        }

        // Check for opponent moves (Lichess or Multiplayer)
        // Skip if we're in puzzle mode
        if self.lichess_state.puzzle_game.is_some() {
            return; // Puzzles have all moves pre-loaded, no need to check for opponent moves
        }

        // For Lichess, we need to check here because moves come from polling
        // We check regardless of whose turn it is, because moves arrive asynchronously
        // and the turn state might be out of sync with the actual game state on Lichess
        // For TCP multiplayer, this is handled in main.rs
        if let Some(opponent) = self.game.logic.opponent.as_ref() {
            // Always check for Lichess moves - they arrive asynchronously via polling
            if let Some(crate::game_logic::opponent::OpponentKind::Lichess { .. }) = opponent.kind {
                // Check if there's a move available in the channel
                // execute_opponent_move() uses try_recv() which is non-blocking
                // This may also process status updates (GAME_STATUS messages)
                let move_executed = self.game.logic.execute_opponent_move();
                if move_executed {
                    log::info!("tick(): Opponent move executed, switching turn");
                    self.game.logic.switch_player_turn();
                    self.check_and_show_game_end();
                } else {
                    // Even if no move was executed, check for game end
                    // (status updates like draw/checkmate don't execute moves)
                    self.check_and_show_game_end();
                }
            } else {
                // For TCP multiplayer, only check when it's the opponent's turn
                let is_opponent_turn = self.game.logic.player_turn == opponent.color;
                if is_opponent_turn && self.game.logic.execute_opponent_move() {
                    self.game.logic.switch_player_turn();
                    self.check_and_show_game_end();
                }
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        // Cancel any active Lichess seek before quitting
        if let Some(cancellation_token) = &self.lichess_state.cancellation_token {
            cancellation_token.store(true, std::sync::atomic::Ordering::Relaxed);
            log::info!("Cancelling Lichess seek before quit");
        }
        self.running = false;
    }
}
