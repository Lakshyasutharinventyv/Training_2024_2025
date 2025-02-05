use axum::{routing::{get, post}, Router};

use self::services::{create_student, delete_all_student, delete_student_id, get_student_data, get_student_id, update_student_id};

pub mod model;
pub mod services;



pub fn api() -> Router {
    Router::new()
        .route("/get_all_sdata", get(get_student_data))
        .route("/get_student/:id",get(get_student_id))
        .route("/delete_student/:id",get(delete_student_id))  
        .route("/delete_allstudent",get(delete_all_student))          
        .route("/update_student/:id", post(update_student_id))
        .route("/create_student",post(create_student))  
}


// to do !!!!!!!!!!!!!!!!!!

// delete all students
// add multiple or single entity vector means one or more then one data