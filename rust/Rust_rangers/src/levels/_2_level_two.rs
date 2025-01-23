use std::time::Duration;
use tokio::time::sleep;


pub async fn level2() {
    println!("Loading..");
    sleep(Duration::from_millis(3000)).await;
    println!("game started.");
    sleep(Duration::from_millis(3000)).await;
    println!("\n\nTask 1: While Loop - Monitoring Water Level");
    sleep(Duration::from_millis(3000)).await;
    let mut water_level = 5;
    let max_safe_level = 10;
    while water_level <= max_safe_level {
        sleep(Duration::from_millis(500)).await;
        println!("Water level: {}", water_level);
        water_level += 1;
    }
    println!("Warning! Water level exceeded safe limits.");
    sleep(Duration::from_millis(3000)).await;
    println!("\n\nTask 2: Infinite Loop - Escaping the Whirlpool");
    let mut attempts = 0;
    loop {
        attempts += 1;
        sleep(Duration::from_millis(500)).await;
        println!("Attempt {} to escape the whirlpool!", attempts);

        if attempts == 3 {
            println!("Whirlpool escaped!");
            break;
        }
    }
    sleep(Duration::from_millis(3000)).await;
    println!("\n\nTask 3: Using Continue - Catching Big Fish");
    let fish_sizes = 10;
    println!("caught following large fishes!");
    for size in 0..fish_sizes {
        if size < 5 {
            continue;
        }
        print!("fish of size {}", size);
    }
    super::_3_level_three::level3().await;
}
