mod handlers;
mod router;
mod middlewares;

use axum::Router;
use router::routes::router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(router()); 
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
