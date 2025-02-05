use elasticsearch::{auth::Credentials, http::{transport::{SingleNodeConnectionPool, TransportBuilder}, Url}, Elasticsearch, SearchParts};
use serde_json::{json, Value};

pub async fn get_client()->Result<Elasticsearch,String>{
    let creds=Credentials::Basic("elastic".to_string(), "+pgx2n3wQ*pwn*2YbXR5".to_string());
    let conn_pool = SingleNodeConnectionPool::new(Url::parse("https://localhost:9200").unwrap());
    match TransportBuilder::new(conn_pool).auth(creds).build(){
        Ok(trans_layer)=>{
            let client = Elasticsearch::new(trans_layer);
            Ok(client)
        }
        Err(err)=>{
            Err(format!("Error in transport :{}",err.to_string()))
        }
    }
}




// searching query.


pub async fn search_students(index:&str) -> Result<String, String> {
    // Obtain the Elasticsearch client
    let client = match get_client().await {
        Ok(client) => client,
        Err(err) => return Err(format!("Failed to create Elasticsearch client: {}", err)),
    };

   
    let search_response = match client
        .search(SearchParts::Index(&[index]))
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .send()
        .await {
            Ok(response) => response,
            Err(err) => return Err(format!("Failed to send search request: {}", err)),
        };

    println!("Response body: {:#?}", search_response);

    // Extract the response body
    let response_body = match search_response.json::<Value>().await {
        Ok(body) => body,
        Err(err) => return Err(format!("Failed to parse response body: {}", err)),
    };

    // Extract hits from the response
    let hits = match response_body["hits"]["hits"].as_array() {
        Some(hits) => hits,
        None => return Err("No hits found".to_string()),
    };

    // Convert hits to JSON string
    let json_string = match serde_json::to_string(&hits) {
        Ok(string) => string,
        Err(err) => return Err(format!("Failed to serialize hits: {}", err)),
    };

    Ok(json_string)
}


// query builder (1)



// pub async fn dynamic_query_builder(bool_type:&str,field: &str, value: &str) -> Result<String, String> {
//     // Build the must query
//     let bool_query = json!({
//         "query": {
//             "bool": {
              
//               bool_type: [ 
//                 {
//                   "match": {
//                     field: value
//                   }
//                 }
//               ]
//             }
//           }
//     });

//     let client = match get_client().await {
//         Ok(client) => client,
//         Err(err) => return Err(format!("Failed to create Elasticsearch client: {}", err)),
//     };

//     // Define the search request
//     let search_response = match client
//         .search(SearchParts::Index(&["movies"]))
//         .body(bool_query)
//         .send()
//         .await {
//             Ok(response) => response,
//             Err(err) => return Err(format!("Failed to send search request: {}", err)),
//         };

//     // Extract the response body
//     let response_body = match search_response.json::<Value>().await {
//         Ok(body) => body,
//         Err(err) => return Err(format!("Failed to parse response body: {}", err)),
//     };

//     Ok(response_body.to_string())
// }



// querybuilder (2)

pub async fn dynamic_query_builder(
    must: Option<(&str, &str)>,
    should: Option<(&str, &str)>,
    must_not: Option<(&str, &str)>,
) -> Result<String, String> {
    let mut bool_query = json!({
        "query": {
            "bool": {}
        }
    });

    // Add must clause if provided
    if let Some((must_field, must_value)) = must {
        if let Some(must_array) = bool_query["query"]["bool"]["must"].as_array_mut() {
            must_array.push(json!({"match": {must_field: must_value}}));
        } else {
            bool_query["query"]["bool"]["must"] = json!([{"match": {must_field: must_value}}]);
        }
    }

    // Add should clause if provided
    if let Some((should_field, should_value)) = should {
        if let Some(should_array) = bool_query["query"]["bool"]["should"].as_array_mut() {
            should_array.push(json!({"match": {should_field: should_value}}));
            // should_array.push(json!({"match": {"rating": "5.2"}}));

        } else {
            bool_query["query"]["bool"]["should"] = json!([{"match": {should_field: should_value}}]);
            // bool_query["query"]["bool"]["should"] = json!([{"match": {should_field: should_value}}, {"match": {"rating": "5.2"}}]);
        }
    }

    // Add must_not caluse if provided

       if let Some((must_not_field, must_not_value)) = must_not {
        if let Some(must_not_array) = bool_query["query"]["bool"]["must_not"].as_array_mut() {
            must_not_array.push(json!({"match": {must_not_field: must_not_value}}));
        } else {
            bool_query["query"]["bool"]["must_not"] = json!([{"match": {must_not_field: must_not_value}}]);
        }
    }
   

    // print bool query
    println!("{:#?}", bool_query);
    println!("");

    let client = match get_client().await {
        Ok(client) => client,
        Err(err) => return Err(format!("Failed to create Elasticsearch client: {}", err)),
    };

  
    let search_response = match client
        .search(SearchParts::Index(&["movies"]))
        .body(bool_query)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(format!("Failed to send search request: {}", err)),
    };

    let response_body = match search_response.text().await {
        Ok(body) => body,
        Err(err) => return Err(format!("Failed to read response body: {}", err)),
    };

    Ok(response_body)
}




// dynamic filter query .


pub async fn filter_query_builder(field: &str, value: &str) -> Result<String, String> {
  
    let filter_query = json!({
        "query": {
            "bool": {
                "filter": [
                    {
                        "term": {
                            field: {
                                "value": value
                            }
                        }
                    }
                ]
            }
        }
    });

    let client = match get_client().await {
        Ok(client) => client,
        Err(err) => return Err(format!("Failed to create Elasticsearch client: {}", err)),
    };

    let search_response = match client
        .search(SearchParts::Index(&["movies"]))
        .body(filter_query)
        .send()
        .await {
            Ok(response) => response,
            Err(err) => return Err(format!("Failed to send search request: {}", err)),
        };

    
    let response_body = match search_response.text().await {
        Ok(body) => body,
        Err(err) => return Err(format!("Failed to read response body: {}", err)),
    };

    Ok(response_body)
}