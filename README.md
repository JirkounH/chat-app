# Rust TCP Chat Application

📡 A synchronous TCP-based chat application supporting text communication and file transfer between clients via a central server.

## 📁 Project Structure

```plaintext
chat-app/
│── client/         # Client
│   ├── src/
│   │   ├── main.rs
│   ├── Cargo.toml
│── server/         # Server
│   ├── src/
│   │   ├── main.rs
│   ├── Cargo.toml
│── common/         # Shared Library
│   ├── src/
│   │   ├── lib.rs
│   ├── Cargo.toml
│── Cargo.toml       # Root workspace
│── README.md        # Documentation
```

## 🛠 Requirements

- Rust **1.70+**
- Cargo (Rust package manager)
- Linux/macOS/Windows

## 🚀 Installation and Running

First, build the project:

```sh
cargo build
```

Then, start the server in one terminal:

```sh
cargo run -p server
```

And run the client in another terminal:

```sh
cargo run -p client
```

## 💬 Using the Client

| Command                    | Description         |
| -------------------------- | ------------------- |
| `Hello, world!`            | Send a text message |
| `.file path/to/file.txt`   | Send a file         |
| `.image path/to/image.png` | Send an image       |
| `.quit`                    | Disconnect          |

## 🔧 How It Works

1. **Server** listens for incoming connections on `0.0.0.0:1111`.
2. **Clients** connect and can send messages to each other.
3. Messages are serialized using `bincode` and transmitted via **TCP**.
4. Files and images are stored in designated directories.
5. Clients can disconnect using the `.quit` command.

## 📜 Sample Output

```plaintext
[Server] Listening on 0.0.0.0:1111
[Client] Connected to 127.0.0.1:1111
[Client] Sending message: "Hello, world!"
[Server] Received: "Hello, world!"
```

## 🛠 Technologies Used

- **Rust** – for system-level performance
- **Serde** – for serialization and deserialization of data
- **Bincode** – efficient binary data transmission
- **Log** – structured event logging
- **CtrlC** – for handling graceful shutdown
- **Arc/Mutex** – for safe concurrency with shared state

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request with your changes.

## 📜 License

This project is distributed under the **MIT License**.
