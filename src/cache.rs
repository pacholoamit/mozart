use crate::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Cache {
    /// the standard ttl as number in seconds for every generated cache element
    ttl: u32,
    /// whether variables will be deleted automatically when they expire. If true the variable will be deleted. If false
    delete_on_expire: bool,
    store: HashMap<String, String>,
}

pub struct KeyValue<'a, T: ?Sized = str> {
    key: &'a str,
    value: &'a T,
}

impl Cache {
    pub fn new(ttl: u32, delete_on_expire: bool) -> Self {
        Cache {
            ttl,
            delete_on_expire,
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store.insert(key.into(), value.into());
    }

    pub fn set_multiple(&mut self, vec: Vec<KeyValue>) {
        for item in vec.iter() {
            self.set(item.key, item.value)
        }
    }

    pub fn get(&mut self, key: &str) -> Result<String> {
        match self.store.get(key) {
            Some(v) => Ok(v.to_owned()),
            None => Err(Error::CacheKeyNotFound(key.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cache() {
        let cache = Cache::default();

        assert_eq!(cache.ttl, 0);
        assert!(!cache.delete_on_expire)
    }

    #[test]
    fn test_new_cache() {
        let cache = Cache::new(60, true);

        assert_eq!(cache.ttl, 60);
        assert!(cache.delete_on_expire);
    }

    #[test]
    fn test_set_value_in_cache() {
        let mut cache = Cache::default();

        cache.set("foo", "123");
        assert_eq!(cache.get("foo"), Ok("123".to_owned()));

        cache.set("foo", "456");
        assert_eq!(cache.get("foo"), Ok("456".to_owned()));
    }

    #[test]
    fn test_set_multiple() {
        let mut cache = Cache::default();

        let input = vec![
            KeyValue {
                key: "key1",
                value: "value1",
            },
            KeyValue {
                key: "key2",
                value: "value2",
            },
            KeyValue {
                key: "key3",
                value: "value3",
            },
        ];

        cache.set_multiple(input);

        assert_eq!(cache.get("key1"), Ok("value1".to_owned()));
        assert_eq!(cache.get("key2"), Ok("value2".to_owned()));
        assert_eq!(cache.get("key3"), Ok("value3".to_owned()));
    }

    #[test]
    fn test_get_value_from_cache() {
        let mut cache = Cache::new(60, false);
        cache.set("example", "123");

        assert_eq!(cache.get("example"), Ok("123".to_string()));

        assert_eq!(
            cache.get("bar"),
            Err(Error::CacheKeyNotFound(("bar").to_string()))
        );
    }
}
