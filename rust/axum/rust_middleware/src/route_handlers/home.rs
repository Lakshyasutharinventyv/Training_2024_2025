use axum::response::IntoResponse;

pub async fn home_route_handler() -> impl IntoResponse {
    "hello World"
}
