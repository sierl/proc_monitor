use axum::extract::State;
use axum::{routing::get, Router};
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use sysinfo::System;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(root)).with_state(AppState {
        sys: Arc::new(Mutex::new(System::new())),
    });

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Listening on {}...", addr);

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn root(State(state): State<AppState>) -> String {
    assert_eq!(sysinfo::IS_SUPPORTED_SYSTEM, true);
    let mut res = String::new();

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        writeln!(&mut res, "CPU {}: {}%", i + 1, cpu.cpu_usage()).unwrap();
    }

    res
}
