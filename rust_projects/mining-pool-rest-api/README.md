# A fully-functional REST API for a crypto-mining pool

## Topics learned:

- REST API with Rust

## Application Flow:

![alt text](scheme.png)

## Libraries used:

- **actix** - a powerful actor-based framework for building asynchronous applications and web servers in Rust.
- **serde** - a framework for serializing and deserializing Rust data structures efficiently and generically.
- **env_logger** - a logging implementation that reads log configuration from environment variables.
- **diesel** → a type-safe, ORM-like query builder for interacting with SQL databases in Rust.
- **r2d2** → a generic connection pool library for managing database (or other resource) connections.
- **r2d2-diesel** → a Diesel-specific adapter for using r2d2 as a database connection pool.
- **rand** → a library for random number generation, including secure and customizable RNGs.
- **uuid** → a library for generating and working with UUIDs, with features like v4 random UUIDs and Serde support.
