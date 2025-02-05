
// use std::thread;


use lazy_static::lazy_static;

use tikv_client::{Key, RawClient, Value};
use tokio::sync::OnceCell;



lazy_static! {
    static ref CLIENT: OnceCell<RawClient> = OnceCell::new();

}


// get client fn
pub async fn get_client() -> Result<RawClient, String> {
    match CLIENT.get() {
        Some(_) => {
            if let Some(client) = CLIENT.get() {
                Ok(client.to_owned())
            } else {
                Err("Failed to retrieve client".to_string())
            }
        }
        None => {
            match RawClient::new(vec!["127.0.0.1:2379"]).await {
                Ok(client) => {
                    CLIENT.get_or_init(|| async { client }).await;
                    if let Some(client) = CLIENT.get() {
                        Ok(client.to_owned())
                    } else {
                        Err("Failed to retrieve client".to_string())
                    }
                }
                Err(err) => Err(err.to_string()),
            }
        }
    }
}
 


// scan data fn 
pub async fn scan_data(start_key: String, end_key: String) -> Result<Vec<(Key, Value)>, String> {
    match get_client().await {
        Ok(client) => {
            match client.scan(Key::from(start_key)..Key::from(end_key), 10).await {
                Ok(data) => {
                    let key_value_pairs: Vec<(Key, Value)> = data.into_iter().map(|kv_pair| (kv_pair.0,kv_pair.1)).collect();
                    Ok(key_value_pairs)
                }
                Err(err) => Err(format!("Error scanning data: {}", err)),
            }
        }
        Err(err) => Err(format!("Error getting client: {}", err)),
    }
}





// get data fn
pub async fn get_data(key: String) -> Result<String, String> {
    match get_client().await {
        Ok(client) => {
            match client.get(key).await {
                Ok(Some(value)) => {
                    let final_value = String::from_utf8(value).unwrap();
                    Ok(final_value)
                }
                Ok(None) => Err("Data not found".to_string()),
                Err(err) => Err(format!("Error: {}", err)),
            }
        }
        Err(err) => Err(format!("Error: {}", err)),
    }
}


// delete data fn

pub async fn delete_data(key: String) -> Result<String, String> {
    match get_client().await {
        Ok(client) => {
            match client.delete(key.clone()).await {
                Ok(_) => Ok(format!("Data deleted successfully for key: {} ",key)),
                Err(err) => Err(format!("Error in deleting data")),
            }
        }
        Err(_) =>  Err("Failed to get client".to_string()),
    }
}

// delete range fn

pub async fn delete_data_range(skey: String, ekey: String) -> Result<String, String> {
    match get_client().await {
        Ok(client) => {
            match client.delete_range(skey.clone()..=ekey.clone()).await {
                Ok(_) => Ok(format!("Data deleted successfully forom startkey: {} to endkey: {}", skey,ekey)),
                Err(err) => Err(format!("Error in deleting data")),
            }
        }
        Err(_) =>Err("Failed to get client".to_string()),
    }
}

// put data fn

pub async fn put_data(key: String, value: String) -> Result<String, String> {
    match get_client().await {
        Ok(client) => {
            match client.put(key.clone(), value).await {
                Ok(_) => Ok(format!("Data stored successfully for key: {}", key)),
                Err(err) => Err(format!("Error storing data: {}", err)),
            }
        }
        Err(_) => Err("Failed to get client".to_string()),
    }
}


// ----------------------------------------------------------------


 
// pub async fn get_client()-> Result<RawClient,String>{
 
  // loop {
    // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    // match RawClient::new(vec!["127.0.0.1:2379"]).await {
        // Ok(client) => return Ok(client),
        // Err(err) => {
            // eprintln!("Error: failed to connect to db {}. Retrying...", err);
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        // }
    // }
// }
// }
 
