use axum::{response::Redirect, routing::get, Router};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tera::Tera;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, limit::RequestBodyLimitLayer,
    normalize_path::NormalizePathLayer, services::ServeDir, timeout::TimeoutLayer,
};

mod controllers;

#[derive(Clone)]
pub struct AppState {
    tera: Tera,
}

#[tokio::main]
async fn main() -> Result<(), Redirect> {
    let max_request_size = env::var("MAX_REQUEST_SIZE")
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?
        .parse::<usize>()
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?;
    let timeout_duration = env::var("TIMEOUT_DURATION")
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?
        .parse::<u64>()
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?;

    let tera = Tera::new("src/templates/**/*").map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

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
        .route("/upload", get(controllers::upload::handler))
        .with_state(app_state)
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(timeout_duration)))
        .layer(RequestBodyLimitLayer::new(max_request_size));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?;
    axum::serve(listener, app).await.map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    Ok(())
}
