use std::{io, string::FromUtf8Error};
use thiserror::Error;
use reqwest::Error as ReqwestError;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),

    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),

    #[error("Invalid INI: {0}")]
    InvalidIni(#[from] ini::ini::Error),

    #[error("Enquote error: {0}")]
    Enquote(#[from] enquote::Error),

    #[error("{command} exited with status code {code}")]
    CommandFailed { command: String, code: i32 },

    #[error("Could not find config directory")]
    NoConfigDir,

    #[error("No {0} image found")]
    NoImage(&'static str),

    #[cfg(all(unix, not(target_os = "macos")))]
    #[error("No desktops found")]
    XfceNoDesktops,

    #[error("Unsupported Desktop")]
    UnsupportedDesktop,

    #[error("Invalid path")]
    InvalidPath,

    #[error("{0}")]
    Message(String),

    #[error("Network request failed: {0}")]
    Reqwest(#[from] ReqwestError),

}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Message(s.to_string())
    }
}