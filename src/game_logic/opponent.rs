use crate::pieces::{PieceColor, PieceMove};
use log;
use std::{
    io::{Read, Write},
    net::TcpStream,
    panic,
};

pub struct Opponent {
    // The stream to communicate with the engine
    pub stream: Option<TcpStream>,
    /// Used to indicate if a Opponent move is following
    pub opponent_will_move: bool,
    // The color of the Opponent
    pub color: PieceColor,
    /// Is Game started
    pub game_started: bool,
}

// Custom Default implementation
impl Default for Opponent {
    fn default() -> Self {
        Opponent {
            stream: None,
            opponent_will_move: false,
            color: PieceColor::Black,
            game_started: false,
        }
    }
}

impl Clone for Opponent {
    fn clone(&self) -> Self {
        Opponent {
            stream: self.stream.as_ref().and_then(|s| s.try_clone().ok()), // Custom handling for TcpStream
            opponent_will_move: self.opponent_will_move,
            color: self.color,
            game_started: self.game_started,
        }
    }
}

impl Opponent {
    pub fn copy(&self) -> Self {
        Opponent {
            stream: None,
            opponent_will_move: self.opponent_will_move,
            color: self.color,
            game_started: self.game_started,
        }
    }

    pub fn new(addr: String, color: Option<PieceColor>) -> Opponent {
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
                    get_color_from_stream(&stream)
                }
            };

            let opponent_will_move = match color {
                PieceColor::White => true,
                PieceColor::Black => false,
            };
            log::info!(
                "Created opponent with color {:?}, will_move: {}",
                color,
                opponent_will_move
            );

            Opponent {
                stream: Some(stream),
                opponent_will_move,
                color,
                game_started: false,
            }
        } else {
            log::error!("Failed to connect after 5 attempts to {}", addr);
            panic!(
                "Failed to connect to the server after 5 attempts to the following address {}",
                addr
            );
        }
    }

    pub fn start_stream(&mut self, addr: &str) {
        match TcpStream::connect(addr) {
            Ok(stream) => {
                self.stream = Some(stream);
            }
            Err(e) => {
                panic!("Failed to connect: {}", e);
            }
        }
    }

    pub fn send_end_game_to_server(&mut self) {
        if let Some(game_stream) = self.stream.as_mut() {
            if let Err(e) = game_stream.write_all("ended".as_bytes()) {
                eprintln!("Failed to send end game: {}", e);
            }
        }
    }

    pub fn send_move_to_server(
        &mut self,
        move_to_send: &PieceMove,
        promotion_type: Option<String>,
    ) {
        if let Some(game_stream) = self.stream.as_mut() {
            let move_str = format!(
                "{}{}{}{}{}",
                move_to_send.from.row,
                move_to_send.from.col,
                move_to_send.to.row,
                move_to_send.to.col,
                match promotion_type {
                    Some(promotion) => promotion,
                    None => "".to_string(),
                }
            );
            if let Err(e) = game_stream.write_all(move_str.as_bytes()) {
                eprintln!("Failed to send move: {}", e);
            }
        }
    }

    pub fn read_stream(&mut self) -> String {
        if let Some(game_stream) = self.stream.as_mut() {
            let mut buffer = vec![0; 5];
            match game_stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        return String::new();
                    }
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                    if response.trim() == "ended" || response.trim() == "" {
                        log::error!("Game ended by the other opponent");
                        panic!("Game ended by the other opponent");
                    }
                    response.to_string()
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // This is expected for non-blocking sockets
                    log::debug!("Socket not ready, would block");
                    String::new()
                }
                Err(e) => {
                    log::error!("Failed to read from stream: {}", e);
                    String::new()
                }
            }
        } else {
            String::new()
        }
    }
}

pub fn get_color_from_stream(mut stream: &TcpStream) -> PieceColor {
    let mut buffer = [0; 5];
    let bytes_read = stream.read(&mut buffer).unwrap(); // Number of bytes read
    let color = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    match color.as_str() {
        "w" => PieceColor::White,
        "b" => PieceColor::Black,
        _ => panic!("Failed to get color from stream"),
    }
}

pub fn wait_for_game_start(mut stream: &TcpStream) -> Result<(), &str>{
    let mut buffer = [0; 5];
    let bytes_read = stream.read(&mut buffer).unwrap(); // Number of bytes read
    let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    match response.as_str() {
        "s" => Ok(()),
        _   => Err("Failed to get color from stream"),
    }
}
