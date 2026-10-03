use crate::{
    app::{App, AppResult},
    event::Event,
    game_logic::opponent::wait_for_game_start,
    handlers::handler::{handle_key_events, handle_mouse_events},
    ui::tui::Tui,
};

impl App {
    pub fn run(&mut self) -> AppResult<()> {
        // Initialize the terminal user interface.
        let mut tui = Tui::new()?;

        // Start the main loop.
        while self.running {
            // Render the user interface.
            tui.draw(self)?;

            self.handle_events(&mut tui)?;

            self.check_bot_thinking();

            // Check if bot move is ready
            if self.check_and_apply_bot_move() {
                self.check_and_show_game_end();
            }

            self.check_lichess_updates();

            // Check if game ended
            self.check_game_end_status();

            // Persist local/bot games each loop iteration. A move always
            // triggers a write; clock-only changes are throttled to ~1 Hz inside
            // so we don't hit the disk on every animation frame.
            self.tick_resume_state();

            self.check_multiplayer_game_start();

            self.wait_for_opponent_move(&mut tui)?;
        }

        Ok(())
    }

    fn handle_events(&mut self, tui: &mut Tui) -> AppResult<()> {
        // Handle events fast loop during animations, blocking otherwise.
        if self.animations.is_active() {
            match tui.events.try_next() {
                Some(Event::Tick) => self.tick(),
                Some(Event::Key(key_event)) => handle_key_events(key_event, self)?,
                Some(Event::Mouse(mouse_event)) => handle_mouse_events(mouse_event, self)?,
                Some(Event::Resize(_, _)) | None => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        } else {
            match tui.events.next()? {
                Event::Tick => self.tick(),
                Event::Key(key_event) => handle_key_events(key_event, self)?,
                Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, self)?,
                Event::Resize(_, _) => {}
            }
        };

        Ok(())
    }

    /// Check if bot should start thinking
    fn check_bot_thinking(&mut self) {
        let bot_thinking = self.bot_state.is_bot_thinking();
        let bot_will_move = self
            .game
            .logic
            .bot
            .as_ref()
            .is_some_and(|bot| bot.bot_will_move);

        if bot_thinking || !bot_will_move {
            return;
        }

        if let Some(bot) = &self.game.logic.bot {
            self.bot_state.start_bot_thinking(
                self.game.logic.game_board.fen_position(),
                bot.depth,
                bot.difficulty,
            );
        }

        if let Some(bot) = self.game.logic.bot.as_mut() {
            bot.bot_will_move = false;
        }
    }

    fn check_multiplayer_game_start(&mut self) {
        // Check if hosting player received game start signal from background thread
        if let Some(ref game_start_rx) = self.multiplayer_state.game_start_rx
            && let Ok(()) = game_start_rx.try_recv()
            && let Some(opponent) = self.game.logic.opponent.as_mut()
        {
            log::info!("Host received game start signal, starting game");
            opponent.game_started = true;
            self.ui_state.close_popup();
        }

        // For non-hosting players, check directly on the stream
        if let Some(opponent) = self.game.logic.opponent.as_mut()
            && !opponent.game_started
            && self.multiplayer_state.game_start_rx.is_none()
        {
            match wait_for_game_start(opponent) {
                Ok(true) => {
                    opponent.game_started = true;
                    self.ui_state.close_popup();
                }
                Ok(false) => {
                    // Still waiting, do nothing
                }
                Err(e) => {
                    log::error!("Error waiting for game start: {}", e);
                }
            }
        }
    }

    /// If it's the opponent turn, wait for the opponent to move
    /// Only check when it's actually the opponent's turn to avoid unnecessary work
    fn wait_for_opponent_move(&mut self, tui: &mut Tui) -> AppResult<()> {
        let Some(opponent) = self.game.logic.opponent.as_ref() else {
            return Ok(());
        };

        // Check if it's the opponent's turn: if player_turn == opponent.color, it's opponent's turn
        let is_opponent_turn = self.game.logic.player_turn == opponent.color;

        // Only check for TCP multiplayer moves here (Lichess is handled in tick())
        // Check both the turn and the flag for TCP multiplayer
        //
        // Check if it's TCP (not Lichess) - Lichess is handled in tick()
        if !is_opponent_turn || !opponent.opponent_will_move || !opponent.is_tcp_multiplayer() {
            return Ok(());
        }

        tui.draw(self)?;

        let is_checkmate = self.game.logic.game_board.is_checkmate();
        let is_draw = self.game.logic.game_board.is_draw();

        if !is_checkmate && !is_draw && self.game.logic.execute_opponent_move() {
            self.game.switch_player_turn();
        }

        // need to be centralised
        self.check_game_end_status();
        tui.draw(self)?;

        Ok(())
    }
}
