use axum::{body::Body, extract::Request, middleware::Next, response::Response};



pub async fn logger(req: Request<Body>, next: Next) -> Response {
    println!("logger middleware");
    next.run(req).await
}