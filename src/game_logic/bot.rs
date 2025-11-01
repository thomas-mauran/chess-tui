use ruci::{Engine, Go, NormalBestMove};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::{Move, Role};
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
    /// Bot thinking depth for chess engine
    pub depth: u8,
}

impl Bot {
    pub fn new(engine_path: &str, is_bot_starting: bool, depth: u8) -> Bot {
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
            depth,
        }
    }

    pub fn get_move(&self, fen: &str) -> UciMove {
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
                    depth: Some(self.depth as usize),
                    ..Default::default()
                },
                |_| {},
            )
            .unwrap()
            .take_normal()
            .unwrap();

        return best_move.r#move;
    }
}
