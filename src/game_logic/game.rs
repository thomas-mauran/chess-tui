use super::{bot::Bot, coord::Coord, game_board::GameBoard, opponent::Opponent, ui::UI};
use crate::{
    pieces::{PieceMove, PieceType},
    utils::get_int_from_char,
};
use shakmaty::{Color, Position};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum GameState {
    Checkmate,
    Draw,
    Playing,
    Promotion,
}

pub struct Game {
    /// The GameBoard storing data about the board related stuff
    pub game_board: GameBoard,
    /// The struct to handle UI related stuff
    pub ui: UI,
    /// The struct to handle Bot related stuff
    pub bot: Option<Bot>,
    /// The other player when playing in multiplayer
    pub opponent: Option<Opponent>,
    /// Which player is it to play
    pub player_turn: Color,
    /// The current state of the game (Playing, Draw, Checkmate. Promotion)
    pub game_state: GameState,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        let opponent_clone = self.opponent.as_ref().map(|p| Opponent {
            stream: p.stream.as_ref().and_then(|s| s.try_clone().ok()),
            opponent_will_move: p.opponent_will_move,
            color: p.color,
            game_started: p.game_started,
        });

        Game {
            game_board: self.game_board.clone(),
            ui: self.ui.clone(),
            bot: self.bot.clone(),
            opponent: opponent_clone,
            player_turn: self.player_turn,
            game_state: self.game_state,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            game_board: GameBoard::default(),
            ui: UI::default(),
            bot: None,
            opponent: None,
            player_turn: Color::White,
            game_state: GameState::Playing,
        }
    }
}

impl Game {
    // SETTERS
    pub fn new(game_board: GameBoard, player_turn: Color) -> Self {
        Self {
            game_board,
            ui: UI::default(),
            bot: None,
            opponent: None,
            player_turn,
            game_state: GameState::Playing,
        }
    }

    /// Allows you to pass a specific GameBoard
    pub fn set_board(&mut self, game_board: GameBoard) {
        self.game_board = game_board;
    }

    /// Allows you to set the player turn
    pub fn set_player_turn(&mut self, player_turn: Color) {
        self.player_turn = player_turn;
    }

    /// Switch the player turn
    pub fn switch_player_turn(&mut self) {
        match self.player_turn {
            Color::White => self.player_turn = Color::Black,
            Color::Black => self.player_turn = Color::White,
        }
    }

    // Methods to select a cell on the board
    pub fn handle_cell_click(&mut self) {
        // If we are doing a promotion the cursor is used for the popup
        if self.game_state == GameState::Promotion {
            self.handle_promotion();
        } else if !(self.game_state == GameState::Checkmate)
            && !(self.game_state == GameState::Draw)
        {
            if self.ui.is_cell_selected() {
                self.already_selected_cell_action();
            } else {
                self.select_cell()
            }
        }
        self.update_game_state();
    }

    fn update_game_state(&mut self) {
        if self.game_board.is_checkmate(self.player_turn) {
            self.game_state = GameState::Checkmate;
        } else if self.game_board.is_draw(self.player_turn) {
            self.game_state = GameState::Draw;
        } else if self.game_board.is_latest_move_promotion() {
            self.game_state = GameState::Promotion;
        }
    }

    pub fn handle_promotion(&mut self) {
        self.promote_piece();

        if self.opponent.is_some() {
            self.handle_multiplayer_promotion();
        }

        if self.bot.is_some() {
            self.execute_bot_move();
        }
    }
    pub fn already_selected_cell_action(&mut self) {
        // We already selected a piece so we apply the move
        if self.ui.cursor_coordinates.is_valid() {
            let selected_coords_usize = &self.ui.selected_coordinates.clone();
            let cursor_coords_usize = &self.ui.cursor_coordinates.clone();
            self.execute_move(selected_coords_usize, cursor_coords_usize);
            self.ui.unselect_cell();
            self.switch_player_turn();

            if self.game_board.is_draw(self.player_turn) {
                self.game_state = GameState::Draw;
            }

            // Only flip the board in single-player mode (no bot, no opponent)
            if self.bot.is_none()
                && self.opponent.is_none()
                && (!self.game_board.is_latest_move_promotion()
                    || self.game_board.is_draw(self.player_turn)
                    || self.game_board.is_checkmate(self.player_turn))
            {
                self.game_board.flip_the_board();
            }

            // If we play against a bot we will play his move and switch the player turn again
            if self.bot.is_some() {
                // do this in background
                if self.game_board.is_latest_move_promotion() {
                    self.game_state = GameState::Promotion;
                }

                if !(self.game_state == GameState::Promotion) {
                    if self.game_board.is_checkmate(self.player_turn) {
                        self.game_state = GameState::Checkmate;
                    }

                    if self.game_board.is_draw(self.player_turn) {
                        self.game_state = GameState::Draw;
                    }

                    if !(self.game_state == GameState::Checkmate) {
                        if let Some(bot) = self.bot.as_mut() {
                            bot.bot_will_move = true;
                        }
                    }
                }
            }
            // If we play against a player we will wait for his move
            if self.opponent.is_some() {
                if self.game_board.is_latest_move_promotion() {
                    self.game_state = GameState::Promotion;
                } else {
                    if self.game_board.is_checkmate(self.player_turn) {
                        self.game_state = GameState::Checkmate;
                    }

                    if self.game_board.is_draw(self.player_turn) {
                        self.game_state = GameState::Draw;
                    }

                    if !(self.game_state == GameState::Checkmate) {
                        if let Some(opponent) = self.opponent.as_mut() {
                            opponent.opponent_will_move = true;
                        }
                    }
                    self.opponent
                        .as_mut()
                        .unwrap()
                        .send_move_to_server(self.game_board.move_history.last().unwrap(), None);
                }
            }
        }
    }

    pub fn select_cell(&mut self) {
        // Check if the piece on the cell can move before selecting it
        let authorized_positions = self
            .game_board
            .get_authorized_positions(self.player_turn, self.ui.cursor_coordinates);

        if authorized_positions.is_empty() {
            return;
        }
        if let Some(piece_color) = self.game_board.get_piece_color(&self.ui.cursor_coordinates) {
            let authorized_positions = self
                .game_board
                .get_authorized_positions(self.player_turn, self.ui.cursor_coordinates);

            if piece_color == self.player_turn {
                self.ui.selected_coordinates = self.ui.cursor_coordinates;
                self.ui.old_cursor_position = self.ui.cursor_coordinates;
                self.ui
                    .move_selected_piece_cursor(true, 1, authorized_positions);
            }
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

        let from_y = get_int_from_char(bot_move.chars().next());
        let from_x = get_int_from_char(bot_move.chars().nth(1));
        let to_y = get_int_from_char(bot_move.chars().nth(2));
        let to_x = get_int_from_char(bot_move.chars().nth(3));

        let promotion_piece: Option<PieceType> = if bot_move.chars().count() == 5 {
            match bot_move.chars().nth(4) {
                Some('q') => Some(PieceType::Queen),
                Some('r') => Some(PieceType::Rook),
                Some('b') => Some(PieceType::Bishop),
                Some('n') => Some(PieceType::Knight),
                _ => None,
            }
        } else {
            None
        };

        let from = Coord::new(from_y, from_x);
        let to = Coord::new(to_y, to_x);

        // Get piece type before executing move
        // We need to query using standard coordinates since execute_standard_move expects them
        let piece_type_from = {
            let from_square = from.to_square();
            if let Some(square) = from_square {
                let chess = self.game_board.position_history.last().unwrap();
                chess
                    .board()
                    .piece_at(square)
                    .map(|p| PieceType::from_role(p.role))
            } else {
                None
            }
        };

        // Execute move with promotion if needed (using standard coordinates from UCI)
        if !self
            .game_board
            .execute_standard_move(from, to, promotion_piece)
        {
            return;
        }

        // Store in history (convert to visual coordinates if board is flipped)
        if let Some(piece_type) = piece_type_from {
            let (visual_from, visual_to) = if self.game_board.is_flipped {
                (
                    Coord::new(7 - from.row, 7 - from.col),
                    Coord::new(7 - to.row, 7 - to.col),
                )
            } else {
                (from, to)
            };

            self.game_board.move_history.push(PieceMove {
                piece_type,
                piece_color: self.player_turn,
                from: visual_from,
                to: visual_to,
            });

        }
    }

    // Method to promote a pawn
    pub fn promote_piece(&mut self) {
        if let Some(last_move) = self.game_board.move_history.last().cloned() {
            let new_piece = match self.ui.promotion_cursor {
                0 => PieceType::Queen,
                1 => PieceType::Rook,
                2 => PieceType::Bishop,
                3 => PieceType::Knight,
                _ => unreachable!("Promotion cursor out of boundaries"),
            };

            // Remove the last move and position from history
            self.game_board.move_history.pop();
            self.game_board.position_history.pop();

            // Re-execute the move with the correct promotion
            if self.game_board.execute_shakmaty_move_with_promotion(
                last_move.from,
                last_move.to,
                Some(new_piece),
            ) {
                // Update move history with correct piece type
                self.game_board.move_history.push(PieceMove {
                    piece_type: new_piece,
                    piece_color: last_move.piece_color,
                    from: last_move.from,
                    to: last_move.to,
                });
            }
        }

        self.game_state = GameState::Playing;
        self.ui.promotion_cursor = 0;
        if !self.game_board.is_draw(self.player_turn)
            && !self.game_board.is_checkmate(self.player_turn)
            && self.opponent.is_none()
            && self.bot.is_none()
        {
            self.game_board.flip_the_board();
        }
    }

    /// Move a piece from a cell to another
    // TODO: Split this in multiple methods
    pub fn execute_move(&mut self, from: &Coord, to: &Coord) {
        if !from.is_valid() || !to.is_valid() {
            return;
        }

        let piece_type_from = self.game_board.get_piece_type(from);
        let piece_type_to = self.game_board.get_piece_type(to);

        // Check if moving a piece
        let Some(piece_type_from) = piece_type_from else {
            return;
        };

        // Execute the move using shakmaty (this updates both position_history and the board)
        if !self.game_board.execute_shakmaty_move(*from, *to) {
            return; // Invalid move
        }

        // We increment the consecutive_non_pawn_or_capture if the piece type is a pawn or if there is no capture
        self.game_board
            .increment_consecutive_non_pawn_or_capture(piece_type_from, piece_type_to);

        // We store it in the history
        self.game_board.move_history.push(PieceMove {
            piece_type: piece_type_from,
            piece_color: self.player_turn,
            from: *from,
            to: *to,
        });
    }

    pub fn execute_opponent_move(&mut self) {
        let opponent_move = self.opponent.as_mut().unwrap().read_stream();
        self.game_board.flip_the_board();
        self.opponent.as_mut().unwrap().opponent_will_move = false;

        if opponent_move.is_empty() {
            return;
        }

        let from_y = get_int_from_char(opponent_move.chars().next());
        let from_x = get_int_from_char(opponent_move.chars().nth(1));
        let to_y = get_int_from_char(opponent_move.chars().nth(2));
        let to_x = get_int_from_char(opponent_move.chars().nth(3));

        let promotion_piece: Option<PieceType> = if opponent_move.chars().count() == 5 {
            match opponent_move.chars().nth(4) {
                Some('q') => Some(PieceType::Queen),
                Some('r') => Some(PieceType::Rook),
                Some('b') => Some(PieceType::Bishop),
                Some('n') => Some(PieceType::Knight),
                _ => None,
            }
        } else {
            None
        };

        let from = Coord::new(from_y, from_x);
        let to = Coord::new(to_y, to_x);

        // Get piece type before executing move
        // For opponent moves, we need to account for board flip to get the piece type correctly
        let visual_from = if self.game_board.is_flipped {
            Coord::new(7 - from.row, 7 - from.col)
        } else {
            from
        };
        let piece_type_from = self.game_board.get_piece_type(&visual_from);

        // Execute move with promotion if needed (using standard coordinates)
        if !self
            .game_board
            .execute_standard_move(from, to, promotion_piece)
        {
            self.game_board.flip_the_board();
            return;
        }

        // Store in history (use visual coordinates for history)
        if let Some(piece_type) = piece_type_from {
            self.game_board.move_history.push(PieceMove {
                piece_type,
                piece_color: self.player_turn,
                from: visual_from,
                to: if self.game_board.is_flipped {
                    Coord::new(7 - to.row, 7 - to.col)
                } else {
                    to
                },
            });
        }

        self.game_board.flip_the_board();
    }

    pub fn handle_multiplayer_promotion(&mut self) {
        let opponent = self.opponent.as_mut().unwrap();

        let last_move_promotion_type = self.game_board.get_last_move_piece_type_as_string();

        opponent.send_move_to_server(
            self.game_board.move_history.last().unwrap(),
            Some(last_move_promotion_type),
        );
        opponent.opponent_will_move = true;
    }
}
