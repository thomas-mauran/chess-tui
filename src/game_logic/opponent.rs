use crate::constants::NETWORK_BUFFER_SIZE;
use log;
use shakmaty::{Color, Move, Role, Square};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

#[derive(Debug)]
pub enum OpponentKind {
    Tcp(TcpStream),
    Lichess {
        game_id: String,
        move_rx: Receiver<String>,
        move_tx: Sender<String>,
        player_move_tx: Option<Sender<()>>,
    },
}

pub struct Opponent {
    pub kind: Option<OpponentKind>,
    /// Used to indicate if a Opponent move is following
    pub opponent_will_move: bool,
    // The color of the Opponent
    pub color: Color,
    /// Is Game started
    pub game_started: bool,
    /// Number of moves already played when joining (for ongoing games)
    pub initial_move_count: usize,
    /// Counter for moves received from stream
    pub moves_received: usize,
}

// Custom Default implementation
impl Default for Opponent {
    fn default() -> Self {
        Opponent {
            kind: None,
            opponent_will_move: false,
            color: Color::Black,
            game_started: false,
            initial_move_count: 0,
            moves_received: 0,
        }
    }
}

impl Clone for Opponent {
    fn clone(&self) -> Self {
        // TcpStream cannot be cloned easily, and channels neither.
        // For now, we might need to rethink Clone or just set kind to None for clones.
        // The original code tried to clone TcpStream.
        let kind = match &self.kind {
            Some(OpponentKind::Tcp(stream)) => stream.try_clone().ok().map(OpponentKind::Tcp),
            Some(OpponentKind::Lichess { .. }) => None, // Cannot clone channels or senders
            None => None,
        };

        Opponent {
            kind,
            opponent_will_move: self.opponent_will_move,
            color: self.color,
            game_started: self.game_started,
            initial_move_count: self.initial_move_count,
            moves_received: self.moves_received,
        }
    }
}

impl Opponent {
    pub fn copy(&self) -> Self {
        Opponent {
            kind: None,
            opponent_will_move: self.opponent_will_move,
            color: self.color,
            game_started: self.game_started,
            initial_move_count: self.initial_move_count,
            moves_received: self.moves_received,
        }
    }

    /// Check if this opponent is a TCP multiplayer connection
    pub fn is_tcp_multiplayer(&self) -> bool {
        matches!(self.kind, Some(OpponentKind::Tcp(_)))
    }

    /// Check if this opponent is a Lichess connection
    pub fn is_lichess(&self) -> bool {
        matches!(self.kind, Some(OpponentKind::Lichess { .. }))
    }

    pub fn new(addr: String, color: Option<Color>) -> Result<Opponent, String> {
        log::info!(
            "Creating new opponent with addr: {} and color: {:?}",
            addr,
            color
        );

        // Attempt to connect 5 times to the provided address
        let mut stream: Option<TcpStream> = None;
        for attempt in 1..=5 {
            log::debug!("Connection attempt {} to {}", attempt, addr);
            match TcpStream::connect(addr.clone()) {
                Ok(s) => {
                    log::info!("Successfully connected to server");
                    stream = Some(s);
                    break;
                }
                Err(e) => {
                    log::error!("Failed connection attempt {} to {}: {}", attempt, addr, e);
                }
            }
        }

        if let Some(stream) = stream {
            let color = match color {
                Some(color) => {
                    log::info!("Using provided color: {:?}", color);
                    color
                }
                None => {
                    log::info!("Getting color from stream");
                    get_color_from_stream(&stream)?
                }
            };

            let opponent_will_move = match color {
                Color::White => true,
                Color::Black => false,
            };
            log::info!(
                "Created opponent with color {:?}, will_move: {}",
                color,
                opponent_will_move
            );

            // Set stream to non-blocking mode
            if let Err(e) = stream.set_nonblocking(true) {
                log::error!("Failed to set stream to non-blocking: {}", e);
                return Err(format!("Failed to set stream to non-blocking: {}", e));
            }

            Ok(Opponent {
                kind: Some(OpponentKind::Tcp(stream)),
                opponent_will_move,
                color,
                game_started: false,
                initial_move_count: 0,
                moves_received: 0,
            })
        } else {
            log::error!("Failed to connect after 5 attempts to {}", addr);
            Err(format!(
                "Failed to connect to the server after 5 attempts to the following address {}",
                addr
            ))
        }
    }

    pub fn new_lichess(
        game_id: String,
        color: Color,
        move_rx: Receiver<String>,
        move_tx: Sender<String>,
        initial_move_count: usize,
        player_move_tx: Option<Sender<()>>,
    ) -> Self {
        let opponent_will_move = color == Color::White;
        Opponent {
            kind: Some(OpponentKind::Lichess {
                game_id,
                move_rx,
                move_tx,
                player_move_tx,
            }),
            opponent_will_move,
            color,
            game_started: true, // Lichess game starts immediately when we join/seek
            initial_move_count,
            moves_received: 0,
        }
    }

    pub fn send_end_game_to_server(&mut self) {
        match &mut self.kind {
            Some(OpponentKind::Tcp(stream)) => {
                if let Err(e) = stream.write_all("ended".as_bytes()) {
                    eprintln!("Failed to send end game: {}", e);
                }
            }
            Some(OpponentKind::Lichess { .. }) => {
                // For Lichess, maybe we resign or abort?
                // For now, do nothing or implement resignation later.
            }
            None => {}
        }
    }

    pub fn send_move_to_server(&mut self, move_to_send: &Move, promotion_type: Option<Role>) {
        let from = self.convert_position_to_string(move_to_send.from());
        let to = self.convert_position_to_string(Some(move_to_send.to()));
        let move_str = format!(
            "{}{}{}",
            from,
            to,
            match promotion_type {
                Some(promotion) => match promotion {
                    Role::Queen => "q",
                    Role::Rook => "r",
                    Role::Bishop => "b",
                    Role::Knight => "n",
                    _ => "",
                },
                None => "",
            }
        );

        match &mut self.kind {
            Some(OpponentKind::Tcp(stream)) => {
                if let Err(e) = stream.write_all(move_str.as_bytes()) {
                    eprintln!("Failed to send move: {}", e);
                }
            }
            Some(OpponentKind::Lichess { move_tx, .. }) => {
                if let Err(e) = move_tx.send(move_str) {
                    eprintln!("Failed to send move to Lichess channel: {}", e);
                }
            }
            None => {}
        }
    }

    fn convert_position_to_string(&self, position: Option<Square>) -> String {
        position.map(|p| p.to_string()).unwrap_or_else(|| {
            log::warn!("Attempted to convert None position to string");
            String::new()
        })
    }

    pub fn read_stream(&mut self) -> Result<String, String> {
        match &mut self.kind {
            Some(OpponentKind::Tcp(stream)) => {
                let mut buffer = vec![0; NETWORK_BUFFER_SIZE];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            return Ok(String::new());
                        }
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        if response.trim() == "ended" || response.trim() == "" {
                            log::error!("Game ended by the other opponent");
                            return Err("Game ended by the other opponent".to_string());
                        }
                        Ok(response.to_string())
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Socket not ready, return empty string
                        Ok(String::new())
                    }
                    Err(e) => {
                        log::error!("Failed to read from stream: {}", e);
                        Ok(String::new())
                    }
                }
            }
            Some(OpponentKind::Lichess { move_rx, .. }) => {
                match move_rx.try_recv() {
                    Ok(m) => Ok(m),
                    Err(_) => Ok(String::new()), // Empty string means no move yet
                }
            }
            None => Ok(String::new()),
        }
    }
}

pub fn get_color_from_stream(mut stream: &TcpStream) -> Result<Color, String> {
    let mut buffer = [0; NETWORK_BUFFER_SIZE];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let color = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            match color.as_str() {
                "w" => Ok(Color::White),
                "b" => Ok(Color::Black),
                _ => Err("Failed to get color from stream".to_string()),
            }
        }
        Err(e) => Err(format!("Failed to read color from stream: {}", e)),
    }
}

pub fn wait_for_game_start(opponent: &mut Opponent) -> Result<bool, String> {
    match &mut opponent.kind {
        Some(OpponentKind::Tcp(stream)) => {
            let mut buffer = [0; NETWORK_BUFFER_SIZE];
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        return Ok(false);
                    }
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                    match response.as_str() {
                        "s" => Ok(true),
                        _ => Err("Failed to get start signal from stream".to_string()),
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Still waiting
                    Ok(false)
                }
                Err(e) => Err(format!("Failed to read start signal from stream: {}", e)),
            }
        }
        Some(OpponentKind::Lichess { .. }) => Ok(true), // Lichess game starts immediately
        None => Err("No opponent connected".to_string()),
    }
}
