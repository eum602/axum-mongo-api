use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let server_address = ([127, 0, 0, 1], 8080).into();
    println!("server_address: http://{:?}/", server_address);
    Server::bind(&server_address);
    let app = Router::new()
        .route("/", get(|| async { "Welcome to main page" }))
        .route("/greetings", get(greet));
    Server::bind(&server_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet() -> &'static str {
    "Hello world!"
}
