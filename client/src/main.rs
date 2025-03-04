use std::io::{self, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use common::Message;

/// Sending messages to the server
fn send_message(stream: &mut TcpStream, message: &Message) {
    message.send(stream).expect("Failed to send message");
}

/// Loop for receiving messages (runs in a separate thread)
fn receive_loop(mut stream: TcpStream) {
    thread::spawn(move || {
        while let Ok(message) = Message::receive(&mut stream) {
            match message {
                Message::Text(text) => println!("> {}", text),
                Message::Broadcast(text) => println!("[Broadcast] {}", text),
                _ => {}
            }
        }
    });
}

fn main() {
    let address = "127.0.0.1:1111";
    let stream = TcpStream::connect(&address).expect("Cannot connect to server");
    println!("Connected to {}", address);

    let send_stream = stream.try_clone().expect("Failed to clone stream");
    let recv_stream = stream;

    receive_loop(recv_stream);

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    for line in reader.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();

        if trimmed == ".quit" {
            send_message(&mut send_stream.try_clone().unwrap(), &Message::Quit);
            break;
        } else {
            send_message(&mut send_stream.try_clone().unwrap(), &Message::Text(line));
        }
    }
}