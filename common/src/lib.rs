use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use bincode;

/// Structure for messages between clients and the server.
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Text(String),         // Normal text message
    Broadcast(String),    // Broadcast message to all clients
    File { filename: String, content: Vec<u8> },
    Image { filename: String, content: Vec<u8> },
    Quit,
}

impl Message {
    /// Sends a message over a TCP stream.
    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let encoded = bincode::serialize(self).map_err(|e| {
            eprintln!("Serialization error: {:?}", e);
            std::io::Error::new(std::io::ErrorKind::Other, "Serialization failed")
        })?;

        let size = (encoded.len() as u64).to_le_bytes();
        stream.write_all(&size)?;
        stream.write_all(&encoded)?;
        Ok(())
    }

    /// Receives a message from a TCP stream.
    pub fn receive(stream: &mut TcpStream) -> std::io::Result<Self> {
        let mut size_buf = [0u8; 8];
        stream.read_exact(&mut size_buf)?;

        let size = u64::from_le_bytes(size_buf) as usize;
        let mut buffer = vec![0; size];
        stream.read_exact(&mut buffer)?;

        bincode::deserialize(&buffer).map_err(|e| {
            eprintln!("Deserialization error: {:?}", e);
            std::io::Error::new(std::io::ErrorKind::Other, "Deserialization failed")
        })
    }
}