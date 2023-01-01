# Guidelines

## doc commands

```sh
cargo doc --no-deps --open --document-private-items
cargo test
cargo add axum
cargo add dotenv
cargo add env_logger log
cargo add tracing tracing-subscriber    -> non blocking logging library
```

## Running

```sh
 RUST_LOG=info cargo run
```

## Routes

- "/"
- "/greetings"

## Notes

- gRPC -> Rust library -> Tonic
- Axum: Part of Tokio Project
- Copilot
