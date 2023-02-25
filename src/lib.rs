#![allow(unused)]

mod cache;
pub use cache::HashMapCache;
pub use cache::{Cache, KeyValue};

mod server;

mod error;
mod prelude;
mod utils;
