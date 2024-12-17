# A toy HTTP 1.1 server built in Rust to learn

## Goals
The main objectives of this project are to learn, specifically:
 - Rust
 - HTTP (in depth)
 - How to read and implement specs generally

## Functionality still to implement
At the moment it is very limited with the following functionality as the immediate items on the TODO list:
 - Headers
 - More HTTP methods
 - Query parameters
 - A "builder" style for generating responses

Further down the road it may be interesting to look into:
 - HTTP over TLS
 - Spec compliant caching
 - Persistent connections
 - Other more complex portions of the HTTP 1.1 spec

## How to run
- Download the project
- Ensure that you have Rust installed
- Run `cargo run` from the root directory (i.e. same directory as Cargo.toml)
- Navigate to localhost:8080 in your browser (or other HTTP client)