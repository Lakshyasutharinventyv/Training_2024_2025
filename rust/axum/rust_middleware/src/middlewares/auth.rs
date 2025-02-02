use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    let token = request.headers().get("authToken");

    match token {
        Some(r) => {
            if r == "1234" {
                next.run(request).await
            } else {
                "Unauthorized".into_response()
            }
        }
        None => "Unauthorized".into_response(),
    }
}
