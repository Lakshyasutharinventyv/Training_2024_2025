use axum::body::Body;
use axum::{middleware::Next, http::Request, response::Response};
use std::fs::OpenOptions;
use std::io::Write;


pub async fn logger( req: Request<Body>,
    next: Next,) -> Response {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let current_time =     format!("{}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));

    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("requests.log") {
        let _ = writeln!(file, "[{:?}] {} {}", current_time, method, uri);
    }

    return next.run(req).await;
}
