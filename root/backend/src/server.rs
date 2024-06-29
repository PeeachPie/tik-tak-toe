use axum::{http::StatusCode, routing::{get, post}, Json, Router};
// use serde::{Deserialize, Serialize};

#[tokio::main]
pub async fn start_server() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let api_route = Router::new()   
        .route("/hello_world", get(hello_world))
        .route("get_move", post(get_best_move));


    let app = Router::new()
        .fallback(fallback)
        .nest("/api", api_route);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

// basic handler that responds with a static string
async fn hello_world() -> &'static str {
    "Hello, World!\n"
}

async fn get_best_move(Json(payload): Json<Value>) -> Json<Value> {}

