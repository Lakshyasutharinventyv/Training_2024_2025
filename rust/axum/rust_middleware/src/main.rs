mod route_handlers;
mod middlewares;

use axum::{
    routing::{get, post},
    Router,
    middleware
};
use route_handlers::{home::home_route_handler, login::login};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use middlewares::auth::auth_middleware;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_route_handler).layer(middleware::from_fn(auth_middleware)))
        .route("/login", post(login));

   
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
