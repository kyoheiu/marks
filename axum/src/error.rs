use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Io(String),
    WalkDir(String),
    Json(String),
    FromUtf8(String),
    Git(String),
    Env,
    SystemTime,
    Query,
    Fd,
    Grep,
    SameName,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            Error::Io(s) => s,
            Error::WalkDir(s) => s,
            Error::Json(s) => s,
            Error::FromUtf8(s) => s,
            Error::Git(s) => s,
            Error::Env => "Cannot read env SHELL.",
            Error::SystemTime => "SystemTimeError.",
            Error::Query => "Query must be a pattern.",
            Error::Fd => "Cannot finish searching by ripgrep properly.",
            Error::Grep => "Cannot finish searching by fd properly.",
            Error::SameName => "A file with the same name exists.",
        };
        write!(f, "{}", printable)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::WalkDir(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(_err: std::time::SystemTimeError) -> Self {
        Error::SystemTime
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8(err.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(_err: std::env::VarError) -> Self {
        Error::Env
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::Git(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::Io(s) => s,
            Error::WalkDir(s) => s,
            Error::Json(s) => s,
            Error::FromUtf8(s) => s,
            Error::Git(s) => s,
            Error::Env => "Cannot read env SHELL.".to_string(),
            Error::SystemTime => "SystemTimeError.".to_string(),
            Error::Query => "Query must be a pattern.".to_string(),
            Error::Fd => "Cannot finish searching by fd properly.".to_string(),
            Error::Grep => "Cannot finish searching by ripgrep properly.".to_string(),
            Error::SameName => "A file with the same name exists.".to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
