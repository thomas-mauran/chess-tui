use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};


pub struct Client{
    addr: String,
    stream: TcpStream,
}


#[derive(Clone)]
pub struct GameServer {
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub client_id: usize,
    /// Defines the color of the server side
    pub is_server_white: bool,
}

impl GameServer {
    pub fn new(is_server_white: bool) -> Self {
        log::info!("Creating new server");
        Self {
            clients: Arc::new(Mutex::new(vec![])),
            client_id: 0,
            is_server_white,
        }
    }

    pub fn run(&self) {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:2308").expect("Failed to create listener");

        log::info!("Server is running on 127.0.0.1:2308");
        
        let state = self.clients.clone();

        for stream in listener.incoming(){
            match stream{
                Ok(stream) => {
                    let state = Arc::clone(&state);
                    std::thread::spawn(move || {
                        {
                            let mut state_lock: std::sync::MutexGuard<'_, Vec<Client>> = state.lock().unwrap();
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
        let mut buffer = [0; 1024];

        let addr = stream.peer_addr().unwrap().to_string();

        stream.read(&mut buffer).expect("Faiiled to read from client!");

        let request = String::from_utf8_lossy(&buffer[..]);

        broadcast_message(state.clone(), format!("{}: {}\n",addr, request.to_string()), addr);
        println!("Received {}", request);
    }
}


pub fn broadcast_message(state: Arc<Mutex<Vec<Client>>>, message: String, sender_addr: String){
    let state = state.lock().unwrap();

    for client in state.iter(){
        if client.addr == sender_addr {
            continue;
        }
        let mut client_stream = client.stream.try_clone().unwrap();
        client_stream.write(message.as_bytes()).expect("Failed to write to client!");
    }
}
