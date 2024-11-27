use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[derive(Clone)]
pub struct GameServer {
    pub clients: Arc<Mutex<Vec<TcpStream>>>,
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
            clients: Arc::new(Mutex::new(vec![])),
            client_id: 0,
            game_stream: Arc::new(Mutex::new(
                TcpListener::bind("127.0.0.1:2308").await.unwrap(),
            )),
            is_server_white,
        }
    }

    pub async fn run(&self) {
        log::info!("Server is running on 127.0.0.1:2308");

        let listener = self.game_stream.clone();
        let listener = listener.lock().await;

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    log::info!("New connection from {}", addr);

                    // Handle the connection in a separate task
                    tokio::spawn(Self::handle_connection(socket));
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn handle_connection(mut socket: TcpStream) {
        let (reader, mut writer) = socket.split();
        let mut buf_reader = BufReader::new(reader);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            match buf_reader.read_line(&mut buffer).await {
                Ok(0) => {
                    log::info!("Client disconnected");
                    break;
                }
                Ok(_) => {
                    log::info!("Received: {}", buffer.trim());
                    if let Err(e) = writer.write_all(b"Message received\n").await {
                        log::error!("Failed to write to socket: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    log::error!("Failed to read from socket: {}", e);
                    break;
                }
            }
        }
    }
}
