use axum::{extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, Json};
use serde_json::json;
use sqlx::MySqlPool;
use crate::models::Animal;
use sqlx::Row;




pub async fn get_animal(
    State(db): State<MySqlPool>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let id = match headers.get("X-Animal-ID") {
        Some(value) => match value.to_str() {
            Ok(id) => match id.parse::<i32>() {
                Ok(parsed_id) => parsed_id,
                Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Invalid ID format" }))),
            },
            Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Invalid header format" }))),
        },
        None => return (StatusCode::BAD_REQUEST, Json(json!({ "message": "X-Animal-ID header missing" }))),
    };

    println!("Extracted ID: {}", id);

    let result = sqlx::query(
        "SELECT id, name FROM animals WHERE id = ?
        "
    ).bind(id)
    .fetch_optional(&db)
    .await;

    match result {
        Ok(Some(row)) => {
            let animal = Animal {
                name: row.get("name"),
            };
            (StatusCode::OK, Json(json!({ "message": "Animal found", "data": animal })))
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "message": "No animal found with the given ID" }))),
        Err(err) => {
            println!("Error fetching animal: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Failed to fetch data" })))
        }
    }
    
}

#[axum::debug_handler]
pub async fn post_animals(
    State(db): State<MySqlPool>, // Use the correct type for the database pool
    Json(animal): Json<Animal>,
) -> impl IntoResponse {
    println!("Received animal: {:?}", animal);

    // Ensure the table exists before inserting
    let create_table_result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS animals (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(255) NOT NULL
        )"
    )
    .execute(&db)
    .await;

    if let Err(err) = create_table_result {
        println!("Error creating table: {:?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Failed to create table" })));
    }

    // Insert the animal into the database
    let result = sqlx::query(
        "INSERT INTO animals (name) VALUES (?)"
    )
    .bind(&animal.name)
    .execute(&db)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "message": "Animal saved successfully" }))),
        Err(err) => {
            println!("Error saving animal: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Failed to save animal" })))
        }
    }
}

pub async fn delete_animals(
    State(db): State<MySqlPool>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // ✅ Extract `X-Animal-ID` safely
    let id = match headers.get("X-Animal-ID") {
        Some(value) => match value.to_str() {
            Ok(id) => match id.parse::<i32>() {
                Ok(parsed_id) => parsed_id,
                Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Invalid ID format" }))),
            },
            Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Invalid header format" }))),
        },
        None => return (StatusCode::BAD_REQUEST, Json(json!({ "message": "X-Animal-ID header missing" }))),
    };

    println!("Deleting Animal with ID: {}", id);

    let result = sqlx::query("DELETE FROM animals WHERE id = ?")
        .bind(id)
        .execute(&db) // ✅ Use execute() for DELETE
        .await;

    match result {
        Ok(query_result) => {
            if query_result.rows_affected() > 0 { // ✅ Now using `rows_affected()`
                (StatusCode::OK, Json(json!({ "message": "Animal deleted successfully" })))
            } else {
                (StatusCode::NOT_FOUND, Json(json!({ "message": "No animal found with the given ID" })))
            }
        }
        Err(err) => {
            println!("Error deleting animal: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Failed to delete animal" })))
        }
    }
}





