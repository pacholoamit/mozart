#![allow(unused)]

use anyhow::Result;
use serde_json::json;

use crate::cache::{Cache, HashMapCache};

mod cache;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let mut cache = HashMapCache::default();

    let value = json!(1);

    cache.set("example", &value);

    let v = cache.get("example").unwrap();

    print!("{v}");

    Ok(())
}
