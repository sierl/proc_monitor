use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::thread;
use sysinfo::System;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    assert_eq!(sysinfo::IS_SUPPORTED_SYSTEM, true);

    let (tx, _) = broadcast::channel(1);

    let app_state = AppState { tx: tx.clone() };

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/script.js", get(get_script_js))
        .route("/style.css", get(get_style_css))
        .route("/realtime/cpus", get(get_realtime_cpu))
        .with_state(app_state.clone());

    // Compute CPU usage in background
    thread::spawn(move || {
        let mut sys = System::new();

        loop {
            sys.refresh_cpu();

            let v = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(v);

            thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        }
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
    tx: broadcast::Sender<Vec<f32>>,
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
async fn get_realtime_cpu(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async { realtime_cpu_stream(ws, state).await })
}

async fn realtime_cpu_stream(mut ws: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let payload = serde_json::to_string(&msg).unwrap();
        ws.send(Message::Text(payload)).await.unwrap();
    }
}
