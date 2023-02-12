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

## Before Running

- start a mongodb instance

```sh
cd mongodb
docker compose up
```

- Create a database with name "examplemongo-ms"
- Create a collection with name "orders"

## Routes

- "/"
- "/greetings"

- Creating a new Order:

```sh
curl -iX POST "http://127.0.0.1:8080/orders"
```

Response:

```sh
{"id":"7abe5565-cb35-474a-bccf-6170f562e1a3","user_id":"a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8","items":[]}
```

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
