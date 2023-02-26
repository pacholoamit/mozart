#![allow(unused)]

mod cache;
pub use cache::HashMapCache;
pub use cache::{CacheTrait, KeyValue, Cache};

mod server;

mod error;
mod prelude;
mod utils;
