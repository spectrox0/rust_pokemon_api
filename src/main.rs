
use tokio::net::TcpListener;
use std::net::SocketAddr;
mod db;
use db::setup::setup_db;
mod controllers;
mod routes;
mod services;
mod models;
use routes::create_routes;
use tracing_subscriber;
#[tokio::main]
async fn main() {
    let _db_connection = setup_db();
    // Check if the database connection is working

    tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();

    let app = create_routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    let server = axum::serve(listener, app);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    };
}
