use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[derive(Clone)]
pub struct GameServer {
    pub clients: Vec<String>,
    pub client_id: usize,
    /// The stream is in ARC (share the ownership between threads) and Mutex (handle the ownership between threads)
    pub game_stream: Arc<Mutex<TcpListener>>,
    /// Defines the color of the server side
    pub is_server_white: bool,
}

impl GameServer {
    pub async fn new(is_server_white: bool) -> Self {
        log::info!("Creating new server");
        Self {
            clients: vec![],
            client_id: 0,
            game_stream: Arc::new(Mutex::new(
                TcpListener::bind("127.0.0.1:2308").await.unwrap(),
            )),
            is_server_white,
        }
    }
}
