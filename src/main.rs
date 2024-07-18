use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(root));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Listening on {}...", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Helllooo"
}
