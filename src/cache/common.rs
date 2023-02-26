use crate::cache::HashMapCache;
use crate::prelude::*;
use serde_json::Value;

pub struct Cachable<'a> {
    pub key: &'a str,
    pub value: Value,
}

pub trait CacheStore {
    fn new(ttl: u32, delete_on_expire: bool) -> Self;
    fn set(&mut self, key: &str, value: &Value);
    fn set_multiple(&mut self, vec: Vec<Cachable>);
    fn get(&mut self, key: impl Into<String>) -> Option<Value>;
    fn get_multiple(&mut self, keys: Vec<impl Into<String>>) -> Vec<Value>;
    fn delete(&mut self, key: impl Into<String>);
    fn delete_multiple(&mut self, keys: Vec<impl Into<String>>);
    fn keys(&mut self) -> Vec<String>;
    fn has(&mut self, key: impl Into<String>) -> bool;
}

pub enum Cache {
    Empty,
    HashMap,
}

impl Cache {
    // TODO: Accept parameters here
    pub fn create(cache_type: Cache) -> impl CacheStore {
        match cache_type {
            Cache::HashMap => HashMapCache::new(0, false),
            Cache::Empty => HashMapCache::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let mut cache = Cache::create(Cache::HashMap);
        cache.set("test", &Value::String("test".to_string()));
        assert_eq!(cache.get("test"), Some(Value::String("test".to_string())));
    }
}
