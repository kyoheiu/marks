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
use std::fs::Metadata;
use std::path::Path;
use std::time::UNIX_EPOCH;
use tower_http::services::ServeDir;
use tracing::{error, info};
use walkdir::DirEntry;

#[derive(Clone)]
struct Core {}

impl Core {
    fn default() -> Result<Self, Error> {
        Ok(Core {})
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    desc: String,
    modified: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    original: String,
    new: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Res {
    result: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();
    info!("Initialized logger.");
    let core = Core::default()?;

    // build our application with a single route
    let app = Router::new()
        .route("/health", get(health))
        .route("/item", get(read_item).post(post_item).delete(remove_item))
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
        let file = std::fs::read_to_string(to_path_string(&file_name))?;
        Ok(file.into_response())
    } else {
        let mut result: Vec<Item> = Vec::new();
        for item in walkdir::WalkDir::new("./data") {
            let item = item?;
            if item.file_type().is_dir() {
                continue;
            }
            result.push(get_item(&item));
        }
        result.sort_by(|a, b| b.modified.partial_cmp(&a.modified).unwrap());
        Ok(Json(result).into_response())
    }
}

#[debug_handler]
async fn post_item(Json(payload): Json<Payload>) -> Result<(), Error> {
    if payload.original == payload.new {
        Ok(std::fs::write(
            to_path_string(&payload.new),
            &payload.content,
        )?)
    } else {
        if Path::new(&to_path_string(&payload.new)).exists() {
            error!("A file with the same name exists.");
            return Err(Error::SameName);
        }
        if !payload.original.is_empty() {
            std::fs::rename(
                to_path_string(&payload.original),
                to_path_string(&payload.new),
            )?;
        } else {
            std::fs::write(to_path_string(&payload.new), payload.content)?;
        }
        Ok(())
    }
}

#[debug_handler]
async fn remove_item(body: String) -> Result<(), Error> {
    Ok(std::fs::remove_file(to_path_string(body.trim()))?)
}

#[debug_handler]
async fn search(body: String) -> Result<impl IntoResponse, Error> {
    let mut rg_result = Vec::new();
    let q = body.split_whitespace().collect::<Vec<&str>>();
    info!("query: {:?}", q);
    if let Ok(output) = std::process::Command::new(&std::env::var("SHELL")?)
        .arg("-c")
        .args(["rg", "-l"])
        .args(q)
        .output()
    {
        let output = String::from_utf8(output.stdout)?;
        for item in output.lines() {
            if let Some(file_path) = item.lines().next() {
                if let Some(file_name) = file_path.split("/").last() {
                    rg_result.push(file_name.to_string());
                }
            }
        }

        let mut result = Vec::new();
        for file_name in rg_result {
            result.push(get_item_searched(&file_name));
        }

        result.sort_by(|a, b| b.modified.partial_cmp(&a.modified).unwrap());
        Ok(Json(result).into_response())
    } else {
        error!("ripgrep did not work successfully.");
        Err(Error::Grep)
    }
}

fn to_modified(metadata: &Metadata) -> u64 {
    metadata
        .modified()
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or(std::time::Duration::ZERO)
        .as_secs()
}

fn to_path_string(file_name: &str) -> String {
    format!("./data/{}", file_name)
}

fn get_item(item: &DirEntry) -> Item {
    let name = item
        .file_name()
        .to_str()
        .unwrap_or("UNKNWON FILE")
        .to_string();

    let path = item.path();
    let desc = match std::fs::read_to_string(path) {
        Ok(s) => s.chars().take(100).collect::<String>().trim().to_string(),
        Err(_) => "".to_string(),
    };

    // get modified time (UNIX time).
    let modified = match item.metadata() {
        Ok(m) => to_modified(&m),
        Err(_) => 0,
    };

    Item {
        name,
        desc,
        modified,
    }
}

fn get_item_searched(name: &str) -> Item {
    let desc = match std::fs::read_to_string(std::path::Path::new(&to_path_string(name))) {
        Ok(s) => s.chars().take(100).collect::<String>().trim().to_string(),
        Err(_) => "".to_string(),
    };

    // get modified time (UNIX time).
    let modified = match std::fs::metadata(&to_path_string(name)) {
        Ok(m) => to_modified(&m),
        Err(_) => 0,
    };

    Item {
        name: name.to_string(),
        desc,
        modified,
    }
}
