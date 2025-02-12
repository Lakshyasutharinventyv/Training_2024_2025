mod handlers;
mod router;
mod middlewares;
mod database;
mod models;

use axum::Router;
use router::routes::router;
use tokio::net::TcpListener;
use database::connection::connect_to_tidb;
#[tokio::main]
async fn main() {
    connect_to_tidb().await;
    let app = Router::new().merge(router(connect_to_tidb().await)); 
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
