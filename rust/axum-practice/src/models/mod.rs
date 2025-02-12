use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug,Deserialize)]
pub struct UserLogin{
    pub username: String,
    pub password: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct  Animal{
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub username: String,
    pub exp: usize, 
}

