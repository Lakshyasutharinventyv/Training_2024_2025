use serde::{Deserialize, Serialize};
use tikv_client::RawClient;
use std::{fs, sync::{Arc, RwLock}};
use lazy_static::lazy_static;

use crate::utils::db_config::get_client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub s_id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Message<T> {
    pub status: u32,
    pub message_key: String,
    pub data: T,
}

lazy_static! {
    pub static ref ALL_ID_COUNT: Arc<RwLock<Vec<String>>> = {
        let all_id: Vec<String> = vec![
            "student_1".to_string(),
            "student_2".to_string(),
            "student_3".to_string(),
            "student_4".to_string(),
            "student_5".to_string(),
        ];
        Arc::new(RwLock::new(all_id))
    };
}


pub fn parse_json_data(filename: &str) -> Vec<Student> {
    let json_data = fs::read_to_string(filename)
        .expect("Failed to read the JSON file");
    serde_json::from_str(&json_data)
        .expect("Failed to parse JSON")
}


pub async fn store_data_in_tikv(students: Vec<Student>, client: &RawClient) {
    let mut counter = 1; 

    for student in students {
        let key = format!("student_{}", counter);
        let value = serde_json::to_string(&student).expect("Failed to serialize student");
        client.put(key, value).await.expect("Failed to store data in TiKV");

        counter += 1; 
    }
}

pub async fn student_data_loading() {
    let client = get_client().await.unwrap();
    let students = parse_json_data("./json_files/StudentData.json");
    store_data_in_tikv(students, &client).await;
}


// lazy_static! {

//     #[derive(Debug)]
//     pub static ref ALL_STUDENT: Arc<RwLock<Vec<Student>>> = {
//         let data = fs::read_to_string("./Axum_server_json/StudentData.json")
//             .expect("Failed to read the JSON file");

//         Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
//     };

// }
