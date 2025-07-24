// src/error.rs
use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse ini file: {0}")]
    Ini(#[from] ini::Error),

    #[error("enquote error: {0}")]
    Enquote(#[from] enquote::Error),

    #[error("string from UTF-8 error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Could not find a valid cache directory on your system")]
    NoCacheDir,

    #[error("Could not convert the temporary file path to a string")]
    InvalidFilePath,

    #[error("{0}")]
    Other(Box<dyn StdError + Send + Sync>),
}