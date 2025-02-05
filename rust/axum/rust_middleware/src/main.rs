
// Main function to set up Axum server
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user", post(create_user))  // Route to create user
        .layer(DeserializeUserLayer);       // Use custom middleware

    // Start the Axum server on 0.0.0.0:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
