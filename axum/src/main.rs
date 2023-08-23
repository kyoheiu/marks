mod error;

use axum::body::StreamBody;
use axum::debug_handler;
use axum::extract::{Json, Query, State};
use axum::response::{Html, IntoResponse};
use axum::{
    routing::{get, post},
    Router,
};
use error::Error;
use git2::{IndexAddOption, Repository};
use http::{header, HeaderMap};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::Metadata;
use std::path::Path;
use std::time::UNIX_EPOCH;
use tokio_util::io::ReaderStream;
use tower_http::services::ServeDir;
use tracing::{error, info};
use walkdir::DirEntry;

const DATA_PATH: &str = "./data";

#[derive(Clone)]
struct Core {
    git_user: String,
    git_email: String,
}

impl Core {
    fn default() -> Result<Self, Error> {
        Ok(Core {
            git_user: std::env::var("MARKS_GIT_USER").unwrap_or("marks".to_string()),
            git_email: std::env::var("MARKS_GIT_EMAIL").unwrap_or("git@example.com".to_string()),
        })
    }

    fn commit(&self, file_name: &str, commit_message: &str) -> Result<(), Error> {
        let repo = Repository::open(DATA_PATH)?;
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let new_tree_oid = index.write_tree()?;
        let new_tree = repo.find_tree(new_tree_oid)?;
        let author = git2::Signature::now(&self.git_user, &self.git_email)?;
        let head = repo.head()?;
        let parent = repo.find_commit(
            head.target()
                .ok_or_else(|| Error::Git("Cannot get the OID.".to_string()))?,
        )?;
        repo.commit(
            Some("HEAD"),
            &author,
            &author,
            &format!("{}: {}", commit_message, file_name),
            &new_tree,
            &[&parent],
        )?;

        Ok(())
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
        .route("/item/search", post(search))
        .route("/item/download", get(download))
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
        let file = std::fs::read_to_string(to_path_string(file_name))?;
        Ok(file.into_response())
    } else {
        let mut result: Vec<Item> = Vec::new();
        let walker = walkdir::WalkDir::new(DATA_PATH).into_iter();
        for item in walker.filter_entry(|x| !is_hidden(x)) {
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
async fn post_item(State(core): State<Core>, Json(payload): Json<Payload>) -> Result<(), Error> {
    if payload.original == payload.new {
        std::fs::write(to_path_string(&payload.new), &payload.content)?;
        core.commit(&payload.new, "Update")?;
        Ok(info!("Updated item: {}", payload.new))
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
            std::fs::write(to_path_string(&payload.new), payload.content)?;
            core.commit(&payload.new, "Rename")?;
            Ok(info!("Renamed item: {}", payload.new))
        } else {
            std::fs::write(to_path_string(&payload.new), payload.content)?;
            core.commit(&payload.new, "Create")?;
            Ok(info!("Created item: {}", payload.new))
        }
    }
}

#[debug_handler]
async fn remove_item(State(core): State<Core>, body: String) -> Result<(), Error> {
    std::fs::remove_file(to_path_string(body.trim()))?;
    core.commit(body.trim(), "Remove")?;
    Ok(info!("Removed item: {}", body.trim()))
}

#[debug_handler]
async fn search(body: String) -> Result<impl IntoResponse, Error> {
    let q = body.split_whitespace().collect::<Vec<&str>>();
    if q.is_empty() {
        return Err(Error::Query);
    }
    //Currently single pattern is supported.
    let q = q[0];
    info!("query: {:?}", q);
    let mut search_result = BTreeSet::new();

    //exec fd
    if let Ok(output) = std::process::Command::new("fdfind")
        .arg(q)
        .arg(DATA_PATH)
        .output()
    {
        let output = String::from_utf8(output.stdout)?;
        info!(output);
        for file_path in output.lines() {
            if let Some(file_name) = file_path.split('/').last() {
                search_result.insert(file_name.to_string());
            }
        }
    } else {
        error!("fd did not work successfully.");
        return Err(Error::Fd);
    }

    //exec ripgrep
    if let Ok(output) = std::process::Command::new("rg")
        .arg("-l")
        .arg(q)
        .arg(DATA_PATH)
        .output()
    {
        let output = String::from_utf8(output.stdout)?;
        for item in output.lines() {
            if let Some(file_path) = item.lines().next() {
                if let Some(file_name) = file_path.split('/').last() {
                    search_result.insert(file_name.to_string());
                }
            }
        }
    } else {
        error!("ripgrep did not work successfully.");
        return Err(Error::Grep);
    }
    let mut result = Vec::new();
    for file_name in search_result {
        result.push(get_item_searched(&file_name));
    }

    result.sort_by(|a, b| b.modified.partial_cmp(&a.modified).unwrap());
    Ok(Json(result).into_response())
}

#[debug_handler]
async fn download(Query(q): Query<BTreeMap<String, String>>) -> Result<impl IntoResponse, Error> {
    if let Some(file_name) = q.get("item") {
        let file = tokio::fs::File::open(to_path_string(file_name)).await?;
        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);

        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "text/plain; charset=utf-8".parse()?);
        headers.insert(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_name).parse()?,
        );

        Ok((headers, body))
    } else {
        Err(Error::Io("Possibly file name does not exist.".to_string()))
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
    format!("{}/{}", DATA_PATH, file_name)
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
    let modified = match std::fs::metadata(to_path_string(name)) {
        Ok(m) => to_modified(&m),
        Err(_) => 0,
    };

    Item {
        name: name.to_string(),
        desc,
        modified,
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
