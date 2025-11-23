use crate::constants::NETWORK_BUFFER_SIZE;
use log;
use shakmaty::{Color, Move, Role, Square};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct Opponent {
    // The stream to communicate with the engine
    pub stream: Option<TcpStream>,
    /// Used to indicate if a Opponent move is following
    pub opponent_will_move: bool,
    // The color of the Opponent
    pub color: Color,
    /// Is Game started
    pub game_started: bool,
}

// Custom Default implementation
impl Default for Opponent {
    fn default() -> Self {
        Opponent {
            stream: None,
            opponent_will_move: false,
            color: Color::Black,
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

            Ok(Opponent {
                stream: Some(stream),
                opponent_will_move,
                color,
                game_started: false,
            })
        } else {
            log::error!("Failed to connect after 5 attempts to {}", addr);
            Err(format!(
                "Failed to connect to the server after 5 attempts to the following address {}",
                addr
            ))
        }
    }

    pub fn start_stream(&mut self, addr: &str) -> Result<(), String> {
        match TcpStream::connect(addr) {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(format!("Failed to connect: {}", e)),
        }
    }

    pub fn send_end_game_to_server(&mut self) {
        if let Some(game_stream) = self.stream.as_mut() {
            if let Err(e) = game_stream.write_all("ended".as_bytes()) {
                eprintln!("Failed to send end game: {}", e);
            }
        }
    }

    pub fn send_move_to_server(&mut self, move_to_send: &Move, promotion_type: Option<Role>) {
        let from = self.convert_position_to_string(move_to_send.from());
        let to = self.convert_position_to_string(Some(move_to_send.to()));

        if let Some(game_stream) = self.stream.as_mut() {
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
            if let Err(e) = game_stream.write_all(move_str.as_bytes()) {
                eprintln!("Failed to send move: {}", e);
            }
        }
    }
    // 192.168.1.28:2308

    fn convert_position_to_string(&self, position: Option<Square>) -> String {
        position.map(|p| p.to_string()).unwrap_or_else(|| {
            log::warn!("Attempted to convert None position to string");
            String::new()
        })
    }

    pub fn read_stream(&mut self) -> Result<String, String> {
        if let Some(game_stream) = self.stream.as_mut() {
            let mut buffer = vec![0; NETWORK_BUFFER_SIZE];
            match game_stream.read(&mut buffer) {
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
                    // This is expected for non-blocking sockets
                    log::debug!("Socket not ready, would block");
                    Ok(String::new())
                }
                Err(e) => {
                    log::error!("Failed to read from stream: {}", e);
                    Ok(String::new())
                }
            }
        } else {
            Ok(String::new())
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

pub fn wait_for_game_start(mut stream: &TcpStream) -> Result<(), String> {
    let mut buffer = [0; NETWORK_BUFFER_SIZE];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            match response.as_str() {
                "s" => Ok(()),
                _ => Err("Failed to get start signal from stream".to_string()),
            }
        }
        Err(e) => Err(format!("Failed to read start signal from stream: {}", e)),
    }
}
