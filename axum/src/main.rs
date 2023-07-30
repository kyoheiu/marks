mod error;

use axum::debug_handler;
use axum::extract::{Json, Query};
use axum::response::{Html, IntoResponse};
use axum::{
    routing::{get, post},
    Router,
};
use error::Error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Clone)]
struct Core {}

impl Core {
    fn default() -> Result<Self, Error> {
        Ok(Core {})
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    name: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Res {
    result: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();
    info!("Initialized log.");
    let core = Core::default()?;

    // build our application with a single route
    let app = Router::new()
        .route("/health", get(health))
        .route("/item", get(read_item).post(save_item).delete(remove_item))
        .route("/api/search", post(search))
        .nest_service("/", ServeDir::new("static"))
        .with_state(core);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[debug_handler]
async fn health() -> Html<&'static str> {
    Html("Hello, developer.")
}

#[debug_handler]
async fn read_item(Query(q): Query<BTreeMap<String, String>>) -> Result<impl IntoResponse, Error> {
    if let Some(file_name) = q.get("item") {
        let file = std::fs::read_to_string(format!("./data/{}", file_name))?;
        Ok(file.into_response())
    } else {
        let mut result = Vec::new();
        for item in walkdir::WalkDir::new("./data") {
            result.push(item?.file_name().to_str().unwrap().to_string());
        }
        Ok(Json(result).into_response())
    }
}

#[debug_handler]
async fn save_item(Json(payload): Json<Payload>) -> Result<(), Error> {
    Ok(std::fs::write(
        format!("./data/{}", payload.name),
        &payload.body,
    )?)
}

#[debug_handler]
async fn remove_item(body: String) -> Result<(), Error> {
    Ok(std::fs::remove_file(format!("./data/{}",body.trim()))?)
}

#[debug_handler]
async fn search(body: String) -> Result<impl IntoResponse, Error> {
    let mut result = Vec::new();
    let q = body.split_whitespace().collect::<Vec<&str>>();
    info!("query: {:?}", q);
    info!("dir: {}", std::env::var("PWD").unwrap());
    if let Ok(output) = std::process::Command::new("rg")
        .arg("-l")
        .args(q)
        .arg("./data")
        .output()
    {
        let output = String::from_utf8(output.stdout)?;
        for item in output.lines() {
            info!("item: {}", item);
            if let Some(file_path) = item.lines().next() {
                if let Some(file_name) = file_path.split("/").last() {
                    result.push(file_name.to_string());
                }
            }
        }
        Ok(Json(Res { result }).into_response())
    } else {
        Err(Error::Grep)
    }
}
