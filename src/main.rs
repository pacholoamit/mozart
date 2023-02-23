#![allow(unused)]

use crate::prelude::*;
use cache::Cache;

mod cache;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let mut cache = Cache::new(0, false);

    cache.set("Example", "123").unwrap();

    let result = cache.get("Example").unwrap();

    println!("{}", result);

    Ok(())
}
