use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    Io(String),
    WalkDir(String),
    Env(String),
    Json(String),
    ParseInt(String),
    FromUtf8(String),
    SystemTime,
    Grep,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            Error::Io(s) => s,
            Error::WalkDir(s) => s,
            Error::Env(s) => s,
            Error::Json(s) => s,
            Error::ParseInt(s) => s,
            Error::FromUtf8(s) => s,
            Error::SystemTime => "SystemTimeError.",
            Error::Grep => "Cannot finish searching properly.",
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

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::Env(err.to_string())
    }
}

impl From<http::header::ToStrError> for Error {
    fn from(err: http::header::ToStrError) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err.to_string())
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

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::Io(s) => s,
            Error::WalkDir(s) => s,
            Error::Env(s) => s,
            Error::Json(s) => s,
            Error::ParseInt(s) => s,
            Error::FromUtf8(s) => s,
            Error::SystemTime => "SystemTimeError.".to_string(),
            Error::Grep => "Cannot finish searching properly.".to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
