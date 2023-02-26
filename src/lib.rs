#![allow(unused)]

mod cache;
use cache::HashMapCache;
pub use cache::{Cache, CacheTrait, KeyValue};

mod server;

mod error;
mod prelude;
mod utils;
