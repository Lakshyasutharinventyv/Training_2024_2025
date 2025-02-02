use serde::{Deserialize, Serialize};
use axum::extract::Json;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    name: String,
    age: i8,
    gender: char,
}


pub async fn login(Json(data): Json<User>){
    println!("Received user data: {:?}", data);
}
