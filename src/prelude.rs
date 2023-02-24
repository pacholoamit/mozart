pub use crate::error::{CacheError, Error};

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct W<T>(pub T);
