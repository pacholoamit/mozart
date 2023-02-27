use serde_json::Value;
use std::collections::HashMap;

use crate::cache::common::*;
use crate::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct HashMapCache {
    /// the standard ttl as number in seconds for every generated cache element
    ttl: u32,
    /// whether variables will be deleted automatically when they expire. If true the variable will be deleted. If false
    delete_on_expire: bool,
    store: HashMap<String, Value>,
}

impl HashMapCache {
    fn new(ttl: u32, delete_on_expire: bool) -> Self {
        HashMapCache {
            ttl,
            delete_on_expire,
            store: HashMap::new(),
        }
    }
}

impl CacheLike for HashMapCache {
    fn set(&mut self, key: &str, value: &Value) {
        self.store.insert(key.to_string(), value.clone());
    }

    fn set_multiple(&mut self, objects: Vec<Cachable>) {
        objects.iter().for_each(|obj| self.set(obj.key, &obj.value))
    }

    fn get(&self, key: &str) -> Option<Value> {
        self.store.get(key).to_owned().cloned()
    }

    fn get_multiple(&self, keys: Vec<&str>) -> Vec<Value> {
        keys.into_iter().filter_map(|key| self.get(key)).collect()
    }

    fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }

    fn delete_multiple(&mut self, keys: Vec<&str>) {
        keys.into_iter().for_each(|key| self.delete(key))
    }

    fn keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    fn has(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

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
        cache.set("foo", &json!("bar"));
        assert_eq!(cache.get("foo").unwrap().as_str(), Some("bar"));
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
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_cache_set_multiple() {
        let mut cache = HashMapCache::default();
        let vec = vec![
            Cachable {
                key: "key1",
                value: json!("value1"),
            },
            Cachable {
                key: "key2",
                value: json!("value2"),
            },
        ];

        cache.set_multiple(vec);

        assert_eq!(cache.store.get("key1"), Some(&json!("value1")));
        assert_eq!(cache.store.get("key2"), Some(&json!("value2")));
    }

    #[test]
    fn test_cache_get_multiple() {
        let mut cache = HashMapCache::default();
        let vec = vec![
            Cachable {
                key: "key1",
                value: json!("value1"),
            },
            Cachable {
                key: "key2",
                value: json!("value2"),
            },
        ];

        cache.set_multiple(vec);

        let values = cache.get_multiple(vec!["key1", "key2"]);
        assert_eq!(values, vec![json!("value1"), json!("value2")]);
    }

    #[test]
    fn test_cache_delete() {
        let mut cache = HashMapCache::default();

        cache.set("example", &json!("value"));
        cache.delete("example");
        assert_eq!(cache.get("example"), None)
    }

    #[test]
    fn test_cache_delete_multiple() {
        let mut cache = HashMapCache::default();

        cache.set("example1", &json!("value1"));
        cache.set("example2", &json!("value2"));
        cache.delete_multiple(vec!["example1", "example2"]);
        assert_eq!(cache.get("example1"), None);
        assert_eq!(cache.get("example2"), None);
    }

    #[test]
    fn test_cache_list_keys() {
        let mut cache = HashMapCache::default();

        cache.set("example1", &json!("value1"));
        assert_eq!(cache.keys(), vec!["example1"]);
    }

    #[test]
    fn test_cache_has() {
        let mut cache = HashMapCache::default();

        cache.set("example1", &json!("value1"));
        assert!(cache.has("example1"));
        assert!(!cache.has("example2"));
    }
}
