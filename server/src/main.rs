use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use common::Message;
use log::{info, error};
use ctrlc;

type ClientList = Arc<Mutex<HashMap<usize, TcpStream>>>;

fn handle_client(mut stream: TcpStream, id: usize, clients: ClientList) {
    loop {
        match Message::receive(&mut stream) {
            Ok(Message::Text(text)) => {
                info!("Client {} sent: {}", id, text);
                broadcast_message(&clients, id, Message::Broadcast(format!("Client {}: {}", id, text)));
            }
            Ok(Message::Quit) => {
                info!("Client {} disconnected.", id);
                clients.lock().unwrap().remove(&id);
                let _ = stream.shutdown(std::net::Shutdown::Both);
                break;
            }
            Err(_) => {
                info!("Client {} disconnected unexpectedly.", id);
                clients.lock().unwrap().remove(&id);
                break;
            }
            _ => {}
        }
    }
}

fn broadcast_message(clients: &ClientList, sender_id: usize, message: Message) {
    let clients = clients.lock().unwrap();
    for (&id, client) in clients.iter() {
        if id != sender_id {
            let mut client = client.try_clone().expect("Failed to clone stream");
            let _ = message.send(&mut client);
        }
    }
}

fn main() {
    env_logger::init();
    let address = "0.0.0.0:1111";
    let listener = TcpListener::bind(&address).expect("Cannot start server");
    info!("Server listening on {}", address);

    let clients: ClientList = Arc::new(Mutex::new(HashMap::new()));
    let running = Arc::new(AtomicBool::new(true));

    // Setting up the handler for Ctrl+C
    let running_clone = Arc::clone(&running);
    let clients_clone = Arc::clone(&clients);
    ctrlc::set_handler(move || {
        println!("\nShutting down server...");
        running_clone.store(false, Ordering::SeqCst);

        // Disconnect all clients
        let mut clients = clients_clone.lock().unwrap();
        for (_, client) in clients.drain() {
            let _ = client.shutdown(std::net::Shutdown::Both);
        }

        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    let mut client_id = 0;

    for stream in listener.incoming() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                let id = client_id;
                clients.lock().unwrap().insert(id, stream.try_clone().unwrap());

                thread::spawn(move || handle_client(stream, id, clients));
                client_id += 1;
            }
            Err(e) => error!("Connection failed: {}", e),
        }
    }

    println!("Server shutdown complete.");
}

