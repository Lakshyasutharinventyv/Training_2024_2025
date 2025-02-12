use axum::{body::Body, http::StatusCode, middleware::Next, response::Response, http::Request};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::MySqlPool;
use std::sync::Arc;
use crate::models::Claims;
use std::env;

pub async fn auth(
    req: Request<Body>,
    next: Next,
    db: Arc<MySqlPool>,
) -> Response {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_header) = auth_header {
        if let Ok(auth_token) = auth_header.to_str() {
            
            // 🔹 Decode JWT Token
            let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "supersecret".to_string());
            let token_data = decode::<Claims>(
                auth_token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
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
                        Ok(Some((db_token,))) if db_token == auth_token => {
                            println!("Authenticated user: {}", username);
                            return next.run(req).await;
                        }
                        _ => println!("Invalid CSRF token for user: {}", username),
                    }
                }
                Err(_) => println!("Failed to decode JWT token"),
            }
        }
    }

    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized: Invalid CSRF token"))
        .unwrap()
}
