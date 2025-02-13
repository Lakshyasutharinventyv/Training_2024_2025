// 5 threads
// 500 ms add user in vector
// 1 min print length of vector
// every 2min delete users which are old then 2 min
// 5 min print user vector
// every 100 ms check len of vector and if len > 100 break exit code 

// struct User{
//     id:u32,
//     username:String,
//     timestamp:String
// }

// vector(User1,User2,User3)


use chrono::{Local, Duration as ChronoDuration};
use core::panic;
use std::{
    sync::{Arc, RwLock}, 
    thread, 
    time::Duration
};

#[derive(Debug, Clone)]
struct User {
    id: u32,
    username: String,
    timestamp: i64,
}

fn main() {
    let users = Arc::new(RwLock::new(Vec::new()));

    let users_clone = Arc::clone(&users);
    let t1 = thread::spawn(move || {
        let mut id = 0;
        loop {
            let now = Local::now().timestamp();
            id += 1;
            let user = User {
                id,
                username: format!("user_{}", id),
                timestamp: now,
            };
            users_clone.write().unwrap().push(user);
            thread::sleep(Duration::from_millis(100));
        }
    });


    let users_clone = Arc::clone(&users);
    let t2 = thread::spawn(move || {
        loop {
            let length = users_clone.read().unwrap().len();
            println!("Length of user vector: {length}");
            thread::sleep(Duration::from_secs(2));
        }
    });


    let users_clone = Arc::clone(&users);
    let t3 = thread::spawn(move || {
        loop {
            let cutoff_time = Local::now() - ChronoDuration::seconds(10);
            let cutoff_timestamp = cutoff_time.timestamp();
            
            let mut users_lock = users_clone.write().unwrap();

            let mut removed_count = 0;
            users_lock.retain(|user| {
                if user.timestamp < cutoff_timestamp {
                    println!("Removing user: {:?}", user);
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });

            println!("Removed {removed_count} old users.");
            thread::sleep(Duration::from_secs(5));
        }
    });


    let users_clone = Arc::clone(&users);
    let t4 = thread::spawn(move || {
        
        println!("{:?}",users_clone.read().unwrap());
        thread::sleep(Duration::from_secs(15));
    });

    let users_clone = Arc::clone(&users);
    let t5 = thread::spawn(move || {
        loop {
            let length = users_clone.read().unwrap().len();
            if length > 100 {
                panic!("Exiting: User count exceeded 100.");
             
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();
}
