mod apis;
mod db;

use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;
use futures::executor::block_on;
use apis::*;
use db::create_dbconnection;

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(create_dbconnection()) {
        panic!("{}", err);
    }
    
    let app = Router::new()
        .route("/healthcheck", get(healthcheck::healthcheck));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("-> Listening on {}", listener.local_addr().unwrap());
    serve(listener, app).await.unwrap();
}