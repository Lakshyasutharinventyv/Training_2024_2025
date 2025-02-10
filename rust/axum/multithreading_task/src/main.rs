use std::sync::mpsc;
use std::thread;
use reqwest;


#[derive(Debug)]
enum RequestType {
    POST,
    PUT,
    GET,
    DELETE,
    PATCH,
}

#[derive(Debug)]
struct Request {
    url: String,
    req_type: RequestType,
}

fn generate_random_request() -> Request {
    let endpoints = [
        (RequestType::GET, "https://jsonplaceholder.typicode.com/posts/1"),
        (RequestType::GET, "https://jsonplaceholder.typicode.com/posts/1/comments"),
        (RequestType::GET, "https://jsonplaceholder.typicode.com/comments?postId=1"),
        (RequestType::POST, "https://jsonplaceholder.typicode.com/posts"),
        (RequestType::PUT, "https://jsonplaceholder.typicode.com/posts/1"),
        (RequestType::PATCH, "https://jsonplaceholder.typicode.com/posts/1"),
        (RequestType::DELETE, "https://jsonplaceholder.typicode.com/posts/1"),
    ];
    
    let index = rand::random::<u8>() % 7;
    let (req_type, url) = &endpoints[index as usize];
    
    Request {
        url: url.to_string(),
        req_type: match req_type {
            RequestType::POST => RequestType::POST,
            RequestType::PUT => RequestType::PUT,
            RequestType::GET => RequestType::GET,
            RequestType::DELETE => RequestType::DELETE,
            RequestType::PATCH => RequestType::PATCH,
        },
    }
}

fn fetch_data(url: &str) -> String {
    match reqwest::blocking::get(url) {
        Ok(response) => match response.text() {
            Ok(text) => text,
            Err(_) => "Failed to read response body.".to_string(),
        },
        Err(_) => "Failed to fetch data.".to_string(),
    }
    
}

fn main() {
    const NUM_REQUESTS: u64 = 10;

    let (post_tx, post_rx) = mpsc::channel();
    let (put_tx, put_rx) = mpsc::channel();
    let (get_tx, get_rx) = mpsc::channel();
    let (delete_tx, delete_rx) = mpsc::channel();
    let (patch_tx, patch_rx) = mpsc::channel();

    // Sorting Thread
    let sorter_thread = thread::spawn(move || {
        for _ in 0..NUM_REQUESTS {
            let request = generate_random_request();
            let result = match request.req_type {
                RequestType::POST => post_tx.send(request),
                RequestType::PUT => put_tx.send(request),
                RequestType::GET => get_tx.send(request),
                RequestType::DELETE => delete_tx.send(request),
                RequestType::PATCH => patch_tx.send(request),
            };

            if result.is_err() {
                eprintln!("Sorter failed to send request.");
            }
        }

        drop(post_tx);
        drop(put_tx);
        drop(get_tx);
        drop(delete_tx);
        drop(patch_tx);
    });

    // 5 Worker Threads (Fetch Data from API)
    let workers = vec![
        ("POST", post_rx),
        ("PUT", put_rx),
        ("GET", get_rx),
        ("DELETE", delete_rx),
        ("PATCH", patch_rx),
    ]
    .into_iter()
    .map(|(name, rx)| {
        thread::spawn(move || {
            for request in rx {
                let response = fetch_data(&request.url);
                println!("👉 {} worker thread processed response for {}: {}",name, request.url, response);
                
            }
        })
    })
    .collect::<Vec<_>>();

    if sorter_thread.join().is_err() {
        eprintln!("Sorter thread panicked.");
    }

    for worker in workers {
        if worker.join().is_err() {
            eprintln!("Worker thread panicked.");
        }
    }

    println!("All requests processed!");
}
