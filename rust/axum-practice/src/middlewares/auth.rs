use axum::{body::Body, extract::Request, middleware::Next, response::Response};



pub async fn auth(req: Request<Body>, next: Next) -> Response {
    println!("auth middleware");
    next.run(req).await
}