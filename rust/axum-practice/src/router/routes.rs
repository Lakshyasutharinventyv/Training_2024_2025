use axum::{routing::post, Router, middleware};
use crate::handlers::main_handler::{login_handler, register_handler};
use crate::middlewares::{logger::logger, auth::auth, add_id::add_id};

use tower::ServiceBuilder;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(logger))
                .layer(middleware::from_fn(auth))
                .layer(middleware::from_fn(add_id))
        )
}
