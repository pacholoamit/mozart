#![allow(unused)]

use anyhow::Result;
use cache::Cache;
use serde_json::json;

mod cache;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let mut cache = Cache::default();

    let value = json!(1);

    cache.set("example", &value);

    let v = cache.get("example").unwrap();

    print!("{v}");

    Ok(())
}
