use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};


#[derive(Debug)]
pub struct Client{
    addr: String,
    stream: TcpStream,
}


#[derive(Clone)]
pub struct GameServer {
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub client_id: usize,
    /// Defines the color of the server side
    pub is_host_white: bool,
}

impl GameServer {
    pub fn new(is_host_white: bool) -> Self {
        Self {
            clients: Arc::new(Mutex::new(vec![])),
            client_id: 0,
            is_host_white,
        }
    }

    pub fn run(&self) {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:2308").expect("Failed to create listener");

        let state = self.clients.clone();

        for stream in listener.incoming(){
            match stream{
                Ok(mut stream) => {
                    let state = Arc::clone(&state);
                    // We send the other player color, so that if the host is white the other player is black
                    let color = if self.is_host_white {
                        "w"
                    }else {
                        "b"
                    };
                    std::thread::spawn(move || {
                        {
                            let mut state_lock: std::sync::MutexGuard<'_, Vec<Client>> = state.lock().unwrap();

                            // We already have the host in the game we will send the color of the other player
                            if state_lock.len() == 1 {
                                stream.write_all(color.as_bytes());

                                // We tell the other player that the game started
                                let other_player = state_lock.last().unwrap();
                                let mut other_player_stream = other_player.stream.try_clone().unwrap();
                                other_player_stream.write_all("s".as_bytes());

                            }
                            else if state_lock.len() >= 2 {
                                stream.write_all("Game is already full".as_bytes()).expect("Failed to write to client!");
                                // Close the stream as we won't handle it
                                drop(stream);
                                return;
                            }

                            
                            state_lock.push(Client{
                                addr: stream.peer_addr().unwrap().to_string(),
                                stream: stream.try_clone().unwrap(),
                            });
                        }
                        handle_client(state, stream);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to establish connection: {}", e);
                }
            }
        }
    }

    
}

    
fn handle_client(state: Arc<Mutex<Vec<Client>>> , mut stream: TcpStream){
    loop {
        let mut buffer = [0; 5];

        let addr = stream.peer_addr().unwrap().to_string();

        let bytes_read = stream.read(&mut buffer).expect("Failed to read from client!");

        // If the client closed the connection, break the loop
        if bytes_read == 0 {
            println!("Client {} disconnected", addr);
            remove_client(&state, &addr);
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..]); 
        
        broadcast_message(state.clone(), format!("{}", request.to_string()), &addr);

        // Check if the request is "end"
        if request.trim() == "ended" {
            // Notify other clients about disconnection
            for client in state.lock().unwrap().iter(){
                remove_client(&state, &client.addr);
            }
            break;
        }
    }
}


pub fn broadcast_message(state: Arc<Mutex<Vec<Client>>>, message: String, sender_addr: &String){

    let state = state.lock().unwrap();

    for client in state.iter(){
        if &client.addr == sender_addr {
            continue;
        }
        let mut client_stream = client.stream.try_clone().unwrap();
        client_stream.write_all(message.as_bytes()).expect("Failed to write to client!");
    }
}

fn remove_client(state: &Arc<Mutex<Vec<Client>>>, addr: &str) {
    let mut state_lock = state.lock().unwrap();
    if let Some(index) = state_lock.iter().position(|client| client.addr == addr) {
        state_lock.remove(index);
        println!("Removed client {} from the state", addr);
    }
}