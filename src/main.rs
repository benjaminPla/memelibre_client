use axum::{routing::get, Router};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tera::Tera;
use tower_http::{
    compression::CompressionLayer, normalize_path::NormalizePathLayer, services::ServeDir,
    timeout::TimeoutLayer,
};

mod controllers;

#[derive(Clone)]
pub struct AppState {
    tera: Tera,
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");

    let timeout_duration = env::var("TIMEOUT_DURATION")
        .expect("Missing TIMEOUT_DURATION env var")
        .parse::<u64>()
        .expect("Error parsing TIMEOUT_DURATION env var");

    let templates_dir = if std::path::Path::new("src/templates").exists() {
        "src/templates/**/*"
    } else {
        "templates/**/*"
    };
    let tera = Tera::new(templates_dir).expect("Error initializing Tera templates");

    let app_state = Arc::new(AppState { tera });

    let public_dir = if std::path::Path::new("src/public").exists() {
        "src/public"
    } else {
        "public"
    };

    let app = Router::new()
        .nest_service("/public", ServeDir::new(public_dir))
        .route("/", get(controllers::home::handler))
        .route("/meme/{id}", get(controllers::meme::handler))
        .route("/error", get(controllers::error::handler))
        .route("/not_found", get(controllers::not_found::handler))
        .route("/upload", get(controllers::upload::handler))
        .with_state(app_state)
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(timeout_duration)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Error binding to port 3001");

    axum::serve(listener, app)
        .await
        .expect("Error starting server");
}
