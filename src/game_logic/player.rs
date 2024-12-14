use crate::pieces::{PieceColor, PieceMove};
use std::{
    io::{Read, Write},
    net::TcpStream,
    panic,
};

pub struct Player {
    // The stream to communicate with the engine
    pub stream: Option<TcpStream>,
    /// Used to indicate if a player move is following
    pub player_will_move: bool,
    // The color of the player
    pub color: PieceColor,
    /// Is Game started
    pub game_started: bool,
}

// Custom Default implementation
impl Default for Player {
    fn default() -> Self {
        Player {
            stream: None,
            player_will_move: false,
            color: PieceColor::Black,
            game_started: false,
        }
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            stream: self.stream.as_ref().and_then(|s| s.try_clone().ok()), // Custom handling for TcpStream
            player_will_move: self.player_will_move,
            color: self.color,
            game_started: self.game_started,
        }
    }
}

impl Player {
    pub fn copy(&self) -> Self {
        Player {
            stream: None,
            player_will_move: self.player_will_move,
            color: self.color,
            game_started: self.game_started,
        }
    }

    pub fn new(addr: String, color: Option<PieceColor>) -> Player {
        // Attempt to connect to the provided address
        let stream = TcpStream::connect(addr).expect("Failed to connect to server");

        // Determine the player's color
        let color = match color {
            Some(color) => color, // Use the provided color if available
            None => get_color_from_stream(&stream),
        };

        let player_will_move = match color {
            PieceColor::White => true,
            PieceColor::Black => false,
        };

        Player {
            stream: Some(stream),
            player_will_move,
            color,
            game_started: false,
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
            let buf = game_stream.read(&mut buffer);
            match buf {
                Ok(bytes_read) => {
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                    if response.trim() == "ended" || response.trim() == "" {
                        panic!("Game ended by the other player");
                    }

                    response.to_string()
                }
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    "".to_string()
                }
            }
        } else {
            "".to_string()
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

pub fn wait_for_game_start(mut stream: &TcpStream) {
    let mut buffer = [0; 5];
    let bytes_read = stream.read(&mut buffer).unwrap(); // Number of bytes read
    let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    match response.as_str() {
        "s" => (),
        _ => panic!("Failed to get color from stream"),
    }
}
