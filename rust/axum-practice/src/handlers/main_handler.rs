use std::env;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::MySqlPool;
use tower_cookies::{Cookie, Cookies};

use crate::models::{Claims, UserLogin, UserRegister};

pub async fn login_handler(
    State(db): State<MySqlPool>,
    cookies: Cookies,
    Json(user): Json<UserLogin>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, username, email FROM users WHERE username = ? AND password = ?"
    )
    .bind(&user.username)
    .bind(&user.password)
    .fetch_one(&db)
    .await;
    
    match result {
        Ok((user_id, username, email)) => {
            // 🔹 Generate CSRF Token with user_id and username
            let csrf_token = generate_token(user_id, &username).await;

            let update_result = sqlx::query(
                "UPDATE users SET token = ? WHERE id = ?"
            )
            .bind(&csrf_token)
            .bind(user_id)
            .execute(&db)
            .await;

            if let Err(err) = update_result {
                println!("Error updating CSRF token: {:?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Failed to update CSRF token" })));
            }

            // 🔹 Set CSRF Token inside HTTP-only Secure Cookie
            let mut cookie = Cookie::new("csrf_token", csrf_token.clone());
            cookie.set_path("/");
            cookie.set_secure(false);
            cookie.set_http_only(true);
            cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);

            cookies.add(cookie);
            
            // 🔹 Respond with success message
            (StatusCode::OK, Json(json!({ 
                "message": "User logged in successfully", 
                "user": username, 
                "email": email
            })))
        },
        Err(err) => {
            println!("Error logging in: {:?}", err);
            (StatusCode::UNAUTHORIZED, Json(json!({ "message": "Failed to login" })))
        }
    }
}
pub async fn register_handler(
    State(db): State<MySqlPool>,
    Json(user): Json<UserRegister>,
) -> impl IntoResponse {
    println!("Received user: {:?}", user);

    // 🔹 Ensure the `users` table exists
    let create_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(255) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            token TEXT NULL
        )"
    )
    .execute(&db)
    .await;

    if let Err(err) = create_table {
        println!("Error creating users table: {:?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Failed to initialize database" })));
    }

    // 🔹 Insert the new user
    let result = sqlx::query(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)"
    )
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password)
    .execute(&db)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "message": "User registered successfully" }))),
        Err(err) => {
            println!("Error saving user: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Failed to register user" })))
        }
    }
}



pub async fn generate_token(user_id: i32, username: &str) -> String {
    let claims = Claims {
        user_id,
        username: username.to_string(),
        exp: 10000000000,
    };

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "supersecret".to_string());

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .unwrap_or_else(|_| "invalid_token".to_string())
}