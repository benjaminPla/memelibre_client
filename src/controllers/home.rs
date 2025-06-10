use crate::AppState;
use axum::{
    extract::State,
    response::{Html, Redirect},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tera::Context;

#[derive(Deserialize, Serialize, Debug)]
struct Meme {
    id: i32,
    image_url: String,
}

pub async fn handler(State(state): State<Arc<AppState>>) -> Result<Html<String>, Redirect> {
    let api_url = env::var("API_URL").map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    let client = Client::new();

    let memes: Vec<Meme> = client
        .get(format!("{}/meme/get", api_url))
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

    let mut memes_html = String::new();

    for meme in &memes {
        let mut context = Context::new();
        context.insert("meme", meme);

        match state.tera.render("_meme.html", &context) {
            Ok(rendered) => memes_html.push_str(&rendered),
            Err(e) => {
                eprintln!("Failed to render meme: {}", e);
                continue;
            }
        }
    }

    let mut context = Context::new();
    context.insert("memes", &memes_html);

    let rendered = state.tera.render("home.html", &context).map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    Ok(Html(rendered))
}
