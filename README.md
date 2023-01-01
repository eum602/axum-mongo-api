# Guidelines

## doc commands

```sh
cargo doc --no-deps --open --document-private-items
cargo test
cargo add axum
cargo add dotenv
cargo add env_logger log
cargo add tracing tracing-subscriber # non blocking logging library
cargo add -F  timeout tower &&  cargo add -F trace tower_http  # middlewares, activates default deactivated "timeout" for tower and "trace" for tower_http
```

## Running

```sh
 RUST_LOG="debug,tower_http=trace" cargo run
```

## Routes

- "/"
- "/greetings"

## Notes

- gRPC -> Rust library -> Tonic
- Axum: Part of Tokio Project
- Copilot
