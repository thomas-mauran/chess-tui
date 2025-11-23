use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
};

use crate::constants::{
    NETWORK_BUFFER_SIZE, NETWORK_PORT, SLEEP_DURATION_LONG_MS, SLEEP_DURATION_SHORT_MS,
};
use log;

#[derive(Debug)]
pub struct Client {
    addr: String,
    stream: TcpStream,
}

#[derive(Clone)]
pub struct GameServer {
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub client_id: usize,
    pub is_host_white: bool,
    pub stop_signal: Arc<AtomicBool>,
}

impl GameServer {
    pub fn new(is_host_white: bool) -> Self {
        Self {
            clients: Arc::new(Mutex::new(vec![])),
            client_id: 0,
            is_host_white,
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&self) {
        log::info!("Starting game server on 0.0.0.0:{}", NETWORK_PORT);
        let listener = TcpListener::bind(format!("0.0.0.0:{}", NETWORK_PORT))
            .expect("Failed to create listener");
        listener
            .set_nonblocking(true)
            .expect("Failed to set listener to non-blocking");

        let state = self.clients.clone();
        let stop_signal = self.stop_signal.clone();
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Spawn a thread to watch for the stop signal
        let stop_signal_clone = stop_signal.clone();
        thread::spawn(move || {
            while !stop_signal_clone.load(Ordering::SeqCst) {
                thread::sleep(std::time::Duration::from_millis(SLEEP_DURATION_LONG_MS));
            }
            let _ = shutdown_tx.send(());
        });

        loop {
            // Check for shutdown signal
            if shutdown_rx.try_recv().is_ok() {
                log::info!("Received shutdown signal, stopping server");
                break;
            }

            // Handle incoming connections
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    log::info!("New connection from: {}", addr);
                    let state = Arc::clone(&state);
                    let stop_signal = Arc::clone(&stop_signal);
                    let color = if self.is_host_white { "w" } else { "b" };

                    thread::spawn(move || {
                        {
                            let mut state_lock = state.lock().unwrap();
                            // There is already one player (host who choose the color) we will need to send the color to the joining player and inform the host of the game start
                            if state_lock.len() == 1 {
                                stream.write_all(color.as_bytes()).unwrap();
                                let other_player = state_lock.last().unwrap();
                                let mut other_player_stream =
                                    other_player.stream.try_clone().unwrap();
                                other_player_stream.write_all("s".as_bytes()).unwrap();
                            } else if state_lock.len() >= 2 {
                                stream.write_all("Game is already full".as_bytes()).unwrap();
                                return;
                            }

                            state_lock.push(Client {
                                addr: stream.peer_addr().unwrap().to_string(),
                                stream: stream.try_clone().unwrap(),
                            });
                        }
                        handle_client(state, stop_signal, stream);
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(std::time::Duration::from_millis(SLEEP_DURATION_LONG_MS));
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

fn handle_client(
    state: Arc<Mutex<Vec<Client>>>,
    stop_signal: Arc<AtomicBool>,
    mut stream: TcpStream,
) {
    let addr = stream.peer_addr().unwrap().to_string();
    log::info!("Starting client handler for: {}", addr);

    // Set socket to non-blocking mode
    if let Err(e) = stream.set_nonblocking(true) {
        log::error!("Failed to set non-blocking mode for client {}: {}", addr, e);
        return;
    }

    loop {
        let mut buffer = [0; NETWORK_BUFFER_SIZE];
        match stream.read(&mut buffer) {
            Ok(0) => {
                log::info!("Client {} disconnected", addr);
                broadcast_message(state.clone(), "ended".to_string(), &addr);
                remove_client(&state, &addr);
                stop_signal.store(true, Ordering::SeqCst);
                break;
            }
            Ok(bytes_read) => {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                log::debug!("Received message from {}: {}", addr, request.trim());
                broadcast_message(state.clone(), format!("{}", request), &addr);

                if request.trim() == "ended" {
                    log::info!("Client {} sent end signal", addr);
                    remove_client(&state, &addr);
                    stop_signal.store(true, Ordering::SeqCst);
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // This is normal for non-blocking sockets
                std::thread::sleep(std::time::Duration::from_millis(SLEEP_DURATION_SHORT_MS));
                continue;
            }
            Err(e) => {
                log::error!("Error reading from client {}: {}", addr, e);
                break;
            }
        }
    }
}

fn broadcast_message(state: Arc<Mutex<Vec<Client>>>, message: String, sender_addr: &String) {
    let state = state.lock().unwrap();
    for client in state.iter() {
        if &client.addr == sender_addr {
            continue;
        }
        let mut client_stream = client.stream.try_clone().unwrap();
        client_stream.write_all(message.as_bytes()).unwrap();
    }
}

fn remove_client(state: &Arc<Mutex<Vec<Client>>>, addr: &str) {
    let mut state_lock = state.lock().unwrap();
    if let Some(index) = state_lock.iter().position(|client| client.addr == addr) {
        state_lock.remove(index);
    }
}
