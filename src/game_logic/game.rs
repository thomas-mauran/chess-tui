use super::{bot::Bot, coord::Coord, game_board::GameBoard, opponent::Opponent, ui::UI};
use crate::utils::flip_square_if_needed;
use shakmaty::{Color, Move, Position, Role, Square};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum GameState {
    Checkmate,
    Draw,
    Playing,
    Promotion,
}

impl Clone for GameLogic {
    fn clone(&self) -> Self {
        let opponent_clone = self.opponent.as_ref().map(|p| Opponent {
            stream: p.stream.as_ref().and_then(|s| s.try_clone().ok()),
            opponent_will_move: p.opponent_will_move,
            color: p.color,
            game_started: p.game_started,
        });

        GameLogic {
            game_board: self.game_board.clone(),
            bot: self.bot.clone(),
            opponent: opponent_clone,
            player_turn: self.player_turn,
            game_state: self.game_state,
        }
    }
}

pub struct GameLogic {
    /// The GameBoard storing data about the board related stuff
    pub game_board: GameBoard,
    /// The struct to handle Bot related stuff
    pub bot: Option<Bot>,
    /// The other player when playing in multiplayer
    pub opponent: Option<Opponent>,
    /// Which player is it to play
    pub player_turn: Color,
    /// The current state of the game (Playing, Draw, Checkmate. Promotion)
    pub game_state: GameState,
}

impl Default for GameLogic {
    fn default() -> Self {
        Self {
            game_board: GameBoard::default(),
            bot: None,
            opponent: None,
            player_turn: Color::White,
            game_state: GameState::Playing,
        }
    }
}

#[derive(Default)]
pub struct Game {
    pub logic: GameLogic,
    /// The struct to handle UI related stuff
    pub ui: UI,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            logic: self.logic.clone(),
            ui: self.ui.clone(),
        }
    }
}

impl Game {
    // SETTERS
    pub fn new(game_board: GameBoard, player_turn: Color) -> Self {
        Self {
            logic: GameLogic {
                game_board,
                player_turn,
                ..Default::default()
            },
            ui: UI::default(),
        }
    }

    /// Allows you to pass a specific GameBoard
    pub fn set_board(&mut self, game_board: GameBoard) {
        self.logic.game_board = game_board;
    }

    /// Allows you to set the player turn
    pub fn set_player_turn(&mut self, player_turn: Color) {
        self.logic.player_turn = player_turn;
    }

    /// Switch the player turn
    pub fn switch_player_turn(&mut self) {
        self.logic.switch_player_turn();
    }

    // Methods to select a cell on the board
    pub fn handle_cell_click(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.logic.game_state == GameState::Promotion {
            self.handle_promotion();
        } else if !(self.logic.game_state == GameState::Checkmate)
            && !(self.logic.game_state == GameState::Draw)
        {
            if self.ui.is_cell_selected() {
                self.already_selected_cell_action();
            } else {
                self.select_cell()
            }
        }
        self.logic.update_game_state();
    }

    pub fn handle_promotion(&mut self) {
        self.logic
            .promote_piece(self.ui.promotion_cursor.try_into().unwrap());
        self.ui.promotion_cursor = 0;

        if self.logic.opponent.is_some() {
            self.logic.handle_multiplayer_promotion();
        }

        if self.logic.bot.is_some() {
            self.logic.execute_bot_move();
        }
    }
    pub fn already_selected_cell_action(&mut self) {
        if self.ui.selected_square.is_none() {
            return;
        }

        // We already selected a piece so we apply the move
        if self.ui.cursor_coordinates.is_valid() {
            let selected_coords_usize = &flip_square_if_needed(
                self.ui.selected_square.unwrap(),
                self.logic.game_board.is_flipped,
            );

            let actual_cursor_coords = flip_square_if_needed(
                self.ui.cursor_coordinates.to_square().unwrap(),
                self.logic.game_board.is_flipped,
            );

            self.logic
                .execute_move(*selected_coords_usize, actual_cursor_coords);
            self.ui.unselect_cell();
            self.logic.switch_player_turn();

            if self.logic.game_board.is_draw(self.logic.player_turn) {
                self.logic.game_state = GameState::Draw;
            }

            // Only flip the board in single-player mode (no bot, no opponent)
            if self.logic.bot.is_none()
                && self.logic.opponent.is_none()
                && (!self.logic.game_board.is_latest_move_promotion()
                    || self.logic.game_board.is_draw(self.logic.player_turn)
                    || self.logic.game_board.is_checkmate())
            {
                self.logic.game_board.flip_the_board();
            }

            // If we play against a bot we will play his move and switch the player turn again
            if self.logic.bot.is_some() {
                // do this in background
                if self.logic.game_board.is_latest_move_promotion() {
                    self.logic.game_state = GameState::Promotion;
                }

                if !(self.logic.game_state == GameState::Promotion) {
                    if self.logic.game_board.is_checkmate() {
                        self.logic.game_state = GameState::Checkmate;
                    }

                    if self.logic.game_board.is_draw(self.logic.player_turn) {
                        self.logic.game_state = GameState::Draw;
                    }

                    if !(self.logic.game_state == GameState::Checkmate) {
                        if let Some(bot) = self.logic.bot.as_mut() {
                            bot.bot_will_move = true;
                        }
                    }
                }
            }
            // If we play against a player we will wait for his move
            if self.logic.opponent.is_some() {
                if self.logic.game_board.is_latest_move_promotion() {
                    self.logic.game_state = GameState::Promotion;
                } else {
                    if self.logic.game_board.is_checkmate() {
                        self.logic.game_state = GameState::Checkmate;
                    }

                    if self.logic.game_board.is_draw(self.logic.player_turn) {
                        self.logic.game_state = GameState::Draw;
                    }

                    if !(self.logic.game_state == GameState::Checkmate) {
                        if let Some(opponent) = self.logic.opponent.as_mut() {
                            opponent.opponent_will_move = true;
                        }
                    }
                    // check for the promotion piece
                    let last_move_promotion = self
                        .logic
                        .game_board
                        .move_history
                        .last()
                        .unwrap()
                        .promotion();
                    self.logic.opponent.as_mut().unwrap().send_move_to_server(
                        self.logic.game_board.move_history.last().unwrap(),
                        last_move_promotion,
                    );
                }
            }
        }
    }

    pub fn select_cell(&mut self) {
        let square = self
            .ui
            .cursor_coordinates
            .to_square()
            .map(|s| Coord::from_square(s).to_square().unwrap())
            .unwrap();
        let actual_square = flip_square_if_needed(square, self.logic.game_board.is_flipped);

        // Check if there is a piece on the cell or if the cells is the right color
        if self
            .logic
            .game_board
            .get_role_at_square(&actual_square)
            .is_none()
            || self
                .logic
                .game_board
                .get_piece_color_at_square(&actual_square)
                != Some(self.logic.player_turn)
        {
            return;
        }

        // Check if the piece on the cell can move before selecting it
        let authorized_positions = self
            .logic
            .game_board
            .get_authorized_positions(self.logic.player_turn, &actual_square);

        if authorized_positions.is_empty() {
            return;
        }
        if let Some(piece_color) = self
            .logic
            .game_board
            .get_piece_color_at_square(&actual_square)
        {
            let authorized_positions = self
                .logic
                .game_board
                .get_authorized_positions(self.logic.player_turn, &actual_square);

            if piece_color == self.logic.player_turn {
                self.ui.selected_square = Some(square);
                self.ui.old_cursor_position = self.ui.cursor_coordinates;

                let authorized_positions_flipped: Vec<Square> = authorized_positions
                    .iter()
                    .map(|s| flip_square_if_needed(*s, self.logic.game_board.is_flipped))
                    .collect();

                self.ui.move_selected_piece_cursor(
                    true,
                    1,
                    authorized_positions_flipped
                        .iter()
                        .map(|s| Coord::from_square(*s))
                        .collect(),
                );
            }
        }
    }
}

impl GameLogic {
    fn update_game_state(&mut self) {
        if self.game_board.is_checkmate() {
            self.game_state = GameState::Checkmate;
        } else if self.game_board.is_draw(self.player_turn) {
            self.game_state = GameState::Draw;
        } else if self.game_board.is_latest_move_promotion() {
            self.game_state = GameState::Promotion;
        }
    }

    /// Switch the player turn
    pub fn switch_player_turn(&mut self) {
        match self.player_turn {
            Color::White => self.player_turn = Color::Black,
            Color::Black => self.player_turn = Color::White,
        }
    }

    /* Method to make a move for the bot
       We use the UCI protocol to communicate with the chess engine
    */
    pub fn execute_bot_move(&mut self) {
        // Safely extract bot out of self to reduce overlapping borrows
        let is_bot_starting = if let Some(bot) = self.bot.as_ref() {
            bot.is_bot_starting
        } else {
            return;
        };

        let fen_position = self
            .game_board
            .fen_position(is_bot_starting, self.player_turn);

        // Retrieve the bot move from the bot
        let bot_move = if let Some(bot) = self.bot.as_mut() {
            bot.get_move(&fen_position)
        } else {
            return;
        };

        // Convert UCI move to a shakmaty Move using the current position
        let current_position = self.game_board.position_history.last().unwrap().clone();
        let bot_actual_move: Move = bot_move
            .to_move(&current_position)
            .expect("Engine returned illegal UCI move for current position");

        // Store in history (convert to visual coordinates if board is flipped)
        self.game_board.move_history.push(Move::Normal {
            role: bot_actual_move.role(),
            from: bot_actual_move.from().unwrap(),
            capture: bot_actual_move.capture(),
            to: bot_actual_move.to(),
            promotion: bot_actual_move.promotion(),
        });

        self.game_board
            .position_history
            .push(current_position.play(&bot_actual_move).unwrap());
    }

    // Method to promote a pawn
    pub fn promote_piece(&mut self, promotion_cursor: u8) {
        if let Some(last_move) = self.game_board.move_history.last().cloned() {
            let new_piece = match promotion_cursor {
                0 => Role::Queen,
                1 => Role::Rook,
                2 => Role::Bishop,
                3 => Role::Knight,
                _ => unreachable!("Promotion cursor out of boundaries"),
            };

            // Remove the last move and position from history
            self.game_board.move_history.pop();
            self.game_board.position_history.pop();

            // Re-execute the move with the correct promotion
            if self.game_board.execute_shakmaty_move_with_promotion(
                last_move.from().unwrap(),
                last_move.to(),
                Some(new_piece),
            ) {
                // Update move history with correct piece type
                self.game_board.move_history.push(Move::Normal {
                    role: Role::Pawn,
                    from: last_move.from().unwrap(),
                    capture: last_move.capture(),
                    to: last_move.to(),
                    promotion: Some(new_piece),
                });
            }
        }

        self.game_state = GameState::Playing;
        if !self.game_board.is_draw(self.player_turn)
            && !self.game_board.is_checkmate()
            && self.opponent.is_none()
            && self.bot.is_none()
        {
            self.game_board.flip_the_board();
        }
    }

    /// Move a piece from a cell to another
    pub fn execute_move(&mut self, from: Square, to: Square) {
        // Check if moving a piece
        if self.game_board.get_role_at_square(&from).is_none() {
            return;
        };

        let role_from: Option<Role> = self.game_board.get_role_at_square(&from);
        let role_to: Option<Role> = self.game_board.get_role_at_square(&to);

        if !self.game_board.execute_shakmaty_move(from, to) {
            return;
        }
        // We increment the consecutive_non_pawn_or_capture if the piece type is a pawn or if there is no capture
        self.game_board
            .increment_consecutive_non_pawn_or_capture(role_from.unwrap(), role_to);

        // We store it in the history
        self.game_board.move_history.push(Move::Normal {
            role: role_from.unwrap(),
            from,
            capture: None, // TODO FIX THAT
            to,
            promotion: None, // TODO FIX THAT
        });
    }

    pub fn execute_opponent_move(&mut self) -> bool {
        let opponent = if let Some(opp) = self.opponent.as_mut() {
            opp
        } else {
            return false;
        };

        let opponent_move = opponent.read_stream();

        if opponent_move.is_empty() {
            return false;
        }

        // We received a move, so we can stop waiting
        opponent.opponent_will_move = false;

        if opponent_move.len() < 4 {
            return false;
        }

        let mut chars = opponent_move.chars();
        let from_file_char = chars.next();
        let from_rank_char = chars.next();
        let to_file_char = chars.next();
        let to_x_char = chars.next();

        let promotion_piece: Option<Role> = if opponent_move.chars().count() == 5 {
            match opponent_move.chars().nth(4) {
                Some('q') => Some(Role::Queen),
                Some('r') => Some(Role::Rook),
                Some('b') => Some(Role::Bishop),
                Some('n') => Some(Role::Knight),
                _ => None,
            }
        } else {
            None
        };

        // The move is in standard chess notation (e.g., "e2e4")
        // Files are letters (a-h), ranks are digits (1-8)
        let from_str = format!(
            "{}{}",
            from_file_char.unwrap_or('a'),
            from_rank_char.unwrap_or('1')
        );
        let to_str = format!(
            "{}{}",
            to_file_char.unwrap_or('a'),
            to_x_char.unwrap_or('1')
        );

        let from = match Square::from_ascii(from_str.as_bytes()) {
            Ok(sq) => sq,
            Err(_) => return false,
        };
        let to = match Square::from_ascii(to_str.as_bytes()) {
            Ok(sq) => sq,
            Err(_) => return false,
        };

        let piece_type_from = self.game_board.get_role_at_square(&from);

        // Execute move with promotion if needed (using standard coordinates)
        let executed_move = self
            .game_board
            .execute_standard_move(from, to, promotion_piece);

        if executed_move.is_none() {
            return false;
        }

        // Store in history (use visual coordinates for history)
        if let Some(piece_type) = piece_type_from {
            let move_to_store = executed_move.unwrap();
            self.game_board.move_history.push(Move::Normal {
                role: piece_type,
                from,
                capture: move_to_store.capture(),
                to,
                promotion: move_to_store.promotion(),
            });
        }
        true
    }

    pub fn handle_multiplayer_promotion(&mut self) {
        let opponent = self.opponent.as_mut().unwrap();

        let last_move_promotion_type = self.game_board.move_history.last().unwrap().promotion();

        opponent.send_move_to_server(
            self.game_board.move_history.last().unwrap(),
            last_move_promotion_type,
        );
        opponent.opponent_will_move = true;
    }
}
