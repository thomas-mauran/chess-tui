use crate::app::App;
use crate::game_logic::bot::Bot;
use crate::sound::play_move_sound;
use shakmaty::{Color, Move};

impl App {

    pub fn bot_setup(&mut self) {
        let is_bot_starting =
            self.game_mode_state.selected_color.unwrap_or(Color::White) == shakmaty::Color::Black;
        let path = self.bot_state.chess_engine_path.as_deref().unwrap_or("");
        self.game.logic.bot = Some(Bot::new(
            path,
            is_bot_starting,
            self.bot_state.bot_depth,
            self.bot_state.bot_difficulty,
        ));

        // Initialize clock for bot games if time control is selected
        if let Some(seconds) = self.game_mode_state.get_time_control_seconds() {
            use crate::game_logic::clock::Clock;
            self.game.logic.clock = Some(Clock::new(seconds));
        }

        if let Some(color) = self.game_mode_state.selected_color {
            if color == Color::Black {
                // Flip the board once so Black player sees from their perspective
                self.game.logic.game_board.flip_the_board();

                if let Some(bot) = &self.game.logic.bot {
                    if self.game.logic.player_turn != color {
                        self.bot_state.start_bot_thinking(
                            self.game.logic.game_board.fen_position(),
                            bot.depth,
                            bot.difficulty,
                        );
                    }
                }
                // Don't set player_turn to Black here - the bot (White) moves first,
                // so player_turn should remain White until after the bot's first move
            }
        }

        // Ensure skin is preserved when setting up bot
        if let Some(skin) = &self.theme_state.loaded_skin {
            self.game.ui.skin = skin.clone();
        }

        self.update_config();
    }

       /// Check if bot move is ready and apply it
    pub fn check_and_apply_bot_move(&mut self) -> bool {
        if let Some(rx) = &self.bot_state.bot_move_receiver {
            if let Ok(bot_move) = rx.try_recv() {
                // Apply the bot move
                self.apply_bot_move(bot_move);
                self.bot_state.bot_move_receiver = None;
                return true;
            }
        }
        false
    }


    /// Apply a bot move to the game
    pub fn apply_bot_move(&mut self, bot_move: Move) {
        use shakmaty::Position;

        let current_position = match self.game.logic.game_board.current_position() {
            Some(pos) => pos.clone(),
            None => {
                log::error!("Cannot apply bot move: position history is empty");
                return;
            }
        };

        // Record captured piece (before applying the move) so material display is correct
        match &bot_move {
            Move::Normal { .. } => {
                if let Some(captured_piece) = current_position.board().piece_at(bot_move.to()) {
                    self.game.logic.game_board.taken_pieces.push(captured_piece);
                }
            }
            Move::EnPassant { .. } => {
                if let (Some(from_square), to_square) = (bot_move.from(), bot_move.to()) {
                    let captured_pawn_square =
                        shakmaty::Square::from_coords(to_square.file(), from_square.rank());
                    if let Some(captured_piece) =
                        current_position.board().piece_at(captured_pawn_square)
                    {
                        self.game.logic.game_board.taken_pieces.push(captured_piece);
                    }
                }
            }
            Move::Castle { .. } | Move::Put { .. } => {}
        }

        // Store in history
        let Some(from_square) = bot_move.from() else {
            log::error!("Bot move has no from square");
            return;
        };
        self.game.logic.game_board.move_history.push(Move::Normal {
            role: bot_move.role(),
            from: from_square,
            capture: bot_move.capture(),
            to: bot_move.to(),
            promotion: bot_move.promotion(),
        });

        let Ok(new_position) = current_position.play(&bot_move) else {
            log::error!("Failed to play bot move");
            return;
        };
        self.game
            .logic
            .game_board
            .position_history
            .push(new_position);
        // Reset history navigation when a new move is made
        self.game.logic.game_board.history_position_index = None;
        self.game.logic.game_board.original_flip_state = None;
        self.game.logic.switch_player_turn();

        // Play move sound
        play_move_sound();
    }
}