use crate::AppState;
use axum::{extract::State, response::Html};
use std::sync::Arc;
use tera::Context;

pub async fn handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let context = Context::new();

    let rendered = state
        .tera
        .render("error.html", &context)
        .unwrap_or_else(|e| {
            eprintln!("{}:{} - Tera render error: {}", file!(), line!(), e);
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <title>Meme Libre</title>
            </head>
            <body>
                <h1>Error</h1>
            </body>
            </html>
            "#
            .to_string()
        });

    Html(rendered)
}
