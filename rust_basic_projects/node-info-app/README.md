# A simple app which reads information from the Bitcoin Blockchain

## Flow in steps:

1. What chain will be used?
2. Get info about a wallet (list of tx hashes by wallet)
3. Look up each tx hash using [NowNodes API](https://nownodes.io/)
4. Caclulate the wallet's current balance

## Libraries used:

- **tokio** - runtime library for writing asynchronous applications in Rust, providing the necessary tools to handle concurrent I/O operations and multithreading efficiently.
- **reqwest** - high-level, ergonomic HTTP client for Rust, designed to simplify making HTTP requests to APIs.
- **serde** - a framework for serializing and deserializing Rust data structures efficiently and generically.
