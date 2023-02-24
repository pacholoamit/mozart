use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("Key `{0}` does not exist in cache")]
    CacheKeyNotFound(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum CacheError {
    #[error("Key `{0}` does not exist in cache")]
    CacheKeyNotFound(String),
}
