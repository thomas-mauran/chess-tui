use std::{io::{Read, Write}, net::TcpStream};
use crate::pieces::{PieceColor, PieceMove};

pub struct Player {
    // The stream to communicate with the engine
    pub stream: Option<TcpStream>,
    /// Used to indicate if a player move is following
    pub player_will_move: bool,
    // The color of the player
    pub color: PieceColor,
}

// Custom Default implementation
impl Default for Player {
    fn default() -> Self {
        Player {
            stream: None,
            player_will_move: false,
            color: PieceColor::Black,
        }
    }
}

impl Player {
    pub fn clone(self) -> Self {
        Player {
            stream: self.stream,
            player_will_move: self.player_will_move,
            color: self.color,
        }
    }

    pub fn new(addr: &str, color: Option<PieceColor>) -> Player {
        // Attempt to connect to the provided address
        let stream = TcpStream::connect(addr).expect("Failed to connect to server");

        // Determine the player's color
        let color = match color {
            Some(color) => color, // Use the provided color if available
            None => get_color_from_stream(&stream),
        };

        Player {
            stream: Some(stream),                 // Move the stream into the struct
            player_will_move: false, // Default to false
            color,
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



    pub fn send_move_to_server(&mut self, move_to_send: &PieceMove){
        if let Some(game_stream) = self.stream.as_mut() {
            let move_str = format!(
                "{}{}{}{}",
                move_to_send.from.row,
                move_to_send.from.col,
                move_to_send.to.row,
                move_to_send.to.col
            );
            if let Err(e) = game_stream.write_all(move_str.as_bytes()) {
                eprintln!("Failed to send move: {}", e);
            }

            if let Err(e) = game_stream.flush() {
                eprintln!("Failed to flush stream: {}", e);
            }
            // let _ = game_stream.flush().unwrap();
        }
    }

    pub fn read_stream(&mut self) -> String{
        if let Some(game_stream) = self.stream.as_mut() {
            let mut buffer = vec![0; 4];
            let buf = game_stream.read(&mut buffer);
            match buf {
                Ok(_) => {
                    let response = String::from_utf8_lossy(&buffer);
                    response.to_string()
                }
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    "".to_string()
                }
            }
                
        }else {
            "".to_string()
        }
    }
}


pub fn get_color_from_stream(mut stream: &TcpStream) -> PieceColor {
    let mut buffer = [0; 5];
    let bytes_read = stream.read(&mut buffer).unwrap(); // Number of bytes read
    let color = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    match color.as_str() {
        "white" => PieceColor::White,
        "black" => PieceColor::Black,
        _ => panic!("Failed to get color from stream"),
    }
}