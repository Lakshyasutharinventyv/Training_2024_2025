use crate::users::student::model::student_data_loading;




pub mod users;
pub mod utils;
pub mod configration;
use crate::utils::db_config::get_client;
use crate::utils::logger;

use  crate::utils::db_elasticsearch::{get_client as es_client, search_students,dynamic_query_builder,filter_query_builder};







#[tokio::main]
async fn main() {
  
//   let _ = get_client().await;
      // Initialize Logger
      logger::startLogger();

    //   log::info!("logger is started");
    //   log::error!("failde to connect elastic client");



     // Get the Elasticsearch client------------------------
     
     
    //  let client = match get_client().await {
    //     Ok(client) => client,
    //     Err(err) => {
    //         eprintln!("Failed to create Elasticsearch client: {}", err);
    //         return;
    //     }
    // };

    // // Perform the search query-----------------------------

    // match search_students().await {
    //     Ok(response) => {
    //         // Print the response body (for demonstration purposes)
    //         println!("{:#?}", response);
    //     }
    //     Err(err) => {
    //         eprintln!("Failed to perform search query: {}", err);
    //     }
    // }


    // // Dynamic Bool query builder.------------------------

    // (1)--------------------

    // let result = dynamic_query_builder("must","genre", "Action").await;
    // match result {
    //     Ok(response) => println!("Response: {}", response),
    //     Err(err) => println!("Error: {}", err),
    // }


//(2)------------------------

// let result = dynamic_query_builder(
//     Some(("length", "110")),   // must clause
//     // None,
//     Some(("genre", "Romance")),   // should clause
//     Some(("actors","X"))    // must_not clause
//     // None
// ).await;

//     match result {
//         Ok(response) => println!("Response: {}", response),
//         Err(err) => println!("Error: {}", err),
//     }
    



// filter -------------------------


// let result = filter_query_builder("director.keyword", "Director X").await;
// match result {
//     Ok(response) => println!("Response: {}", response),
//     Err(err) => println!("Error: {}", err),
// }




student_data_loading().await;

    println!("running on http://127.0.0.1:5000");
    let app = users::student::api();
    axum::Server::bind(&"127.0.0.1:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


