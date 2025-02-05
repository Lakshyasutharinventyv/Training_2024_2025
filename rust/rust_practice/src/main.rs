async fn delayed_print() {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("Hello after 2 seconds!");


}

#[tokio::main] // Starts an async runtime
async fn main() {
    delayed_print().await;
    println!("Finished!");
}