use axum::body::Body;
use axum::{middleware::Next, http::Request, response::Response};
use tower_cookies::cookie::time::Time;
use std::fs::OpenOptions;
use std::io::Write;
use std::time;

pub async fn logger( req: Request<Body>,
    next: Next,) -> Response {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let currentTime =     format!("{}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
    // Open the file and append log entry

    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open("requests.log") {
        let _ = writeln!(file, "[{:?}] {} {}", currentTime, method, uri);
    }

    return next.run(req).await;
}
