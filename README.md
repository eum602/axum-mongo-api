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
cargo add -F derive serde && cargo add serde_json
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

- Add item to order:

```sh
curl -iX POST -H "Content-Type: application/json" -d "{\"product_id\": \"e90d2ec4-89ed-11ed-a1eb-0242ac120002\", \"quantity\": 24}" "http://127.0.0.1:8080/orders/362e4ec4-89ed-11ed-a1eb-0242ac121235/items"
```

- ```sh
  curl -iX DELETE "http://127.0.0.1:8080/orders/e90d2ec4-89ed-11ed-a1eb-0242ac120002/items/1"
  ```

## Notes

- gRPC -> Rust library -> Tonic
- Axum: Part of Tokio Project
- Copilot
