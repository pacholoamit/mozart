#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
