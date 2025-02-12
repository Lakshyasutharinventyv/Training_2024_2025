use axum::{body::Body, extract::Request, http::HeaderValue, middleware::Next, response::Response};



pub async fn add_id(req: Request<Body>, next: Next) -> Response {
    
    let id = req.uri().path().split('/').last().unwrap_or("").to_string();
    let (mut parts, body) = req.into_parts();
    
    if let Ok(header_value) = HeaderValue::from_str(&id) {
        parts.headers.insert("X-Animal-ID", header_value);
    }
    let new_req = Request::from_parts(parts, body);
    next.run(new_req).await
}
