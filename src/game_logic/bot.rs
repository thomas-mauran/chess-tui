use uci::Engine;

use crate::utils::convert_notation_into_position;

#[derive(Default, Clone)]
pub struct Bot {
    // the chess engine
    pub engine: Option<Engine>,
    /// Used to indicate if a bot move is following
    pub bot_will_move: bool,
    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
}

impl Bot {
    /// Allows you so set a
    pub fn set_engine(&mut self, engine_path: &str) {
        self.engine = match Engine::new(engine_path) {
            Ok(engine) => Some(engine),
            _ => panic!("An error occcured with the selected chess engine path: {engine_path} Make sure you specified the right path using chess-tui -e"),
        }
    }

    /* Method to make a move for the bot
       We use the UCI protocol to communicate with the chess engine
    */
    pub fn get_bot_move(&mut self, fen_position: String) -> String {
        let engine = self.engine.clone().expect("Missing the chess engine");

        engine.set_position(&(fen_position as String)).unwrap();
        let best_move = engine.bestmove();
        let Ok(movement) = best_move else {
            panic!("An error has occured")
        };

        convert_notation_into_position(&movement)
    }
}
