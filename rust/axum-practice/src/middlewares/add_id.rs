use axum::{body::Body, extract::Request, middleware::Next, response::Response};



pub async fn add_id(req: Request<Body>, next: Next) -> Response {
    println!("Adding ID to request");
    next.run(req).await
}