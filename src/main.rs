mod apis;
mod db;

use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;
use apis::*;
use db::establish_connection;

#[tokio::main]
async fn main() {
    let conn = establish_connection();
    //TODO: DB Code & Model & Schema
    
    let app = Router::new()
        .route("/healthcheck", get(healthcheck::healthcheck));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("-> Listening on {}", listener.local_addr().unwrap());
    serve(listener, app).await.unwrap();
}