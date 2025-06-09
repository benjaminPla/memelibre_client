use crate::AppState;
use axum::{
    extract::State,
    response::{Html, Redirect},
};
use std::sync::Arc;
use tera::Context;

pub async fn handler(State(state): State<Arc<AppState>>) -> Result<Html<String>, Redirect> {
    let context = Context::new();

    let rendered = state.tera.render("not_found.html", &context).map_err(|e| {
        eprintln!("{}:{} - {}", file!(), line!(), e);
        Redirect::to("/error")
    })?;

    Ok(Html(rendered))
}
