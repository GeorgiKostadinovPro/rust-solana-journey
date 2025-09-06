# A console app, built on top of a rest api for reading node and wallet data from Bitcoin main chain

## Topics learned:

- refactoring and SoC.
- Asynchronous programming, concurrency and multithreading in Rust.
- Building and sending HTTP requests to an API using reqwest HTTP client.
- Serializing and Deserializing structs in Rust using serde framework.

## Flow in steps:

1. Choose an address from bitcoin chain and add it to the .env
2. Get wallet tx count and query all tx
3. Choose a specific tx hash
4. Look up each tx hash using [NowNodes API](https://nownodes.io/)

## Libraries used:

- **tokio** - runtime library for writing asynchronous applications in Rust, providing the necessary tools to handle concurrent I/O operations and multithreading efficiently.
- **reqwest** - high-level, ergonomic HTTP client for Rust, designed to simplify making HTTP requests to APIs.
- **serde** - a framework for serializing and deserializing Rust data structures efficiently and generically.
