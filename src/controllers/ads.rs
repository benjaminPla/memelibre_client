use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};

pub async fn handler() -> Response {
    let content = "google.com, pub-2367347449025847, DIRECT, f08c47fec0942fa0";
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("text/plain"));

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain")
        .body(content.into())
        .unwrap()
}
