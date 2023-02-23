use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("Key `{0}` does not exist in cache")]
    CacheKeyNotFound(String),
    // #[error("Key does not exist in cache")]
    // CacheKeyNotFound(),
}
