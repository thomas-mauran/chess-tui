use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    vec,
};

#[derive(Clone)]
pub struct GameServer {
    pub clients: Vec<String>,
    pub client_id: usize,
    /// The stream is in ARC (share the ownership between threads) and Mutex (handle the ownership between threads)
    pub game_stream: Arc<Mutex<TcpStream>>,
    /// Defines the color of the server side
    pub is_server_white: bool,
}

impl GameServer {
    pub fn new(is_server_white: bool) -> Self {
        log::info!("Creating new server");
        Self {
            clients: vec![],
            client_id: 0,
            game_stream: Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:2308").unwrap())),
            is_server_white,
        }
    }
}
