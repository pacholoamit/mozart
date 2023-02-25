#![allow(unused)]

use anyhow::Result;
use mozart::{Cache, HashMapCache};
use serde_json::json;

fn main() -> Result<()> {
    let mut cache = HashMapCache::default();

    let value = json!(1);

    cache.set("example", &value);

    let v = cache.get("example").unwrap();

    print!("{v}");

    Ok(())
}