use axum::response::IntoResponse;

pub async fn login_handler() -> impl IntoResponse {
    "Login Handler"
}

pub async fn register_handler() -> impl IntoResponse {
    "Register Handler"
}
