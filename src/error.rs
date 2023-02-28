use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("Passed Invalid address to server: {0}")]
    InvalidAddress(i32),
}

#[derive(Error, Debug, PartialEq)]
pub enum CacheError {
    #[error("Key `{0}` not found")]
    CacheKeyNotFound(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum StartupError {}
