use serde_json::Value;
use std::collections::HashMap;

use crate::cache::common::*;
use crate::prelude::*;

#[derive(Default)]
pub struct HashMapCache {
    /// the standard ttl as number in seconds for every generated cache element
    ttl: u32,
    /// whether variables will be deleted automatically when they expire. If true the variable will be deleted. If false
    delete_on_expire: bool,
    store: HashMap<String, Value>,
}

impl Cache for HashMapCache {
    fn new(ttl: u32, delete_on_expire: bool) -> Self {
        HashMapCache {
            ttl,
            delete_on_expire,
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: &str, value: &Value) {
        self.store.insert(key.to_string(), value.clone());
    }

    fn set_multiple(&mut self, vec: Vec<KeyValue>) {
        for item in vec.iter() {
            self.set(item.key, &item.value);
        }
    }

    fn get(&mut self, key: &str) -> Result<Value, CacheError> {
        match self.store.get(key) {
            Some(v) => Ok(v.to_owned()),
            None => Err(CacheError::CacheKeyNotFound(key.into())),
        }
    }
}

#[cfg(test)]
mod hashmap_cache_tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_default_cache() {
        let cache = HashMapCache::default();

        assert_eq!(cache.ttl, 0);
        assert!(!cache.delete_on_expire)
    }

    #[test]
    fn test_new_cache() {
        let cache = HashMapCache::new(60, true);

        assert_eq!(cache.ttl, 60);
        assert!(cache.delete_on_expire);
    }

    #[test]
    fn test_cache_string_value() {
        let mut cache = HashMapCache::default();
        cache.set("string_value", &json!("hello world"));
        assert_eq!(
            cache.get("string_value").unwrap().as_str(),
            Some("hello world")
        );
    }

    #[test]
    fn test_cache_json_value() {
        let mut cache = HashMapCache::default();
        let json_value = json!({
            "name": "Alice",
            "age": 30,
            "city": "New York"
        });

        cache.set("json_value", &json_value);
        assert_eq!(cache.get("json_value").unwrap(), json_value.clone());
    }

    #[test]
    fn test_cache_get_not_found() {
        let mut cache = HashMapCache::default();
        assert_eq!(
            cache.get("key1"),
            Err(CacheError::CacheKeyNotFound("key1".into()))
        );
    }

    #[test]
    fn test_cache_set_multiple() {
        let mut cache = HashMapCache::default();
        let vec = vec![
            KeyValue {
                key: "key1",
                value: json!("value1"),
            },
            KeyValue {
                key: "key2",
                value: json!("value2"),
            },
        ];

        cache.set_multiple(vec);

        assert_eq!(cache.store.get("key1"), Some(&json!("value1")));
        assert_eq!(cache.store.get("key2"), Some(&json!("value2")));
    }
}
