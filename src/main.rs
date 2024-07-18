use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use std::sync::{Arc, Mutex};
use sysinfo::System;

#[tokio::main]
async fn main() {
    assert_eq!(sysinfo::IS_SUPPORTED_SYSTEM, true);

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/script.js", get(get_script_js))
        .route("/style.css", get(get_style_css))
        .route("/api/cpus", get(get_cpus))
        .with_state(AppState {
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

#[axum::debug_handler]
async fn root() -> impl IntoResponse {
    let content = tokio::fs::read_to_string("view/index.html").await.unwrap();
    Html(content)
}

#[axum::debug_handler]
async fn get_style_css() -> impl IntoResponse {
    let content = tokio::fs::read_to_string("view/style.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(content)
        .unwrap()
}

#[axum::debug_handler]
async fn get_script_js() -> impl IntoResponse {
    let content = tokio::fs::read_to_string("view/script.js").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(content)
        .unwrap()
}

#[axum::debug_handler]
async fn get_cpus(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    let res: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(res)
}
