# Redis Clone (Rust)

A high-performance, asynchronous, in-memory key-value store inspired by Redis, built with Rust and the Tokio runtime. This project demonstrates core concepts of systems programming, including concurrent state management, TCP networking, and command parsing.

## 🚀 Features

* **Asynchronous Architecture**: Utilizes `tokio` to handle multiple concurrent client connections efficiently using non-blocking I/O.
* **Thread-Safe State**: Implements a shared database state using `Arc<RwLock<...>>`, allowing for parallel reads and synchronized writes across multiple threads.
* **Time-to-Live (TTL) Support**: Features an integrated expiration system that tracks keys using `Instant` for precise timing.
* **In-Memory Storage**: Optimized for speed by storing all data directly in RAM using `HashMap`.

## 🛠 Supported Commands

| Command | Description | Example |
|---------|-------------|---------|
| `SET` | Stores a string value associated with a key. | `SET user:1 rust_dev` |
| `SET ... EXP` | Stores a key-value pair with an expiration time in seconds. | `SET session:101 active EXP 30` |
| `GET` | Retrieves the value of a key. Returns null if expired or not found. | `GET user:1` |
| `DEL` | Removes the specified key and its associated expiration. | `DEL user:1` |

## 📦 Getting Started

### Prerequisites

* Rust (Edition 2024 or later)
* Cargo

### Installation

1. Clone the repository:

```bash
git clone <your-repository-url>
cd redis-clone
```

2. Build the project:

```bash
cargo build --release
```

### Running the Server

The server starts on `127.0.0.1:3002` by default.

```bash
cargo run
```

### Testing the Connection

You can interact with the server using `telnet` or `netcat`:

```bash
nc localhost 3002
# Then type: SET greeting hello
# Then type: GET greeting
```

## 🏗 Project Structure

* `src/main.rs`: Entry point that initializes the server.
* `src/server/`: Manages TCP listening and connection spawning.
* `src/database/`: Core logic for data storage, locking, and TTL management.
* `src/command/`: Logic for parsing raw input strings into executable commands.

## 📝 Roadmap

* [ ] **RESP Protocol**: Implement full Redis Serialization Protocol compatibility.
* [ ] **Active Expiration**: Add a background task to periodically clean up expired keys.
* [ ] **Persistence**: Support RDB snapshots or AOF logging for data durability.
* [ ] **Complex Types**: Add support for Lists, Hashes, and Sets.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](issues).

## 👨‍💻 Author

Your Name - [@Maze](https://github.com/yourusername)