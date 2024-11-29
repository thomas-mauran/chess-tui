
use futures::TryStreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};
use futures::sink::SinkExt;

use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
/// Shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<String>;

/// Shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<String>;

/// `Tx`.
struct Shared {
    pub peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
    /// Create a new, empty, instance of `Shared`.
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    /// Send a `LineCodec` encoded message to every peer, except
    /// for the sender.
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        for peer in self.peers.iter_mut() {
            if *peer.0 != sender {
                let _ = peer.1.send(message.into());
            }
        }
    }
}
/// The state for each connected client.
struct Peer {
    /// The TCP socket wrapped with the `Lines` codec, defined below.
    ///
    /// This handles sending and receiving data on the socket. When using
    /// `Lines`, we can work at the line level instead of having to manage the
    /// raw byte operations.
    lines: Framed<TcpStream, LinesCodec>,

    /// Receive half of the message channel.
    ///
    /// This is used to receive messages from peers. When a message is received
    /// off of this `Rx`, it will be written to the socket.
    rx: Rx,
}

impl Peer {
    /// Create a new instance of `Peer`.
    async fn new(
        state: Arc<Mutex<Shared>>,
        lines: Framed<TcpStream, LinesCodec>,
    ) -> io::Result<Peer> {
        // Get the client socket address
        let addr = lines.get_ref().peer_addr()?;

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded_channel();

        // Add an entry for this `Peer` in the shared state map.
        state.lock().await.peers.insert(addr, tx);

        Ok(Peer { lines, rx })
    }
}

#[derive(Clone)]
pub struct GameServer {
    pub clients: Arc<Mutex<Shared>>,
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
            clients: Arc::new(Mutex::new(Shared {
                peers: HashMap::new(),
            })),
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
                Ok((stream, addr)) => {
                    log::info!("New connection from {}", addr);
                    let state = Arc::clone(&self.clients);


                    // Handle the connection in a separate task
                    tokio::spawn(Self::handle_connection(state, stream, addr));
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn handle_connection(state: Arc<Mutex<Shared>>, mut stream: TcpStream, addr: SocketAddr) {
        let mut lines = Framed::new(stream, LinesCodec::new());

        // Register our peer with state which internally sets up some channels.
        let mut peer = Peer::new(state.clone(), lines).await.unwrap();

        {
            let mut state = state.lock().await;
            let msg = format!("Hello has joined the chat");
            state.broadcast(addr, &msg).await;
        }

        loop {
            tokio::select! {
                // A message was received from a peer. Send it to the current user.
                Some(msg) = peer.rx.recv() => {
                    peer.lines.send(&msg).await.unwrap();
                }
                result = peer.lines.try_next() => match result {
                    // A message was received from the current user, we should
                    // broadcast this message to the other users.
                    Ok(msg) => {
                        let mut state = state.lock().await;
                        let msg = format!("Hello: {addr}");
    
                        state.broadcast(addr, &msg).await;
                    }
                    // An error occurred.
                    Err(e) => {
                        println!("an error occurred; error = {:?}", e);
                    }
                },
            }
        }
    }
}
