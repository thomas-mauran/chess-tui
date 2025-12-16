use ruci::{Engine, Go};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use std::borrow::Cow;
use std::process::Command;
use std::str::FromStr;

#[derive(Clone)]
pub struct Bot {
    pub engine_path: String,
    /// Used to indicate if a bot move is following
    pub bot_will_move: bool,
    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
    /// Bot thinking depth for chess engine
    pub depth: u8,
}

impl Bot {
    pub fn new(engine_path: &str, is_bot_starting: bool, depth: u8) -> Bot {
        Self {
            engine_path: engine_path.to_string(),
            bot_will_move: false,
            is_bot_starting,
            depth,
        }
    }

    pub fn get_move(&self, fen: &str) -> UciMove {
        // Parse engine_path to support command-line arguments
        // Split by spaces, treating first part as command and rest as args
        let parts: Vec<&str> = self.engine_path.split_whitespace().collect();
        let (command, args) = if parts.is_empty() {
            (self.engine_path.as_str(), &[] as &[&str])
        } else {
            (parts[0], &parts[1..])
        };

        let mut cmd = Command::new(command);
        if !args.is_empty() {
            cmd.args(args);
        }

        let mut process = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn engine process");

        let mut engine =
            Engine::from_process(&mut process, false).expect("Failed to initialize engine");

        engine
            .send(ruci::Position::Fen {
                fen: Cow::Owned(Fen::from_str(fen).unwrap()),
                moves: Cow::Borrowed(&[]),
            })
            .unwrap();

        engine
            .go(
                &Go {
                    depth: Some(self.depth as usize),
                    ..Default::default()
                },
                |_| {},
            )
            .unwrap()
            .take_normal()
            .unwrap()
            .r#move
    }
}
