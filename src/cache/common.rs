use crate::cache::HashMapCache;
use crate::prelude::*;
use serde_json::Value;

pub struct Cachable<'a> {
    pub key: &'a str,
    pub value: Value,
}

pub trait CacheLike {
    fn new(ttl: u32, delete_on_expire: bool) -> Self;
    fn set(&mut self, key: &str, value: &Value);
    fn set_multiple(&mut self, vec: Vec<Cachable>);
    fn get(&mut self, key: &str) -> Option<Value>;
    fn get_multiple(&mut self, keys: Vec<&str>) -> Vec<Value>;
    fn delete(&mut self, key: &str);
    fn delete_multiple(&mut self, keys: Vec<&str>);
    fn keys(&mut self) -> Vec<String>;
    fn has(&mut self, key: &str) -> bool;
}

pub enum CacheKind {
    Default,
    HashMap,
}

pub struct Cache;

impl Cache {
    pub fn create(kind: CacheKind) -> HashMapCache {
        match kind {
            CacheKind::Default => HashMapCache::default(),
            CacheKind::HashMap => HashMapCache::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let mut cache = Cache::create(CacheKind::Default);

        cache.set("test", &Value::String("test".to_string()));
        assert_eq!(cache.get("test"), Some(Value::String("test".to_string())));
    }
}
