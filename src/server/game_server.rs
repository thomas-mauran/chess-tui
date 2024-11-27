use std::sync::Arc;

use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream}, sync::Mutex};


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
    pub async fn run(&mut self) {
        log::info!("Server is running on 127.0.0.1:2308");

        let listener = self.game_stream.clone();
        let listener = listener.lock().await;

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    log::info!("New connection from {}", addr);

                    // Notify all clients about the new connection
                    let message = format!("New client connected: {}\n", addr);
                    self.broadcast_message(&message).await;

                    // Add the new client to the list
                    self.clients.lock().await.push(socket);

                    // Handle the connection in a separate task
                    tokio::spawn(Self::handle_connection(self.clients.clone(), socket));
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn broadcast_message(&self, message: &str) {
        let clients = self.clients.lock().await;

        // Iterate through all clients and send the message
        for client in clients.iter() {
            if let Err(e) = Self::send_message(client, message).await {
                log::error!("Failed to send message to a client: {}", e);
            }
        }
    }

    async fn send_message(client: &mut TcpStream, message: &str) -> Result<(), tokio::io::Error> {
        client.write_all(message.as_bytes()).await?;
        Ok(())
    }

    async fn handle_connection(clients: Arc<Mutex<Vec<TcpStream>>>, mut socket: TcpStream) {
        let (reader, mut writer) = socket.split();
        let mut buf_reader = BufReader::new(reader);
        let mut buffer = String::new();
        loop {
            buffer.clear();

            let clients = clients.lock().await;
            match buf_reader.read_line(&mut buffer).await {
                Ok(0) => {
                    log::info!("Client disconnected");
                    break;
                }
                Ok(_) => {
                    log::info!("Received: {}", buffer.trim());

                    if let Err(e) = writer.write_all("hello".as_bytes()).await {
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
