use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex, atomic::{AtomicBool, Ordering}},
    thread,
};

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
        let listener = TcpListener::bind("127.0.0.1:2308").expect("Failed to create listener");
        listener.set_nonblocking(true).expect("Failed to set listener to non-blocking");

        let state = self.clients.clone();
        let stop_signal = self.stop_signal.clone();
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Spawn a thread to watch for the stop signal
        let stop_signal_clone = stop_signal.clone();
        thread::spawn(move || {
            while !stop_signal_clone.load(Ordering::SeqCst) {
                thread::sleep(std::time::Duration::from_millis(100));
            }
            let _ = shutdown_tx.send(());
        });

        loop {
            // Check for shutdown signal
            if let Ok(_) = shutdown_rx.try_recv() {
                println!("Shutting down server...");
                break;
            }

            // Handle incoming connections
            match listener.accept() {
                Ok((mut stream, _addr)) => {
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
                                let mut other_player_stream = other_player.stream.try_clone().unwrap();
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
                    // No connection ready, sleep briefly
                    thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

fn handle_client(state: Arc<Mutex<Vec<Client>>>, stop_signal: Arc<AtomicBool>, mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 5];
        let addr = stream.peer_addr().unwrap().to_string();
        let bytes_read = stream.read(&mut buffer).unwrap_or(0);

        if bytes_read == 0 {
            println!("Client {} disconnected", addr);
            remove_client(&state, &addr);
            // we stop the server if one of the clients disconnects
            stop_signal.store(true, Ordering::SeqCst);
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..]);
        broadcast_message(state.clone(), format!("{}", request), &addr);

        if request.trim() == "ended" {
            remove_client(&state, &addr);
            // We stop the server if one of the clients disconnects
            stop_signal.store(true, Ordering::SeqCst);
            break;
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
