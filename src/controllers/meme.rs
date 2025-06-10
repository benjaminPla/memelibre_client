use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
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
    let api_url = env::var("API_URL").map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    let client = Client::new();

    let response = client
        .get(format!("{}/meme/get/{}", api_url, id))
        .send()
        .await
        .map_err(|e| {
            eprintln!("{}:{} - {}", file!(), line!(), e);
            Redirect::to("/error")
        })?;

    if response.status() == StatusCode::NOT_FOUND {
        return Err(Redirect::to("/not_found"));
    };

    let meme: Meme = response.json().await.map_err(|e| {
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
