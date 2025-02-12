use std::env;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::MySqlPool;

use crate::{database::connection::DbPool, models::{Claims, UserLogin, UserRegister}};

pub async fn login_handler(
    State(db): State<MySqlPool>,
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

            (StatusCode::OK, Json(json!({ 
                "message": "User logged in successfully", 
                "user": username, 
                "email": email, 
                "csrf_token": csrf_token 
            })))
        },
        Err(err) => {
            println!("Error logging in: {:?}", err);
            (StatusCode::UNAUTHORIZED, Json(json!({ "message": "Failed to login" })))
        }
    }
}

pub async fn register_handler(
    State(db): State<DbPool>,
    Json(user): Json<UserRegister>,
) -> impl IntoResponse {
    println!("Received user: {:?}", user);

    let result = sqlx::query(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)"
    )
    .bind(&user.username) // 🔹 Correct order
    .bind(&user.email)
    .bind(&user.password) // 🔹 Include password binding
    .execute(&db) // 🔹 Use execute() instead of fetch_optional()
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