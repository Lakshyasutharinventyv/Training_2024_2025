use axum::{
    routing::{post, get, delete}, Router, middleware
};
use std::sync::Arc;
use sqlx::MySqlPool;
use tower_cookies::CookieManagerLayer;

use crate::handlers::{
    main_handler::{register_handler, login_handler},
    features_handlers::{delete_animals, get_animal, post_animals}
};
use crate::middlewares::{logger::logger, auth::auth, add_id::add_id};

pub fn router(db: MySqlPool) -> Router {
    let db = Arc::new(db); // ✅ Use Arc for shared DB state

    Router::new()
    
    .route("/get_animals/{id}", get(get_animal))
    .route("/delete_animals/{id}", delete(delete_animals))
    .layer(middleware::from_fn(add_id)) // ✅ Add request ID middleware
    .route("/post_animals", post(post_animals))
    .layer(middleware::from_fn_with_state(db.clone(), auth)) // ✅ Ensure auth is after CookieManagerLayer
    .route("/login", post(login_handler))
    .route("/register", post(register_handler))
    .layer(CookieManagerLayer::new()) // ✅ Ensure cookies are handled
    .layer(middleware::from_fn(logger)) // ✅ Logger first
        .with_state(db.as_ref().clone()) // ✅ Correctly pass MySqlPool
}
