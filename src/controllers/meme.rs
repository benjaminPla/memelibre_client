use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::Context;

use crate::AppState;

#[derive(Deserialize, Serialize)]
struct Meme {
    id: i32,
    image_url: String,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Html<String>, Redirect> {
    let client = Client::new();

    let meme: Meme = client
        .get(format!("http://localhost:3000/meme/get/{}", id))
        .send()
        .await
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?
        .json()
        .await
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?;

    let mut context = Context::new();
    context.insert("meme", &meme);

    let rendered = state.tera.render("meme.html", &context).map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    Ok(Html(rendered))
}
