use std::sync::mpsc;
use std::thread;



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
    id: u64,
    req_type: RequestType,
}

fn generate_request(id: u64) -> Request {
    let req_type = match rand::random::<u8>() % 5 {
        0 => RequestType::POST,
        1 => RequestType::PUT,
        2 => RequestType::GET,
        3 => RequestType::DELETE,
        _ => RequestType::PATCH,
    };

    Request { id, req_type }
}

fn main() {
    const NUM_REQUESTS: u64 = 1_000;

    let (post_tx, post_rx) = mpsc::channel();
    let (put_tx, put_rx) = mpsc::channel();
    let (get_tx, get_rx) = mpsc::channel();
    let (delete_tx, delete_rx) = mpsc::channel();
    let (patch_tx, patch_rx) = mpsc::channel();

    //1 Request sorting thread

    let sorter_thread = thread::spawn(move || {
        for id in 1..=NUM_REQUESTS {
            let request = generate_request(id);
            let result = match request.req_type {
                RequestType::POST => post_tx.send(request),
                RequestType::PUT => put_tx.send(request),
                RequestType::GET => get_tx.send(request),
                RequestType::DELETE => delete_tx.send(request),
                RequestType::PATCH => patch_tx.send(request),
            };
            
            if let Err(e) = result {
                eprintln!("Sorter failed to send request: {}", e);
            }
        }
        drop(post_tx);
        drop(put_tx);
        drop(get_tx);
        drop(delete_tx);
        drop(patch_tx);
    });

    // 5 worker threads

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
                println!("{} worker thread is processing {:?}", name, request);
            }
        })
    })
    .collect::<Vec<_>>();

    if let Err(e) = sorter_thread.join() {
        eprintln!("Sorter thread panicked: {:?}", e);
    }

    for worker in workers {
        if let Err(e) = worker.join() {
            eprintln!("Worker thread panicked: {:?}", e);
        }
    }
    

    println!("All requests processed successfully!");
}



