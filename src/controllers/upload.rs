use crate::AppState;
use axum::{
    extract::State,
    response::{Html, Redirect},
};
use std::env;
use std::sync::Arc;
use tera::Context;

pub async fn handler(State(state): State<Arc<AppState>>) -> Result<Html<String>, Redirect> {
    let api_url = env::var("API_URL").map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    let endpoint = format!("{}/meme/post", api_url);

    let mut context = Context::new();
    context.insert("endpoint", &endpoint);

    let rendered = state.tera.render("upload.html", &context).map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    Ok(Html(rendered))
}
