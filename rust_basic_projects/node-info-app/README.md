# A console app, built on top of a rest api for reading node and wallet data from Bitcoin main chain

## Topics learned:

- refactoring and SoC
- Asynchronous programming, concurrency and multithreading in Rust.
- Building and sending HTTP requests to an API using reqwest HTTP client.
- Serializing and Deserializing structs in Rust using serde framework.

## Flow in steps:

1. What chain will be used?
2. Get info about a wallet (list of tx hashes by wallet)
3. Look up each tx hash using [NowNodes API](https://nownodes.io/)
4. Caclulate the wallet's current balance

## Libraries used:

- **tokio** - runtime library for writing asynchronous applications in Rust, providing the necessary tools to handle concurrent I/O operations and multithreading efficiently.
- **reqwest** - high-level, ergonomic HTTP client for Rust, designed to simplify making HTTP requests to APIs.
- **serde** - a framework for serializing and deserializing Rust data structures efficiently and generically.
