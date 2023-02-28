use crate::cache::HashMapCache;
use serde_json::Value;
use std::fmt::Debug;

pub struct Cachable<'a> {
    pub key: &'a str,
    pub value: Value,
}

pub trait CacheLike: Debug + Send + Sync + 'static {
    fn set(&mut self, key: &str, value: &Value);
    fn set_multiple(&mut self, vec: Vec<Cachable>);
    fn get(&self, key: &str) -> Option<Value>;
    fn get_multiple(&self, keys: Vec<&str>) -> Vec<Value>;
    fn delete(&mut self, key: &str);
    fn delete_multiple(&mut self, keys: Vec<&str>);
    fn keys(&self) -> Vec<String>;
    fn has(&self, key: &str) -> bool;
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
