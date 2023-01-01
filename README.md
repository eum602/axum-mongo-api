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
cargo add -F serde uuid #enable serde (serializable-deserializable) feature on installing uuid crate
```

## Running

```sh
 RUST_LOG="debug,tower_http=trace" cargo run
```

## Health Check

```sh
curl -iX GET "http://127.0.0.1:8080/health" # -i to show headers
```

## Routes

- "/"
- "/greetings"

## Notes

- gRPC -> Rust library -> Tonic
- Axum: Part of Tokio Project
- Copilot
