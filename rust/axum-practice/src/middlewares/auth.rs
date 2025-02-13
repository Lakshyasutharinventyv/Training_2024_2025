use axum::{
    body::Body,
    extract::{State, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::MySqlPool;
use tower_cookies::Cookies;
use std::{env, sync::Arc};
use crate::models::Claims;

pub async fn auth(
    State(db): State<Arc<MySqlPool>>, // ✅ Inject DB state properly
    cookies: Cookies,                 // ✅ Extract cookies automatically
    req: Request<Body>,
    next: Next,
) -> Response {
    println!("{:?}",cookies.get("csrf_token"));
    if let Some(cookie) = cookies.get("csrf_token") {
        let csrf_token = cookie.value().to_string();

        // 🔹 Decode JWT Token
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "supersecret".to_string());
        let token_data = decode::<Claims>(
            &csrf_token, 
            &DecodingKey::from_secret(secret.as_ref()), 
            &Validation::default()
        );

        match token_data {
            Ok(token) => {
                let user_id = token.claims.user_id;
                let username = token.claims.username;

                // 🔹 Check token validity in DB
                let result = sqlx::query_as::<_, (String,)>(
                    "SELECT token FROM users WHERE id = ? AND username = ?"
                )
                .bind(user_id)
                .bind(&username)
                .fetch_optional(&*db)
                .await;

                match result {
                    Ok(Some((db_token,))) if db_token == csrf_token => {
                        println!("Authenticated user: {}", username);
                        return next.run(req).await;
                    }
                    _ => println!("Invalid CSRF token for user: {}", username),
                }
            }
            Err(_) => println!("Failed to decode JWT token"),
        }
    } else {
        println!("CSRF token not found in cookies");
    }

    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized: Invalid or missing CSRF token"))
        .unwrap()
}
