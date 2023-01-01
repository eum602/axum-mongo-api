use axum::{Router, Server};

#[tokio::main]
async fn main() {
    let server_address = ([127, 0, 0, 1], 8080).into();
    println!("server_address: http://{:?}/", server_address);
    Server::bind(&server_address);
    let app = Router::new();
    Server::bind(&server_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
