use crate::utils::convert_notation_into_position;
use ruci::{Engine, Go};
use shakmaty::fen::Fen;
use std::borrow::Cow;
use std::cell::RefCell;
use std::io::BufReader;
use std::process::{Child, ChildStdin, ChildStdout, Command};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone)]
pub struct Bot {
    // TODO, FIXME: Don't reuse the same process... Chess engines are not meant to be used like this
    #[allow(dead_code)]
    process: Rc<RefCell<Child>>,
    engine: Rc<RefCell<Engine<BufReader<ChildStdout>, ChildStdin>>>,
    /// Used to indicate if a bot move is following
    pub bot_will_move: bool,
    // if the bot is starting, meaning the player is black
    pub is_bot_starting: bool,
}

impl Bot {
    pub fn new(engine_path: &str, is_bot_starting: bool) -> Bot {
        let mut process = Command::new(engine_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let engine = Rc::new(RefCell::new(
            Engine::from_process(&mut process, false).unwrap(),
        ));

        Self {
            process: Rc::new(RefCell::new(process)),
            engine,
            bot_will_move: false,
            is_bot_starting,
        }
    }

    pub fn get_move(&self, fen: &str) -> String {
        let mut engine = self.engine.borrow_mut();

        engine
            .send(ruci::Position::Fen {
                fen: Cow::Owned(Fen::from_str(fen).unwrap()),
                moves: Cow::Borrowed(&[]),
            })
            .unwrap();

        let best_move = engine
            .go(
                &Go {
                    depth: Some(10),
                    ..Default::default()
                },
                |_| {},
            )
            .unwrap()
            .take_normal()
            .unwrap();

        convert_notation_into_position(&best_move.r#move.to_string())
    }
}
