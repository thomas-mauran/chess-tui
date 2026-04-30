//! Game end detection and restart.

use crate::app::App;
use crate::constants::{Pages, Popups};
use crate::game_logic::game::Game;
use crate::game_logic::game::GameState;
use shakmaty::Color;

impl App {
    /// Shows the end screen popup. Uses the puzzle variant if a puzzle is active.
    pub fn show_end_screen(&mut self) {
        self.ui_state.current_popup = Some(if self.lichess_state.puzzle_game.is_some() {
            Popups::PuzzleEndScreen
        } else {
            Popups::EndScreen
        })
    }

    /// Updates game state and shows the end screen if the state just transitioned to checkmate or draw.
    /// Skipped on the PGN viewer page to avoid interfering with replay navigation.
    pub fn check_game_end_status(&mut self) {
        // PGN viewer drives its own game state via sync_pgn_to_board; running
        // update_game_state here would flip it to Checkmate/Draw on the final
        // position and open the EndScreen popup, which blocks viewer navigation.
        if self.ui_state.current_page == Pages::PgnViewer {
            return;
        }

        let previous_state = self.game.logic.game_state;
        self.game.logic.update_game_state();
        let new_state = self.game.logic.game_state;

        if previous_state != new_state
            && (new_state == GameState::Checkmate || new_state == GameState::Draw)
        {
            self.show_end_screen();
        }
    }

    /// Restarts the current game while keeping the same opponent and bot.
    /// Re-randomizes color if random color mode is active, re-initializes the clock, and
    /// triggers the bot's first move if it starts as White.
    pub fn restart(&mut self) {
        // Clear puzzle state when restarting (for normal games)
        self.lichess_state.puzzle_game = None;
        self.ui_state.end_screen_dismissed = false;
        self.game.logic.game_ended_by_time = false;
        let bot = self.game.logic.bot.clone();
        let opponent = self.game.logic.opponent.clone();
        // Preserve skin and display mode
        let current_skin = self.game.ui.skin.clone();
        let display_mode = self.game.ui.display_mode;
        // Check if we're in a local game (Solo page with no bot/opponent) or bot game to preserve clock
        let is_local_game =
            self.ui_state.current_page == Pages::Solo && bot.is_none() && opponent.is_none();
        let is_bot_game = bot.is_some() && opponent.is_none();

        if is_bot_game && self.game_mode_state.is_random_color {
            self.game_mode_state.selected_color = Some(if rand::random::<bool>() {
                Color::White
            } else {
                Color::Black
            });
        }

        self.game = Game::default();

        self.game.logic.bot = bot;
        self.game.logic.opponent = opponent;
        if let Some(bot) = self.game.logic.bot.as_mut() {
            bot.is_bot_starting =
                self.game_mode_state.selected_color.unwrap_or(Color::White) == Color::Black;
        }
        // Restore skin, display mode and piece styles
        self.game.ui.skin = current_skin;
        self.game.ui.display_mode = display_mode;
        self.game.ui.available_piece_styles = self.theme_state.available_piece_styles.clone();
        self.ui_state.close_popup();

        // Re-initialize clock for local games and bot games
        if (is_local_game || is_bot_game)
            && let Some(seconds) = self.game_mode_state.get_time_control_seconds()
        {
            use crate::game_logic::clock::Clock;
            self.game.logic.clock = Some(Clock::new(seconds));
        }

        if self
            .game
            .logic
            .bot
            .as_ref()
            .is_some_and(|bot| bot.is_bot_starting)
        {
            // Flip the board once so Black player sees from their perspective
            self.game.logic.game_board.flip_the_board();
            if let Some(bot) = &self.game.logic.bot {
                self.bot_state.start_bot_thinking(
                    self.game.logic.game_board.fen_position(),
                    bot.depth,
                    bot.difficulty,
                );
            }
            // Don't set player_turn to Black here - the bot (White) moves first,
            // so player_turn should remain White until after the bot's first move
        }
    }

    /// Checks for game end conditions after a move and shows end screen if needed.
    /// This consolidates the repeated game end checking logic.
    pub fn check_and_show_game_end(&mut self) {
        // Update game state first (this will stop the clock if game ended)
        self.game.logic.update_game_state();

        if self.game.logic.game_state == GameState::Checkmate
            || self.game.logic.game_state == GameState::Draw
        {
            // Game ended - only show end screen if it's not already shown and not dismissed
            if self.ui_state.current_popup != Some(Popups::EndScreen)
                && self.ui_state.current_popup != Some(Popups::PuzzleEndScreen)
                && !self.ui_state.end_screen_dismissed
            {
                self.show_end_screen();
            }
        } else {
            // Game is no longer ended, reset the dismissed flag
            self.ui_state.end_screen_dismissed = false;
        }
    }
}
