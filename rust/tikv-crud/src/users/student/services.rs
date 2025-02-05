
use axum::{extract::Path,response::{IntoResponse, Response}, Json};


use crate::utils::db_config;

use super::model::{Message, Student,ALL_ID_COUNT};


pub async fn get_student_data() -> Response {

    let fkey_ekey = get_count();

   // "s".to_string(),"".to_string()    using this also as start key and end key this will give data of the studentid values.
    let data = match db_config::scan_data(fkey_ekey.0,fkey_ekey.1).await {
        Ok(data) => data,
        Err(_) => {
            
            let message = Message {
                status: 500,
                message_key: String::from("error"),
                data: vec!["Failed to retrieve student data from TiKV".to_string()],
            };
            return Json(message).into_response()
        }
    };


let mut deserialized_values: Vec<Student> = Vec::new();

for (_, value) in data {
    if let Ok(student_str) = String::from_utf8(value) {
        if let Ok(student) = serde_json::from_str::<Student>(&student_str) {
            deserialized_values.push(student);
        }
    }
}

    
    let message = Message {
        status: 200,
        message_key: String::from("success"),
        data: deserialized_values,
    };

   
    Json(message).into_response()
}






pub async fn get_student_id(Path(id): Path<i32>) -> Response {
    let student_key = format!("student_{}", id);

    match db_config::get_data(student_key).await {
        Ok(student_data) => {
            let student_vec: Student = match serde_json::from_str(&student_data) {
                Ok(student) => student,
                Err(err) => {
                    let message = Message {
                        status: 500,
                        message_key: err.to_string(),
                        data: String::from("Failed to deserialize student data"),
                    };
                   
                    return Json(message).into_response();
                }
            };

            let message = Message {
                status: 200,
                message_key: String::from("success"),
                data: student_vec,
            };
            Json(message).into_response()
        }
        Err(_) => {
            let message = Message {
                status: 404,
                message_key: String::from("error"),
                data: "No data found for the given student ID".to_string(),
            };
            Json(message).into_response()
        }
    }
}


pub async fn delete_student_id(Path(id): Path<i32>) -> Response {
  
    let deletion_result = db_config::delete_data(format!("student_{}", id)).await;

   
    if let Ok(_data) = deletion_result {
      
        let message = Message {
            status: 200,
            message_key: String::from("success"),
            data: "Student data deleted successfully".to_string(),
        };

        delete_id(id);
        Json(message).into_response()
    } else {
        
        let message = Message {
            status: 500, 
            message_key: String::from("error"),
            data: "Failed to delete student data".to_string(),
        };
        Json(message).into_response()
    }
}


pub async fn delete_all_student() -> Response {
    let fkey_ekey = get_count();
 
  
    
    let deletion_result = db_config::delete_data_range(fkey_ekey.0,fkey_ekey.1).await;

    if let Ok(_data) = deletion_result {
        let message = Message {
            status: 200,
            message_key: String::from("success"),
            data: "Student data deleted successfully".to_string(),
        };
        Json(message).into_response()
    } else {
        let message = Message {
            status: 500,
            message_key: String::from("error"),
            data: "Failed to delete student data".to_string(),
        };
        Json(message).into_response()
    }
}



pub async fn update_student_id(Path(id): Path<i32>, Json(new_student): Json<Student>) -> Response {
  
    if !check_id(id) {
        let message = Message {
            status: 404,
            message_key: String::from("error"),
            data: "No data found for the given student ID".to_string(),
        };
        return Json(message).into_response();
    }

    let got_student_data = serde_json::to_string(&new_student).expect("err");

    if let Ok(_data) = db_config::put_data(format!("student_{}", id), got_student_data).await {
        let message = Message {
            status: 200,
            message_key: String::from("successfully updated the data you can check it by getting the data through its id."),
            data: new_student,
        };
        Json(message).into_response()
    } else {
        let message = Message {
            status: 500, // Update the status code to indicate internal server error
            message_key: String::from("error"),
            data: "Error occurred while updating the data".to_string(),
        };
        Json(message).into_response()
    }
}



pub async fn create_student(Json(new_students): Json<Vec<Student>>) -> Response {
    let mut response_data: Vec<String> = Vec::new();

 
    
    for new_student in new_students.iter() {
        let  new_id = update_count();

        let got_student_data = serde_json::to_string(&new_student).expect("Failed to serialize new student data");

        
        match db_config::put_data(new_id.clone(), got_student_data).await {
            Ok(_) => {
                
                response_data.push(format!("Successfully created student with ID: {}", new_id));
            }
            Err(_) => {
              
                response_data.push(format!("Failed to create student with ID: {}", new_id));
            }
        }
    }

 
    let status_code = if response_data.iter().all(|msg| msg.contains("Successfully")) {
        200
    } else {
        400
    };

    let message = Message {
        status: status_code,
        message_key: String::from("success"),
        data: response_data,
    };

    Json(message).into_response()
}

fn update_count()->String{
   
    let mut  update_idcount = ALL_ID_COUNT.write().unwrap();

    let value = update_idcount.len() +1;
    let format = format!("student_{}",value);
    update_idcount.push(format.clone());
    format
}


fn get_count()->(String,String){
    let get_valuues = ALL_ID_COUNT.read().unwrap();
let fkey = get_valuues.first().unwrap().clone();
    let ekey = get_valuues.len() +1;
    let ekeyformat = format!("student_{}",ekey);
 
    (fkey,ekeyformat)

}

fn delete_id(id:i32){
let mut all_ids = ALL_ID_COUNT.write().unwrap(); 
let id_to_delete = format!("student_{}", id);
if let Some(index) = all_ids.iter().position(|item| item == &id_to_delete) {
    all_ids.remove(index);
}
}


fn check_id(id: i32) -> bool {
    let all_ids = ALL_ID_COUNT.read().unwrap(); // Lock the RwLock for read access
    let id_to_check = format!("student_{}", id);
    all_ids.iter().any(|item| item == &id_to_check)
}