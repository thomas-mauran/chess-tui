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
    pub fn new(engine_path: &str, is_bot_starting: bool) -> Bot{
        let engine = match Bot::create_engine(engine_path) {
            Some(engine) => Some(engine),
            _ => panic!("An error occcured with the selected chess engine path: {engine_path} Make sure you specified the right path using chess-tui -e")
        };

        Self {
            engine,
            bot_will_move: false,
            is_bot_starting
        }
    }

    /// Allows you so set a
    pub fn set_engine(&mut self, engine_path: &str) {
        self.engine = Bot::create_engine(engine_path)
    }

    pub fn create_engine(engine_path: &str) -> Option<Engine> {
        match Engine::new(engine_path) {
            Ok(engine) => Some(engine),
            Err(e) => {
                panic!(
                    "Failed to initialize the engine at path: {}. Error: {:?}",
                    engine_path, e
                );
            }
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
