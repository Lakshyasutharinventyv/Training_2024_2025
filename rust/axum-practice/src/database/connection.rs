use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;

// Define an alias for better readability
pub type DbPool = MySqlPool;

pub async fn connect_to_tidb() -> DbPool {
    dotenv().ok(); // Load .env file

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        env::var("TIDB_USER").expect("TIDB_USER must be set"),
        env::var("TIDB_PASSWORD").expect("TIDB_PASSWORD must be set"),
        env::var("TIDB_HOST").expect("TIDB_HOST must be set"),
        env::var("TIDB_PORT").unwrap_or_else(|_| "4000".to_string()),
        env::var("TIDB_DATABASE").expect("TIDB_DATABASE must be set"),
    );
    println!("{:?}",database_url);
    match MySqlPool::connect(&database_url).await {
        Ok(pool) => {
            println!("✅ Connected to TiDB!");
            pool
        }
        Err(err) => {
            eprintln!("❌ TiDB Connection Failed: {:?}", err);
            std::process::exit(1);
        }
    }
}
